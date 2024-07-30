use crate::utils::blatt_weisskopf;
use crate::utils::breakup_momentum;
use crate::utils::Decay;

use nalgebra::{SMatrix, SVector};
use rayon::prelude::*;
use rustitude_core::prelude::*;

#[derive(Default, Clone)]
pub struct BreitWigner<F: Field> {
    l: usize,
    decay: Decay,
    m: Vec<F>,
    m1: Vec<F>,
    m2: Vec<F>,
    q: Vec<F>,
    f: Vec<F>,
}
impl<F: Field> BreitWigner<F> {
    pub fn new(l: usize, decay: Decay) -> Self {
        Self {
            l,
            decay,
            ..Default::default()
        }
    }
}
impl<F: Field> Node<F> for BreitWigner<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        (self.m, (self.m1, (self.m2, (self.q, self.f)))) = dataset
            .events
            .par_iter()
            .map(|event| {
                let p1 = self.decay.primary_p4(event);
                let p2 = self.decay.secondary_p4(event);
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

    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let m = self.m[event.index];
        let m1 = self.m1[event.index];
        let m2 = self.m2[event.index];
        let q = self.q[event.index];
        let f = self.f[event.index];
        let m0 = parameters[0];
        let g0 = parameters[1];
        let f0 = blatt_weisskopf(m0, m1, m2, self.l);
        let q0 = breakup_momentum(m0, m1, m2);
        let g = g0 * (m0 / m) * (q / q0) * (f.fpowi(2) / f0.fpowi(2));
        Ok(Complex::new(f * (m0 * g0 / F::PI()), F::ZERO) / Complex::new(m0.fpowi(2) - m.fpowi(2), -F::ONE * m0 * g))
    }

    fn parameters(&self) -> Vec<String> {
        vec!["mass".to_string(), "width".to_string()]
    }
}

#[derive(Clone, Copy)]
pub struct AdlerZero<F: Field> {
    pub s_0: F,
    pub s_norm: F,
}
#[derive(Clone)]
struct KMatrixConstants<F: Field, const C: usize, const R: usize> {
    g: SMatrix<F, C, R>,
    c: SMatrix<F, C, C>,
    m1s: [F; C],
    m2s: [F; C],
    mrs: [F; R],
    adler_zero: Option<AdlerZero<F>>,
    l: usize,
}

impl<F: Field, const C: usize, const R: usize> KMatrixConstants<F, C, R> {
    fn chi_plus(s: F, m1: F, m2: F) -> Complex<F> {
        (F::ONE - ((m1 + m2) * (m1 + m2)) / s).into()
    }

    fn chi_minus(s: F, m1: F, m2: F) -> Complex<F> {
        (F::ONE - ((m1 - m2) * (m1 - m2)) / s).into()
    }

    fn rho(s: F, m1: F, m2: F) -> Complex<F> {
        (Self::chi_plus(s, m1, m2) * Self::chi_minus(s, m1, m2)).sqrt()
    }
    fn c_matrix(&self, s: F) -> SMatrix<Complex<F>, C, C> {
        SMatrix::from_diagonal(&SVector::from_fn(|i, _| {
            Self::rho(s, self.m1s[i], self.m2s[i]) / F::PI()
                * ((Self::chi_plus(s, self.m1s[i], self.m2s[i])
                    + Self::rho(s, self.m1s[i], self.m2s[i]))
                    / (Self::chi_plus(s, self.m1s[i], self.m2s[i])
                        - Self::rho(s, self.m1s[i], self.m2s[i])))
                .ln()
                - Self::chi_plus(s, self.m1s[i], self.m2s[i]) / F::PI()
                    * ((self.m2s[i] - self.m1s[i]) / (self.m1s[i] + self.m2s[i]))
                    * (self.m2s[i] / self.m1s[i]).fln()
        }))
    }
    fn barrier_factor(s: F, m1: F, m2: F, mr: F, l: usize) -> F {
        blatt_weisskopf(s.fsqrt(), m1, m2, l) / blatt_weisskopf(mr, m1, m2, l)
    }
    fn barrier_matrix(&self, s: F) -> SMatrix<F, C, R> {
        SMatrix::from_fn(|i, a| {
            Self::barrier_factor(s, self.m1s[i], self.m2s[i], self.mrs[a], self.l)
        })
    }

    fn k_matrix(&self, s: F) -> SMatrix<Complex<F>, C, C> {
        let bf = self.barrier_matrix(s);
        SMatrix::from_fn(|i, j| {
            (0..R)
                .map(|a| {
                    Complex::from(
                        bf[(i, a)]
                            * bf[(j, a)]
                            * (self.g[(i, a)] * self.g[(j, a)]
                                + (self.c[(i, j)]) * (self.mrs[a].fpowi(2) - s)),
                    ) * self.pole_product_remainder(s, a)
                })
                .sum::<Complex<F>>()
                * self.adler_zero.map_or(F::ONE, |az| (s - az.s_0) / az.s_norm)
        })
    }

    fn pole_product_remainder(&self, s: F, a_i: usize) -> F {
        (0..R)
            .filter_map(|a| {
                if a != a_i {
                    Some(self.mrs[a].fpowi(2) - s)
                } else {
                    None
                }
            })
            .product()
    }
    fn pole_product(&self, s: F) -> F {
        (0..R).map(|a| (self.mrs[a].fpowi(2) - s)).product()
    }

    fn ikc_inv(&self, s: F, channel: usize) -> SVector<Complex<F>, C> {
        let i_mat = SMatrix::<Complex<F>, C, C>::identity().scale(self.pole_product(s));
        let k_mat = self.k_matrix(s);
        let c_mat = self.c_matrix(s);
        let ikc_mat = i_mat + k_mat * c_mat;
        let ikc_inv_mat = ikc_mat.try_inverse().unwrap();
        ikc_inv_mat.row(channel).transpose()
    }

    fn p_vector(
        betas: &SVector<Complex<F>, R>,
        pvector_constants: &SMatrix<Complex<F>, C, R>,
    ) -> SVector<Complex<F>, C> {
        SVector::<Complex<F>, C>::from_fn(|j, _| {
            (0..R).map(|a| betas[a] * pvector_constants[(j, a)]).sum()
        })
    }

    pub fn calculate_k_matrix(
        betas: &SVector<Complex<F>, R>,
        ikc_inv_vec: &SVector<Complex<F>, C>,
        pvector_constants_mat: &SMatrix<Complex<F>, C, R>,
    ) -> Complex<F> {
        ikc_inv_vec.dot(&Self::p_vector(betas, pvector_constants_mat))
    }
}
#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct KMatrixF0<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 5, 5>,
    data: Vec<(SVector<Complex<F>, 5>, SMatrix<Complex<F>, 5, 5>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixF0<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 5, 5>::from_vec(F::fv(vec![
                     0.74987, -0.01257,  0.27536, -0.15102,  0.36103,
                     0.06401,  0.00204,  0.77413,  0.50999,  0.13112,
                    -0.23417, -0.01032,  0.72283,  0.11934,  0.36792, 
                     0.01270,  0.26700,  0.09214,  0.02742, -0.04025,
                    -0.14242,  0.22780,  0.15981,  0.16272, -0.17397,  
                ])),
                c: SMatrix::<F, 5, 5>::from_vec(F::fv(vec![
                     0.03728,  0.00000, -0.01398, -0.02203,  0.01397,
                     0.00000,  0.00000,  0.00000,  0.00000,  0.00000,
                    -0.01398,  0.00000,  0.02349,  0.03101, -0.04003,
                    -0.02203,  0.00000,  0.03101, -0.13769, -0.06722,
                     0.01397,  0.00000, -0.04003, -0.06722, -0.28401,
                ])),
                m1s: F::fa([0.1349768, 2.0 * 0.1349768, 0.493677, 0.547862, 0.547862]),
                m2s: F::fa([0.1349768, 2.0 * 0.1349768, 0.497611, 0.547862, 0.95778]),
                mrs: F::fa([0.51461, 0.90630, 1.23089, 1.46104, 1.69611]),
                adler_zero: Some(AdlerZero {
                    s_0: F::f(0.0091125),
                    s_norm: F::ONE,
                }),
                l: 0,
            },
            data: Vec::default(),
        }
    }
}

impl<F: Field> Node<F> for KMatrixF0<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 5, 5>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas = SVector::<Complex<F>, 5>::new(
            Complex::new(parameters[0], parameters[1]),
            Complex::new(parameters[2], parameters[3]),
            Complex::new(parameters[4], parameters[5]),
            Complex::new(parameters[6], parameters[7]),
            Complex::new(parameters[8], parameters[9]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
#[allow(clippy::type_complexity)]
pub struct KMatrixF2<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 4, 4>,
    data: Vec<(SVector<Complex<F>, 4>, SMatrix<Complex<F>, 4, 4>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixF2<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 4, 4>::from_vec(F::fv(vec![
                     0.40033,  0.15479, -0.08900, -0.00113,
                     0.01820,  0.17300,  0.32393,  0.15256,
                    -0.06709,  0.22941, -0.43133,  0.23721,
                    -0.49924,  0.19295,  0.27975, -0.03987,
                ])),
                c: SMatrix::<F, 4, 4>::from_vec(F::fv(vec![
                    -0.04319,  0.00000,  0.00984,  0.01028,
                     0.00000,  0.00000,  0.00000,  0.00000,
                     0.00984,  0.00000, -0.07344,  0.05533,
                     0.01028,  0.00000,  0.05533, -0.05183,
                ])),
                m1s: F::fa([0.1349768, 2.0 * 0.1349768, 0.493677, 0.547862]),
                m2s: F::fa([0.1349768, 2.0 * 0.1349768, 0.497611, 0.547862]),
                mrs: F::fa([1.15299, 1.48359, 1.72923, 1.96700]),
                adler_zero: None,
                l: 2,
            },
            data: Vec::default()
        }
    }
}

impl<F: Field> Node<F> for KMatrixF2<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 4, 4>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas = SVector::<Complex<F>, 4>::new(
            Complex::new(parameters[0], parameters[1]),
            Complex::new(parameters[2], parameters[3]),
            Complex::new(parameters[4], parameters[5]),
            Complex::new(parameters[6], parameters[7]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
#[allow(clippy::type_complexity)]
pub struct KMatrixA0<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 2, 2>,
    data: Vec<(SVector<Complex<F>, 2>, SMatrix<Complex<F>, 2, 2>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixA0<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 2, 2>::from_vec(F::fv(vec![
                    0.43215, -0.28825, 
                    0.19000,  0.43372
                ])),
                c: SMatrix::<F, 2, 2>::from_vec(F::fv(vec![
                    0.00000,  0.00000,
                    0.00000,  0.00000
                ])),
                m1s: F::fa([0.1349768, 0.493677]),
                m2s: F::fa([0.547862, 0.497611]),
                mrs: F::fa([0.95395, 1.26767]),
                adler_zero: None,
                l: 0,
            },
            data: Vec::default()
        }
    }
}

impl<F: Field> Node<F> for KMatrixA0<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 2, 2>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas = SVector::<Complex<F>, 2>::new(
            Complex::new(parameters[0], parameters[1]),
            Complex::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
#[allow(clippy::type_complexity)]
pub struct KMatrixA2<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 3, 2>,
    data: Vec<(SVector<Complex<F>, 3>, SMatrix<Complex<F>, 3, 2>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixA2<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 3, 2>::from_vec(F::fv(vec![
                     0.30073,  0.21426, -0.09162,
                     0.68567,  0.12543,  0.00184 
                ])),
                c: SMatrix::<F, 3, 3>::from_vec(F::fv(vec![
                    -0.40184,  0.00033, -0.08707,
                     0.00033, -0.21416, -0.06193,
                    -0.08707, -0.06193, -0.17435,
                ])),
                m1s: F::fa([0.1349768, 0.493677, 0.1349768]),
                m2s: F::fa([0.547862, 0.497611, 0.95778]),
                mrs: F::fa([1.30080, 1.75351]),
                adler_zero: None,
                l: 2,
            },
            data: Vec::default()
        }
    }
}

impl<F: Field> Node<F> for KMatrixA2<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 3, 2>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas = SVector::<Complex<F>, 2>::new(
            Complex::new(parameters[0], parameters[1]),
            Complex::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
#[allow(clippy::type_complexity)]
pub struct KMatrixRho<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 3, 2>,
    data: Vec<(SVector<Complex<F>, 3>, SMatrix<Complex<F>, 3, 2>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixRho<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 3, 2>::from_vec(F::fv(vec![
                     0.28023,  0.01806,  0.06501,
                     0.16318,  0.53879,  0.00495,
                ])),
                c: SMatrix::<F, 3, 3>::from_vec(F::fv(vec![
                    -0.06948,  0.00000,  0.07958,
                     0.00000,  0.00000,  0.00000,
                     0.07958,  0.00000, -0.60000,
                ])),
                m1s: F::fa([0.1349768, 2.0 * 0.1349768, 0.493677]),
                m2s: F::fa([0.1349768, 2.0 * 0.1349768, 0.497611]),
                mrs: F::fa([0.71093, 1.58660]),
                adler_zero: None,
                l: 1,
            },
            data: Vec::default(),
        }
    }
}

impl<F: Field> Node<F> for KMatrixRho<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 3, 2>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas = SVector::<Complex<F>, 2>::new(
            Complex::new(parameters[0], parameters[1]),
            Complex::new(parameters[2], parameters[3]),
        );
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
#[allow(clippy::type_complexity)]
pub struct KMatrixPi1<F: Field> {
    channel: usize,
    decay: Decay,
    constants: KMatrixConstants<F, 2, 1>,
    data: Vec<(SVector<Complex<F>, 2>, SMatrix<Complex<F>, 2, 1>)>,
}
#[rustfmt::skip]
impl<F: Field> KMatrixPi1<F> {
    pub fn new(channel: usize, decay: Decay) -> Self {
        Self {
            channel,
            decay,
            constants: KMatrixConstants {
                g: SMatrix::<F, 2, 1>::from_vec(F::fv(vec![
                    0.80564,  1.04595
                ])),
                c: SMatrix::<F, 2, 2>::from_vec(F::fv(vec![
                    1.05000,  0.15163,
                    0.15163, -0.24611,
                ])),
                m1s: F::fa([0.1349768, 0.1349768]),
                m2s: F::fa([0.547862, 0.95778]),
                mrs: F::fa([1.38552]),
                adler_zero: None,
                l: 1,
            },
            data: Vec::default()
        }
    }
}

impl<F: Field> Node<F> for KMatrixPi1<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let s = self.decay.resonance_p4(event).m2();
                let barrier_mat = self.constants.barrier_matrix(s);
                let pvector_constants = SMatrix::<Complex<F>, 2, 1>::from_fn(|i, a| {
                    Complex::from(barrier_mat[(i, a)])
                        * self.constants.g[(i, a)]
                        * self.constants.pole_product_remainder(s, a)
                });
                (self.constants.ikc_inv(s, self.channel), pvector_constants)
            })
            .collect();
        Ok(())
    }
    fn calculate(
        &self,
        parameters: &[F],
        event: &Event<F>,
    ) -> Result<Complex<F>, RustitudeError> {
        let betas =
            SVector::<Complex<F>, 1>::new(Complex::new(parameters[0], parameters[1]));
        let (ikc_inv_vec, pvector_constants_mat) = self.data[event.index];
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
