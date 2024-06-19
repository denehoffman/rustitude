use crate::utils::blatt_weisskopf;
use crate::utils::breakup_momentum;
use std::f64::consts::PI;

use nalgebra::{SMatrix, SVector};
use rayon::prelude::*;
use rustitude_core::prelude::*;

#[derive(Default, Clone)]
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
#[derive(Clone)]
struct KMatrixConstants<const C: usize, const R: usize> {
    g: SMatrix<f64, C, R>,
    c: SMatrix<f64, C, C>,
    m1s: [f64; C],
    m2s: [f64; C],
    mrs: [f64; R],
    adler_zero: Option<AdlerZero>,
    l: usize,
}

impl<const C: usize, const R: usize> KMatrixConstants<C, R> {
    fn chi_plus(s: f64, m1: f64, m2: f64) -> f64 {
        1.0 - ((m1 + m2) * (m1 + m2)) / s
    }

    fn chi_minus(s: f64, m1: f64, m2: f64) -> f64 {
        1.0 - ((m1 - m2) * (m1 - m2)) / s
    }

    fn rho(s: f64, m1: f64, m2: f64) -> Complex64 {
        Complex64::from(Self::chi_plus(s, m1, m2) * Self::chi_minus(s, m1, m2)).sqrt()
    }
    fn c_matrix(&self, s: f64) -> SMatrix<Complex64, C, C> {
        SMatrix::from_diagonal(&SVector::from_fn(|i, _| {
            Self::rho(s, self.m1s[i], self.m2s[i]) / PI
                * ((Self::chi_plus(s, self.m1s[i], self.m2s[i])
                    + Self::rho(s, self.m1s[i], self.m2s[i]))
                    / (Self::chi_plus(s, self.m1s[i], self.m2s[i])
                        - Self::rho(s, self.m1s[i], self.m2s[i])))
                .ln()
                - Self::chi_plus(s, self.m1s[i], self.m2s[i]) / PI
                    * ((self.m2s[i] - self.m1s[i]) / (self.m1s[i] + self.m2s[i]))
                    * (self.m2s[i] / self.m1s[i]).ln()
        }))
    }
    fn barrier_factor(s: f64, m1: f64, m2: f64, mr: f64, l: usize) -> f64 {
        blatt_weisskopf(s.sqrt(), m1, m2, l) / blatt_weisskopf(mr, m1, m2, l)
    }
    fn barrier_matrix(&self, s: f64) -> SMatrix<f64, C, R> {
        SMatrix::from_fn(|i, a| {
            Self::barrier_factor(s, self.m1s[i], self.m2s[i], self.mrs[a], self.l)
        })
    }

    fn k_matrix(&self, s: f64) -> SMatrix<Complex64, C, C> {
        let bf = self.barrier_matrix(s);
        SMatrix::from_fn(|i, j| {
            (0..R)
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

    fn pole_product(&self, _s: f64) -> f64 {
        1.0
        // (0..R).map(|a| (self.mrs[a].powi(2) - s)).sum()
    }

    fn ikc_inv(&self, s: f64, channel: usize) -> SVector<Complex64, C> {
        let i_mat = SMatrix::<Complex64, C, C>::identity();
        let k_mat = self.k_matrix(s);
        let c_mat = self.c_matrix(s);
        let ikc_mat = (i_mat + k_mat * c_mat).scale(self.pole_product(s));
        let ikc_inv_mat = ikc_mat.try_inverse().unwrap();
        ikc_inv_mat.row(channel).transpose()
    }

    fn p_vector(
        betas: &SVector<Complex64, R>,
        pvector_constants: &SMatrix<Complex64, C, R>,
    ) -> SVector<Complex64, C> {
        SVector::<Complex64, C>::from_fn(|j, _| {
            (0..R).map(|a| betas[a] * pvector_constants[(j, a)]).sum()
        })
    }

    pub fn calculate_k_matrix(
        betas: &SVector<Complex64, R>,
        ikc_inv_vec: &SVector<Complex64, C>,
        pvector_constants_mat: &SMatrix<Complex64, C, R>,
    ) -> Complex64 {
        ikc_inv_vec.dot(&Self::p_vector(betas, pvector_constants_mat))
    }
}
#[derive(Clone)]
pub struct KMatrixF0(
    usize,
    KMatrixConstants<5, 5>,
    Vec<(SVector<Complex64, 5>, SMatrix<Complex64, 5, 5>)>,
);
#[rustfmt::skip]
impl KMatrixF0 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 5, 5>::new(
                     0.74987, 0.06401, -0.23417,  0.01270, -0.14242,  
                    -0.01257, 0.00204, -0.01032,  0.26700,  0.22780, 
                     0.27536, 0.77413,  0.72283,  0.09214,  0.15981,  
                    -0.15102, 0.50999,  0.11934,  0.02742,  0.16272, 
                     0.36103, 0.13112,  0.36792, -0.04025, -0.17397,
                ),
                c: SMatrix::<f64, 5, 5>::new(
                     0.03728, 0.00000, -0.01398, -0.02203,  0.01397,
                     0.00000, 0.00000,  0.00000,  0.00000,  0.00000,
                    -0.01398, 0.00000,  0.02349,  0.03101, -0.04003,
                    -0.02203, 0.00000,  0.03101, -0.13769, -0.06722,
                     0.01397, 0.00000, -0.04003, -0.06722, -0.28401,
                ),
                m1s: [0.1349768, 2.0 * 0.1349768, 0.493677, 0.547862, 0.547862],
                m2s: [0.1349768, 2.0 * 0.1349768, 0.497611, 0.547862, 0.95778],
                mrs: [0.51461, 0.90630, 1.23089, 1.46104, 1.69611],
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
                let pvector_constants = SMatrix::<Complex64, 5, 5>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 5>::new(
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
            Complex64::new(parameters[4], parameters[5]),
            Complex64::new(parameters[6], parameters[7]),
            Complex64::new(parameters[8], parameters[9]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
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
#[derive(Clone)]
pub struct KMatrixF2(
    usize,
    KMatrixConstants<4, 4>,
    Vec<(SVector<Complex64, 4>, SMatrix<Complex64, 4, 4>)>,
);
#[rustfmt::skip]
impl KMatrixF2 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 4, 4>::new(
                     0.40033, 0.01820, -0.06709, -0.49924,
                     0.15479, 0.17300,  0.22941,  0.19295,
                    -0.08900, 0.32393, -0.43133,  0.27975, 
                    -0.00113, 0.15256,  0.23721, -0.03987,
                ),
                c: SMatrix::<f64, 4, 4>::new(
                    -0.04319, 0.00000,  0.00984,  0.01028,
                     0.00000, 0.00000,  0.00000,  0.00000,
                     0.00984, 0.00000, -0.07344,  0.05533,
                     0.01028, 0.00000,  0.05533, -0.05183,
                ),
                m1s: [0.1349768, 2.0 * 0.1349768, 0.493677, 0.547862],
                m2s: [0.1349768, 2.0 * 0.1349768, 0.497611, 0.547862],
                mrs: [1.15299, 1.48359, 1.72923, 1.96700],
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
                let pvector_constants = SMatrix::<Complex64, 4, 4>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 4>::new(
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
            Complex64::new(parameters[4], parameters[5]),
            Complex64::new(parameters[6], parameters[7]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
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

#[derive(Clone)]
pub struct KMatrixA0(
    usize,
    KMatrixConstants<2, 2>,
    Vec<(SVector<Complex64, 2>, SMatrix<Complex64, 2, 2>)>,
);
#[rustfmt::skip]
impl KMatrixA0 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 2, 2>::new(
                     0.43215, 0.19000,
                    -0.28825, 0.43372
                ),
                c: SMatrix::<f64, 2, 2>::new(
                    0.00000, 0.00000,
                    0.00000, 0.00000
                ),
                m1s: [0.1349768, 0.493677],
                m2s: [0.547862, 0.497611],
                mrs: [0.95395, 1.26767],
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
                let pvector_constants = SMatrix::<Complex64, 2, 2>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 2>::new(
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
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

#[derive(Clone)]
pub struct KMatrixA2(
    usize,
    KMatrixConstants<3, 2>,
    Vec<(SVector<Complex64, 3>, SMatrix<Complex64, 3, 2>)>,
);
#[rustfmt::skip]
impl KMatrixA2 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 3, 2>::new(
                     0.30073, 0.68567,
                     0.21426, 0.12543, 
                    -0.09162, 0.00184
                ),
                c: SMatrix::<f64, 3, 3>::new(
                    -0.40184,  0.00033, -0.08707,
                     0.00033, -0.21416, -0.06193,
                    -0.08707, -0.06193, -0.17435,
                ),
                m1s: [0.1349768, 0.493677, 0.1349768],
                m2s: [0.547862, 0.497611, 0.95778],
                mrs: [1.30080, 1.75351],
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
                let pvector_constants = SMatrix::<Complex64, 3, 2>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 2>::new(
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
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

#[derive(Clone)]
pub struct KMatrixRho(
    usize,
    KMatrixConstants<3, 2>,
    Vec<(SVector<Complex64, 3>, SMatrix<Complex64, 3, 2>)>,
);
#[rustfmt::skip]
impl KMatrixRho {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 3, 2>::new(
                    0.28023, 0.16318,
                    0.01806, 0.53879, 
                    0.06501, 0.00495,
                ),
                c: SMatrix::<f64, 3, 3>::new(
                    -0.06948, 0.00000,  0.07958,
                     0.00000, 0.00000,  0.00000,
                     0.07958, 0.00000, -0.60000,
                ),
                m1s: [0.1349768, 2.0 * 0.1349768, 0.493677],
                m2s: [0.1349768, 2.0 * 0.1349768, 0.497611],
                mrs: [0.71093, 1.58660],
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
                let pvector_constants = SMatrix::<Complex64, 3, 2>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 2>::new(
            Complex64::new(parameters[0], parameters[1]),
            Complex64::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
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

#[derive(Clone)]
pub struct KMatrixPi1(
    usize,
    KMatrixConstants<2, 1>,
    Vec<(SVector<Complex64, 2>, SMatrix<Complex64, 2, 1>)>,
);
#[rustfmt::skip]
impl KMatrixPi1 {
    pub fn new(channel: usize) -> Self {
        Self(channel,
             KMatrixConstants {
                g: SMatrix::<f64, 2, 1>::new(
                    0.80564,
                    1.04595
                ),
                c: SMatrix::<f64, 2, 2>::new(
                    1.05000,  0.15163,
                    0.15163, -0.24611,
                ),
                m1s: [0.1349768, 0.1349768],
                m2s: [0.547862, 0.95778],
                mrs: [1.38552],
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
                let pvector_constants = SMatrix::<Complex64, 2, 1>::from_fn(|i, a| {
                    Complex64::from(barrier_mat[(i, a)]) * self.1.g[(i, a)]
                        / (self.1.mrs[a].powi(2) - s)
                        * self.1.pole_product(s)
                });
                (self.1.ikc_inv(s, self.0), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let betas = SVector::<Complex64, 1>::new(Complex64::new(parameters[0], parameters[1]));
        let (ikc_inv_vec, pvector_constants_mat) = self.2[event.index];
        Ok(KMatrixConstants::calculate_k_matrix(
            &betas,
            &ikc_inv_vec,
            &pvector_constants_mat,
        ))
    }
    fn parameters(&self) -> Vec<String> {
        vec!["pi1_1600 re".to_string(), "pi1_1600 im".to_string()]
    }
}
