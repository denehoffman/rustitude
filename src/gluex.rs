use std::f64::consts::PI;

use nalgebra::Vector3;
use ndarray::{Array1, Array2, Array3, Axis};
use ndarray_linalg::Inverse;
use num_complex::Complex64;
use polars::error::PolarsError;
use sphrs::{ComplexSH, Coordinates, SHEval};

use crate::dataset::{extract_field, open_parquet, PolarsTypeConversion};
use crate::prelude::*;

/// Open a `GlueX` ROOT data file (flat tree) by `path`. `polarized` is a flag which is `true` if the
/// data file has polarization information included in the `"Px_Beam"` and `"Py_Beam"` branches.
///
/// # Panics
///
/// Panics if it can't find any of the required branches, or if they contain data with an
/// unexpected type.
///
/// # Errors
///
/// Will raise [`PolarsError`] in the event that any of the branches aren't read or converted
/// properly.
pub fn open_gluex(path: &str, polarized: bool) -> Result<Dataset, PolarsError> {
    let dataframe = open_parquet(path).expect("Read error");
    let col_n_fs = dataframe.column("NumFinalState").unwrap();
    let mut dataset = Dataset::new(col_n_fs.len());
    #[allow(clippy::cast_sign_loss)]
    let n_fs = col_n_fs.i32().unwrap().into_iter().next().unwrap().unwrap() as usize;
    let e_beam = extract_field("E_Beam", PolarsTypeConversion::F32ToScalar, &dataframe)?;
    let px_beam = extract_field("Px_Beam", PolarsTypeConversion::F32ToScalar, &dataframe)?;
    let py_beam = extract_field("Py_Beam", PolarsTypeConversion::F32ToScalar, &dataframe)?;
    let pz_beam = extract_field("Pz_Beam", PolarsTypeConversion::F32ToScalar, &dataframe)?;
    let mut beam_p4: Vec<FieldType> = Vec::new();
    let mut eps: Vec<FieldType> = Vec::new();
    if polarized {
        for (((e, px), py), _) in e_beam
            .into_iter()
            .zip(px_beam.into_iter())
            .zip(py_beam.into_iter())
            .zip(pz_beam.into_iter())
        {
            beam_p4.push(FieldType::Momentum(FourMomentum::new(
                e.clone().scalar().unwrap(),
                0.0,
                0.0,
                e.scalar().unwrap(),
            )));
            eps.push(FieldType::Vector(Array1::from_vec(vec![
                px.scalar().unwrap(),
                py.scalar().unwrap(),
                0.0,
            ])));
        }
        dataset.add_field("Beam P4", &beam_p4, false);
        dataset.add_field("EPS", &eps, false);
    } else {
        for (((e, px), py), pz) in e_beam
            .into_iter()
            .zip(px_beam.into_iter())
            .zip(py_beam.into_iter())
            .zip(pz_beam.into_iter())
        {
            beam_p4.push(FieldType::Momentum(FourMomentum::new(
                e.scalar().unwrap(),
                px.scalar().unwrap(),
                py.scalar().unwrap(),
                pz.scalar().unwrap(),
            )));
        }
        dataset.add_field("Beam P4", &beam_p4, false);
    }
    let weight = extract_field("Weight", PolarsTypeConversion::F32ToScalar, &dataframe)?;
    let e_finalstate = extract_field(
        "E_FinalState",
        PolarsTypeConversion::ListToVector,
        &dataframe,
    )?;
    let px_finalstate = extract_field(
        "Px_FinalState",
        PolarsTypeConversion::ListToVector,
        &dataframe,
    )?;
    let py_finalstate = extract_field(
        "Py_FinalState",
        PolarsTypeConversion::ListToVector,
        &dataframe,
    )?;
    let pz_finalstate = extract_field(
        "Pz_FinalState",
        PolarsTypeConversion::ListToVector,
        &dataframe,
    )?;
    dataset.add_field("Weight", &weight, false);
    let fs_p4: Vec<FieldType> = e_finalstate
        .iter()
        .zip(px_finalstate.iter())
        .zip(py_finalstate.iter())
        .zip(pz_finalstate.iter())
        .map(|(((e, px), py), pz)| {
            let mut momentum_vec = Vec::new();
            for i in 0..n_fs {
                momentum_vec.push(FourMomentum::new(
                    e.clone().vector().unwrap()[i],
                    px.clone().vector().unwrap()[i],
                    py.clone().vector().unwrap()[i],
                    pz.clone().vector().unwrap()[i],
                ));
            }
            FieldType::MomentumVec(momentum_vec)
        })
        .collect();
    dataset.add_field("Final State P4", &fs_p4, false);
    Ok(dataset)
}

#[derive(Clone)]
pub struct ParticleInfo {
    pub recoil_index: usize,
    pub daughter_index: usize,
    pub resonance_indices: Vec<usize>,
}

#[derive(Clone)]
pub struct Ylm {
    pub l: usize,
    pub m: isize,
    pub particle_info: ParticleInfo,
}

impl VariableBuilder for Ylm {
    fn into_variable(self) -> Variable {
        Variable::new(
            &format!("Y {} {}", self.l, self.m),
            move |entry: &VarMap| {
                let beam_p4_lab = entry["Beam P4"].momentum_ref().unwrap();
                let fs_p4s_lab = entry["Final State P4"].momenta_ref().unwrap();
                let fs_p4_lab = &fs_p4s_lab.iter().sum();
                let recoil_p4_lab = &fs_p4s_lab[self.particle_info.recoil_index];
                let resonance_p4_lab: &FourMomentum = &self
                    .particle_info
                    .resonance_indices
                    .iter()
                    .map(|i| &fs_p4s_lab[*i])
                    .sum();
                let daughter_p4_lab = &fs_p4s_lab[self.particle_info.daughter_index];

                let beam_p4 = &beam_p4_lab.boost_along(fs_p4_lab);
                let recoil_p4 = &recoil_p4_lab.boost_along(fs_p4_lab);
                let resonance_p4 = &resonance_p4_lab.boost_along(fs_p4_lab);
                let daughter_p4 = &daughter_p4_lab.boost_along(fs_p4_lab);

                let recoil_p4_res = &recoil_p4.boost_along(resonance_p4);
                let daughter_p4_res = &daughter_p4.boost_along(resonance_p4);

                let z = -1.0 * recoil_p4_res.momentum().normalize();
                let y = beam_p4
                    .momentum()
                    .cross(&(-1.0 * recoil_p4.momentum()))
                    .normalize();
                let x = y.cross(&z);

                let daughter_p3_res = daughter_p4_res.momentum();

                let p = Coordinates::cartesian(
                    daughter_p3_res.dot(&x),
                    daughter_p3_res.dot(&y),
                    daughter_p3_res.dot(&z),
                );
                #[allow(clippy::cast_possible_wrap)]
                FieldType::CScalar(ComplexSH::Spherical.eval(self.l as i64, self.m as i64, &p))
            },
            None,
        )
    }
}

impl<'a> AmplitudeBuilder<'a> for Ylm {
    fn into_amplitude(self) -> Amplitude<'a> {
        let ylm_var = self.into_variable();
        let var_name = ylm_var.name.clone();
        Amplitude::new(
            &var_name.clone(),
            move |_pars: &ParMap, vars: &VarMap| Ok(*(vars[&*var_name].cscalar_ref().unwrap())),
            None,
            Some(vec![ylm_var.clone()]),
        )
    }
}

pub enum Reflectivity {
    Positive,
    Negative,
}
pub struct Zlm {
    pub l: usize,
    pub m: isize,
    pub r: Reflectivity,
    pub particle_info: ParticleInfo,
}
impl VariableBuilder for Zlm {
    fn into_variable(self) -> Variable {
        Variable::new(
            &format!(
                "Z {} {} {}",
                self.l,
                self.m,
                match self.r {
                    Reflectivity::Positive => "+",
                    Reflectivity::Negative => "-",
                }
            ),
            move |entry: &VarMap| {
                let beam_p4_lab = entry["Beam P4"].momentum_ref().unwrap();
                let fs_p4s_lab = entry["Final State P4"].momenta_ref().unwrap();
                let fs_p4_lab = &fs_p4s_lab.iter().sum();
                let recoil_p4_lab = &fs_p4s_lab[self.particle_info.recoil_index];
                let resonance_p4_lab: &FourMomentum = &self
                    .particle_info
                    .resonance_indices
                    .iter()
                    .map(|i| &fs_p4s_lab[*i])
                    .sum();
                let daughter_p4_lab = &fs_p4s_lab[self.particle_info.daughter_index];

                let beam_p4 = &beam_p4_lab.boost_along(fs_p4_lab);
                let recoil_p4 = &recoil_p4_lab.boost_along(fs_p4_lab);
                let resonance_p4 = &resonance_p4_lab.boost_along(fs_p4_lab);
                let daughter_p4 = &daughter_p4_lab.boost_along(fs_p4_lab);

                let recoil_p4_res = &recoil_p4.boost_along(resonance_p4);
                let daughter_p4_res = &daughter_p4.boost_along(resonance_p4);

                let z = -1.0 * recoil_p4_res.momentum().normalize();
                let y = beam_p4
                    .momentum()
                    .cross(&(-1.0 * recoil_p4.momentum()))
                    .normalize();
                let x = y.cross(&z);

                let daughter_p3_res = daughter_p4_res.momentum();

                let p = Coordinates::cartesian(
                    daughter_p3_res.dot(&x),
                    daughter_p3_res.dot(&y),
                    daughter_p3_res.dot(&z),
                );
                #[allow(clippy::cast_possible_wrap)]
                let ylm = ComplexSH::Spherical.eval(self.l as i64, self.m as i64, &p);

                // Polarization
                let eps = Vector3::from_vec(entry["EPS"].vector_ref().unwrap().to_vec());
                let big_phi = y
                    .dot(&eps)
                    .atan2(beam_p4.momentum().normalize().dot(&eps.cross(&y)));
                let phase = Complex64::cis(-big_phi);
                let pgamma = eps.norm();

                let zlm = ylm * phase;

                let res = match self.r {
                    Reflectivity::Positive => Complex64 {
                        re: (1.0 + pgamma).sqrt() * zlm.re,
                        im: (1.0 - pgamma).sqrt() * zlm.im,
                    },
                    Reflectivity::Negative => Complex64 {
                        re: (1.0 - pgamma).sqrt() * zlm.re,
                        im: (1.0 + pgamma).sqrt() * zlm.im,
                    },
                };
                FieldType::CScalar(res)
            },
            None,
        )
    }
}
impl<'a> AmplitudeBuilder<'a> for Zlm {
    fn into_amplitude(self) -> Amplitude<'a> {
        let zlm_var = self.into_variable();
        let var_name = zlm_var.name.clone();
        Amplitude::new(
            &var_name.clone(),
            move |_pars: &ParMap, vars: &VarMap| Ok(*(vars[&*var_name].cscalar_ref().unwrap())),
            None,
            Some(vec![zlm_var.clone()]),
        )
    }
}

#[derive(Clone)]
pub struct Mass {
    name: String,
    particle_info: ParticleInfo,
}
impl VariableBuilder for Mass {
    fn into_variable(self) -> Variable {
        Variable::new(
            &self.name,
            move |vars: &VarMap| {
                let fs_p4s_lab = vars["Final State P4"].momenta_ref().unwrap();
                let resonance_p4_lab: &FourMomentum = &self
                    .particle_info
                    .resonance_indices
                    .iter()
                    .map(|i| &fs_p4s_lab[*i])
                    .sum();
                FieldType::Scalar(resonance_p4_lab.m())
            },
            None,
        )
    }
}

#[derive(Clone)]
pub struct KMatrixConstants {
    pub g: Array2<f64>,
    pub m: Array1<f64>,
    pub c: Array2<f64>,
    pub m1: Array1<f64>,
    pub m2: Array1<f64>,
}

#[derive(Clone)]
pub struct BarrierFactor {
    name: String,
    m: Array1<f64>,
    m1: Array1<f64>,
    m2: Array1<f64>,
    l: usize,
    n_resonances: usize,
    n_channels: usize,
    mass: Variable,
}

impl BarrierFactor {
    fn new(
        name: String,
        constants: KMatrixConstants,
        l: usize,
        particle_info: &ParticleInfo,
    ) -> BarrierFactor {
        let n_resonances = constants.g.ncols();
        let n_channels = constants.g.nrows();
        let mass_name = format!("{}_mass", name.clone());
        let mass = Mass {
            name: mass_name.clone(),
            particle_info: particle_info.clone(),
        }
        .into_variable();
        BarrierFactor {
            name,
            m: constants.m,
            m1: constants.m1,
            m2: constants.m2,
            l,
            n_resonances,
            n_channels,
            mass,
        }
    }
}

impl BarrierFactor {
    fn chi_plus(&self, s: f64, channel: usize) -> Complex64 {
        (1.0 - ((&self.m1 + &self.m2) * (&self.m1 + &self.m2))[channel] / s).into()
    }

    fn chi_minus(&self, s: f64, channel: usize) -> Complex64 {
        (1.0 - ((&self.m1 - &self.m2) * (&self.m1 - &self.m2))[channel] / s).into()
    }

    fn rho(&self, s: f64, channel: usize) -> Complex64 {
        (self.chi_plus(s, channel) * self.chi_minus(s, channel)).sqrt()
    }
    fn z(&self, s: f64, channel: usize) -> Complex64 {
        let q = self.rho(s, channel) * Complex64::sqrt(s.into()) / 2.0;
        q * q / (0.1973 * 0.1973)
    }
    fn blatt_weisskopf(&self, s: f64, channel: usize) -> Complex64 {
        let z = self.z(s, channel);
        match self.l {
            0 => 1.0.into(),
            1 => ((2.0 * z) / (z + 1.0)).sqrt(),
            2 => ((13.0 * z.powi(2)) / ((z - 3.0).powi(2) + 9.0 * z)).sqrt(),
            3 => ((277.0 * z.powi(3)) / (z * (z - 15.0).powi(2) + 9.0 * (2.0 * z - 5.0).powi(2)))
                .sqrt(),
            4 => ((12746.0 * z.powi(4)) / (z.powi(2) - 45.0 * z + 105.0).powi(2)
                + 25.0 * z * (2.0 * z - 21.0).powi(2))
            .sqrt(),
            l => panic!("L = {l} is not yet implemented"),
        }
    }
    fn barrier_factor(&self, s: f64, channel: usize, resonance: usize) -> Complex64 {
        let numerator = self.blatt_weisskopf(s, channel);
        let denominator = self.blatt_weisskopf(self.m[resonance].powi(2), channel);
        numerator / denominator
    }
}
impl VariableBuilder for BarrierFactor {
    fn into_variable(self) -> Variable {
        let mass = self.mass.clone();
        let mass_name = mass.name.clone();
        Variable::new(
            &self.name.clone(),
            move |entry: &VarMap| {
                let s = entry[&*mass_name].scalar_ref().unwrap().powi(2);
                FieldType::CMatrix(Array2::from_shape_fn(
                    (self.n_channels, self.n_resonances),
                    |(i, a)| self.barrier_factor(s, i, a),
                ))
            },
            Some(vec![mass]),
        )
    }
}

#[derive(Clone)]
pub struct AdlerZero {
    pub s_0: f64,
    pub s_norm: f64,
}

#[derive(Clone)]
pub struct FrozenKMatrix {
    name: String,
    selected_channel: usize,
    g: Array2<f64>,
    m: Array1<f64>,
    c: Array2<f64>,
    m1: Array1<f64>,
    m2: Array1<f64>,
    l: usize,
    n_resonances: usize,
    n_channels: usize,
    particle_info: ParticleInfo,
    adler_zero: Option<AdlerZero>,
}

impl FrozenKMatrix {
    pub fn new(
        name: &str,
        selected_channel: usize,
        constants: KMatrixConstants,
        l: usize,
        particle_info: ParticleInfo,
        adler_zero: Option<AdlerZero>,
    ) -> FrozenKMatrix {
        let n_resonances = constants.g.ncols();
        let n_channels = constants.g.nrows();
        FrozenKMatrix {
            name: name.into(),
            selected_channel,
            g: constants.g,
            m: constants.m,
            c: constants.c,
            m1: constants.m1,
            m2: constants.m2,
            l,
            n_resonances,
            n_channels,
            particle_info,
            adler_zero,
        }
    }

    fn chi_plus(&self, s: f64, channel: usize) -> Complex64 {
        (1.0 - ((&self.m1 + &self.m2) * (&self.m1 + &self.m2))[channel] / s).into()
    }

    fn chi_minus(&self, s: f64, channel: usize) -> Complex64 {
        (1.0 - ((&self.m1 - &self.m2) * (&self.m1 - &self.m2))[channel] / s).into()
    }

    fn rho(&self, s: f64, channel: usize) -> Complex64 {
        (self.chi_plus(s, channel) * self.chi_minus(s, channel)).sqrt()
    }

    fn c_matrix(&self, s: f64) -> Array2<Complex64> {
        Array2::from_diag(&Array1::from_shape_fn(self.n_channels, |channel| {
            self.rho(s, channel) / PI
                * ((self.chi_plus(s, channel) + self.rho(s, channel))
                    / (self.chi_plus(s, channel) - self.rho(s, channel)))
                .ln()
                + self.chi_plus(s, channel) / PI
                    * ((&self.m2 - &self.m1) / (&self.m1 + &self.m2))[channel]
                    * Complex64::from((&self.m2 / &self.m1)[channel]).ln()
        }))
    }
}

impl VariableBuilder for FrozenKMatrix {
    fn into_variable(self) -> Variable {
        let mass_name = format!("{}_mass", self.name.clone());
        let mass = Mass {
            name: mass_name.clone(),
            particle_info: self.particle_info.clone(),
        }
        .into_variable();
        let bf_name = format!("{}_bf", self.name.clone());
        let barrier_factor = BarrierFactor {
            name: bf_name.clone(),
            m: self.m.clone(),
            m1: self.m1.clone(),
            m2: self.m2.clone(),
            l: self.l,
            n_resonances: self.n_resonances,
            n_channels: self.n_channels,
            mass: mass.clone(),
        }
        .into_variable();

        Variable::new(
            &self.name.clone(),
            move |entry: &VarMap| {
                let s = entry[&mass_name].scalar_ref().unwrap().powi(2);
                let bf = entry[&bf_name].cmatrix_ref().unwrap();
                let k_ija = Array3::from_shape_fn(
                    (self.n_channels, self.n_channels, self.n_resonances),
                    |(i, j, a)| {
                        bf[[i, a]]
                            * ((self.g[[i, a]] * self.g[[j, a]]) / (self.m[a].powi(2) - s)
                                + self.c[[i, j]])
                            * bf[[j, a]]
                    },
                );
                let k_mat = match self.adler_zero {
                    Some(AdlerZero { s_0, s_norm }) => {
                        Complex64::from((s - s_0) / s_norm) * k_ija.sum_axis(Axis(2))
                    }
                    None => k_ija.sum_axis(Axis(2)),
                };
                let c_mat = self.c_matrix(s);
                let i_mat: Array2<Complex64> = Array2::eye(self.n_channels);
                let ikc_mat = i_mat + k_mat * c_mat;
                let ikc_mat_inv = ikc_mat.inv().unwrap();
                FieldType::CVector(ikc_mat_inv.row(self.selected_channel).to_owned())
            },
            Some(vec![mass, barrier_factor]),
        )
    }
}

impl<'a> AmplitudeBuilder<'a> for FrozenKMatrix {
    fn into_amplitude(self) -> Amplitude<'a> {
        let ikc_inv_var = self.clone().into_variable();
        let mass_name = ikc_inv_var.dependencies.clone().unwrap()[0].name.clone();
        let bf_name = ikc_inv_var.dependencies.clone().unwrap()[1].name.clone();
        let var_name = ikc_inv_var.name.clone();
        let internal_parameters: Vec<String> = (0..self.n_resonances)
            .map(|i| format!("beta_{i}"))
            .collect();
        let internal_parameters: Vec<&str> = internal_parameters.iter().map(|s| &**s).collect();
        Amplitude::new(
            &var_name.clone(),
            move |pars: &ParMap, vars: &VarMap| {
                let s = vars[&*mass_name].scalar_ref().unwrap().powi(2);
                let bf = vars[&*bf_name].cmatrix_ref().unwrap();
                let ikc_inv_vec = vars[&*var_name].cvector_ref().unwrap();
                let betas = Array1::from_shape_fn(self.n_resonances, |i| {
                    pars[&format!("beta_{i}")].value.cscalar().unwrap()
                });

                let p_ja = Array2::from_shape_fn((self.n_channels, self.n_resonances), |(j, a)| {
                    ((betas[a] * self.g[[j, a]]) / (self.m[a].powi(2) - s)) * bf[[j, a]]
                });
                let p_vec = p_ja.sum_axis(Axis(1));

                Ok(ikc_inv_vec.dot(&p_vec))
            },
            Some(internal_parameters),
            Some(vec![ikc_inv_var.clone()]),
        )
    }
}
