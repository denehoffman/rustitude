use crate::utils::blatt_weisskopf;
use crate::utils::breakup_momentum;
use std::f64::consts::PI;

use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use rustitude_core::prelude::*;

#[derive(Default)]
pub struct BreitWigner {
    p1_indices: Vec<usize>,
    p2_indices: Vec<usize>,
    l: usize,
    m: Vec<f64>,
    m1: Vec<f64>,
    m2: Vec<f64>,
    q: Vec<f64>,
    f: Vec<f64>,
}
impl BreitWigner {
    pub fn new(p1_indices: &[usize], p2_indices: &[usize], l: usize) -> Self {
        Self {
            p1_indices: p1_indices.into(),
            p2_indices: p2_indices.into(),
            l,
            ..Default::default()
        }
    }
}
impl Node for BreitWigner {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        (self.m, (self.m1, (self.m2, (self.q, self.f)))) = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let p1: FourMomentum = self
                    .p1_indices
                    .iter()
                    .map(|i| &event.daughter_p4s[*i])
                    .sum();
                let p2: FourMomentum = self
                    .p2_indices
                    .iter()
                    .map(|i| &event.daughter_p4s[*i])
                    .sum();
                let m = (p1 + p2).m();
                let m1 = p1.m();
                let m2 = p2.m();
                let q = breakup_momentum(m, m1, m2);
                let f = blatt_weisskopf(m, m1, m2, self.l);
                (m, (m1, (m2, (q, f))))
            })
            .unzip();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let m = self.m[event.index];
        let m1 = self.m1[event.index];
        let m2 = self.m2[event.index];
        let q = self.q[event.index];
        let f = self.f[event.index];
        let m0 = parameters[0];
        let g0 = parameters[1];
        let f0 = blatt_weisskopf(m0, m1, m2, self.l);
        let q0 = breakup_momentum(m0, m1, m2);
        let g = g0 * (m0 / m) * (q / q0) * (f.powi(2) / f0.powi(2));
        Ok(f * (m0 * g0 / PI) / Complex64::new(m0.powi(2) - m.powi(2), -1.0 * m0 * g))
    }

    fn parameters(&self) -> Vec<String> {
        vec!["mass".to_string(), "width".to_string()]
    }
}

#[derive(Clone, Copy)]
pub struct AdlerZero {
    pub s_0: f64,
    pub s_norm: f64,
}
struct KMatrixConstants {
    n_resonances: usize,
    n_channels: usize,
    g: DMatrix<f64>,
    c: DMatrix<f64>,
    m1s: Vec<f64>,
    m2s: Vec<f64>,
    mrs: Vec<f64>,
    adler_zero: Option<AdlerZero>,
    l: usize,
}

impl KMatrixConstants {
    fn chi_plus(s: f64, m1: f64, m2: f64) -> f64 {
        1.0 - ((m1 + m2) * (m1 + m2)) / s
    }

    fn chi_minus(s: f64, m1: f64, m2: f64) -> f64 {
        1.0 - ((m1 - m2) * (m1 - m2)) / s
    }

    fn rho(s: f64, m1: f64, m2: f64) -> Complex64 {
        Complex64::from(Self::chi_plus(s, m1, m2) * Self::chi_minus(s, m1, m2)).sqrt()
    }
    fn c_matrix(&self, s: f64) -> DMatrix<Complex64> {
        DMatrix::from_diagonal(&DVector::from_fn(self.n_channels, |i, _| {
            Self::rho(s, self.m1s[i], self.m2s[i]) / PI
                * ((Self::chi_plus(s, self.m1s[i], self.m2s[i])
                    + Self::rho(s, self.m1s[i], self.m2s[i]))
                    / (Self::chi_plus(s, self.m1s[i], self.m2s[i])
                        - Self::rho(s, self.m1s[i], self.m2s[i])))
                .ln()
                + Self::chi_plus(s, self.m1s[i], self.m2s[i]) / PI
                    * ((self.m2s[i] - self.m1s[i]) / (self.m1s[i] + self.m2s[i]))
                    * (self.m2s[i] / self.m1s[i]).ln()
        }))
    }
    fn barrier_factor(s: f64, m1: f64, m2: f64, mr: f64, l: usize) -> f64 {
        blatt_weisskopf(s.sqrt(), m1, m2, l) / blatt_weisskopf(mr, m1, m2, l)
    }
    fn barrier_matrix(&self, s: f64) -> DMatrix<f64> {
        DMatrix::from_fn(self.n_channels, self.n_resonances, |i, a| {
            Self::barrier_factor(s, self.m1s[i], self.m2s[i], self.mrs[a], self.l)
        })
    }

    fn k_matrix(&self, s: f64) -> DMatrix<Complex64> {
        let bf = self.barrier_matrix(s);
        DMatrix::from_fn(self.n_channels, self.n_channels, |i, j| {
            (0..self.n_resonances)
                .map(|a| {
                    Complex64::from(
                        bf[(i, a)]
                            * bf[(j, a)]
                            * (self.g[(i, a)] * self.g[(j, a)] / (self.mrs[a].powi(2) - s)
                                + self.c[(i, j)]),
                    )
                })
                .sum::<Complex64>()
                * self.adler_zero.map_or(1.0, |az| (s - az.s_0) / az.s_norm)
        })
    }
    fn ikc_inv(&self, s: f64, channel: usize) -> DVector<Complex64> {
        let c_mat = self.c_matrix(s);
        let i_mat = DMatrix::<Complex64>::identity(self.n_channels, self.n_channels);
        let k_mat = self.k_matrix(s);
        let ikc_mat = i_mat + k_mat * c_mat;
        let ikc_inv_mat = ikc_mat.try_inverse().unwrap();
        ikc_inv_mat.row(channel).transpose()
    }

    fn p_vector(
        &self,
        betas: &DVector<Complex64>,
        pvector_constants: &DMatrix<Complex64>,
    ) -> DVector<Complex64> {
        DVector::<Complex64>::from_fn(self.n_channels, |j, _| {
            (0..self.n_resonances)
                .map(|a| betas[a] * pvector_constants[(j, a)])
                .sum()
        })
    }

    pub fn calculate_k_matrix(
        &self,
        betas: &DVector<Complex64>,
        ikc_inv_vec: &DVector<Complex64>,
        pvector_constants_mat: &DMatrix<Complex64>,
    ) -> Complex64 {
        ikc_inv_vec.dot(&self.p_vector(betas, pvector_constants_mat))
    }
}
pub struct KMatrixF0(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixF0 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 5,
                n_channels: 5,
                g: DMatrix::<f64>::from_vec(5, 5, vec![
                     0.74987, -0.01257, 0.02736, -0.15102,  0.36103,
                     0.06401,  0.00204, 0.77413,  0.50999,  0.13112,
                    -0.23417, -0.01032, 0.72283,  0.11934,  0.36792,
                     0.01570,  0.26700, 0.09214,  0.02742, -0.04025,
                    -0.14242,  0.22780, 0.15981,  0.16272, -0.17397,
                ]),
                c: DMatrix::<f64>::from_vec(5, 5, vec![
                     0.03728, 0.00000, -0.01398, -0.02203,  0.01397,
                     0.00000, 0.00000,  0.00000,  0.00000,  0.00000,
                    -0.01398, 0.00000,  0.02349,  0.03101, -0.04003,
                    -0.02203, 0.00000,  0.03101, -0.13769, -0.06722,
                     0.01397, 0.00000, -0.04003, -0.06722, -0.28401,
                ]),
                m1s: vec![0.13498, 0.26995, 0.49368, 0.54786, 0.54786],
                m2s: vec![0.13498, 0.26995, 0.49761, 0.54786, 0.95778],
                mrs: vec![0.51461, 0.90630, 1.23089, 1.46104, 1.69611],
                adler_zero: Some(AdlerZero {
                    s_0: 0.0091125,
                    s_norm: 1.0,
                }),
                l: 0,
            },
            Vec::default())
    }
}

impl Node for KMatrixF0 {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(5, 5, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = DVector::<Complex64>::from_vec(vec![
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
            Complex64::new(parameters[4], parameters[5]),
            Complex64::new(parameters[6], parameters[7]),
            Complex64::new(parameters[8], parameters[9]),
        ]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec![
            "f0_500 re".to_string(),
            "f0_500 im".to_string(),
            "f0_980 re".to_string(),
            "f0_980 im".to_string(),
            "f0_1370 re".to_string(),
            "f0_1370 im".to_string(),
            "f0_1500 re".to_string(),
            "f0_1500 im".to_string(),
            "f0_1710 re".to_string(),
            "f0_1710 im".to_string(),
        ]
    }
}
pub struct KMatrixF2(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixF2 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 4,
                n_channels: 4,
                g: DMatrix::<f64>::from_vec(4, 4, vec![
                     0.40033, 0.01820, -0.06709, -0.49924,
                     0.15479, 0.17300,  0.22941,  0.19295,
                    -0.08900, 0.32393, -0.43133,  0.27975, 
                    -0.00113, 0.15256,  0.23721, -0.03987,
                ]),
                c: DMatrix::<f64>::from_vec(4, 4, vec![
                    -0.04319, 0.00000,  0.00984,  0.01028,
                     0.00000, 0.00000,  0.00000,  0.00000,
                     0.00984, 0.00000, -0.07344,  0.05533,
                     0.01028, 0.00000,  0.05533, -0.05183,
                ]),
                m1s: vec![0.13498, 0.26995, 0.49368, 0.54786],
                m2s: vec![0.13498, 0.26995, 0.49761, 0.54786],
                mrs: vec![1.15299, 1.48359, 1.72923, 1.96700],
                adler_zero: None,
                l: 2,
            },
            Vec::default())
    }
}

impl Node for KMatrixF2 {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(4, 4, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = DVector::<Complex64>::from_vec(vec![
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
            Complex64::new(parameters[4], parameters[5]),
            Complex64::new(parameters[6], parameters[7]),
        ]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec![
            "f2_1270 re".to_string(),
            "f2_1270 im".to_string(),
            "f2_1525 re".to_string(),
            "f2_1525 im".to_string(),
            "f2_1810 re".to_string(),
            "f2_1810 im".to_string(),
            "f2_1950 re".to_string(),
            "f2_1950 im".to_string(),
        ]
    }
}

pub struct KMatrixA0(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixA0 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 2,
                n_channels: 2,
                g: DMatrix::<f64>::from_vec(2, 2, vec![
                     0.43215, 0.19000,
                    -0.28825, 0.43372
                ]),
                c: DMatrix::<f64>::from_vec(2, 2, vec![
                    0.00000, 0.00000,
                    0.00000, 0.00000
                ]),
                m1s: vec![0.13498, 0.49368],
                m2s: vec![0.54786, 0.49761],
                mrs: vec![0.95395, 1.26767],
                adler_zero: None,
                l: 0,
            },
            Vec::default())
    }
}

impl Node for KMatrixA0 {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(2, 2, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = DVector::<Complex64>::from_vec(vec![
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        ]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec![
            "a0_980 re".to_string(),
            "a0_980 im".to_string(),
            "a0_1450 re".to_string(),
            "a0_1450 im".to_string(),
        ]
    }
}

pub struct KMatrixA2(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixA2 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 2,
                n_channels: 3,
                g: DMatrix::<f64>::from_vec(3, 2, vec![
                     0.30073, 0.68567,
                     0.21426, 0.12543, 
                    -0.09162, 0.00184
                ]),
                c: DMatrix::<f64>::from_vec(3, 3, vec![
                    -0.40184,  0.00033, -0.08707,
                     0.00033, -0.21416, -0.06193,
                    -0.08707, -0.06193, -0.17435,
                ]),
                m1s: vec![0.13498, 0.49368, 0.13498],
                m2s: vec![0.54786, 0.49761, 0.95778],
                mrs: vec![1.30080, 1.75351],
                adler_zero: None,
                l: 2,
            },
            Vec::default())
    }
}

impl Node for KMatrixA2 {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(3, 2, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = DVector::<Complex64>::from_vec(vec![
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        ]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec![
            "a2_1320 re".to_string(),
            "a2_1320 im".to_string(),
            "a2_1700 re".to_string(),
            "a2_1700 im".to_string(),
        ]
    }
}

pub struct KMatrixRho(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixRho {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 2,
                n_channels: 3,
                g: DMatrix::<f64>::from_vec(3, 2, vec![
                    0.28023, 0.16318,
                    0.01806, 0.53879, 
                    0.06501, 0.00495,
                ]),
                c: DMatrix::<f64>::from_vec(3, 3, vec![
                    -0.06948, 0.00000,  0.07958,
                     0.00000, 0.00000,  0.00000,
                     0.07958, 0.00000, -0.60000,
                ]),
                m1s: vec![0.13498, 0.26995, 0.49368],
                m2s: vec![0.13498, 0.26995, 0.49761],
                mrs: vec![0.71093, 1.58660],
                adler_zero: None,
                l: 1,
            },
            Vec::default())
    }
}

impl Node for KMatrixRho {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(3, 2, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = DVector::<Complex64>::from_vec(vec![
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        ]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec![
            "rho_770 re".to_string(),
            "rho_770 im".to_string(),
            "rho_1700 re".to_string(),
            "rho_1700 im".to_string(),
        ]
    }
}

pub struct KMatrixPi1(
    usize,
    KMatrixConstants,
    Vec<(DVector<Complex64>, DMatrix<Complex64>)>,
);
#[rustfmt::skip]
impl KMatrixPi1 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                n_resonances: 1,
                n_channels: 2,
                g: DMatrix::<f64>::from_vec(2, 1, vec![
                    0.80564,
                    1.04595
                ]),
                c: DMatrix::<f64>::from_vec(2, 2, vec![
                    1.05000,  0.15163,
                    0.15163, -0.24611,
                ]),
                m1s: vec![0.13498, 0.13498],
                m2s: vec![0.54786, 0.95778],
                mrs: vec![1.38552],
                adler_zero: None,
                l: 1,
            },
            Vec::default())
    }
}

impl Node for KMatrixPi1 {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.2 = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let s = (event.daughter_p4s[0] + event.daughter_p4s[1]).m2();
                let barrier_mat = self.1.barrier_matrix(s);
                let pvector_constants = DMatrix::<Complex64>::from_fn(2, 1, |i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas =
            DVector::<Complex64>::from_vec(vec![Complex64::new(parameters[0], parameters[1])]);
        let (ikc_inv_vec, pvector_constants_mat) = &self.2[event.index];
        Ok(self
            .1
            .calculate_k_matrix(&betas, ikc_inv_vec, pvector_constants_mat))
    }
    fn parameters(&self) -> Vec<String> {
        vec!["pi1_1600 re".to_string(), "pi1_1600 im".to_string()]
    }
}
