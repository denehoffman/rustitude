use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::SHCoordinates;

use crate::utils::{Decay, Frame};

#[derive(Clone)]
pub struct TwoPiSDME<F: Field> {
    decay: Decay,
    frame: Frame,
    data: Vec<(F, F, F, F, F, F)>,
}

impl<F: Field> TwoPiSDME<F> {
    pub fn new(decay: Decay, frame: Frame) -> Self {
        match decay {
            Decay::TwoBodyDecay(_) => Self {
                decay,
                frame,
                data: Vec::default(),
            },

            _ => unimplemented!(),
        }
    }
}

impl<F: Field> Node<F> for TwoPiSDME<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, y, _, p) = self.decay.coordinates(self.frame, 0, event);
                let big_phi = y.dot(&event.eps).fatan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos().fpowi(2),
                    F::fsin(p.theta()).fpowi(2),
                    F::fsin(F::TWO * p.theta()),
                    p.phi(),
                    big_phi,
                    pgamma,
                )
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        let (cossqtheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let rho_000 = parameters[0];
        let rho_100 = parameters[1];
        let rho_1n10 = parameters[2];
        let rho_111 = parameters[3];
        let rho_001 = parameters[4];
        let rho_101 = parameters[5];
        let rho_1n11 = parameters[6];
        let rho_102 = parameters[7];
        let rho_1n12 = parameters[8];

        Ok(F::fsqrt(F::fabs(
            (F::THREE / (F::FOUR * F::PI()))
                * (F::f(0.5) * (F::ONE - rho_000)
                    + F::f(0.5) * (F::THREE * rho_000 - F::ONE) * cossqtheta
                    - F::SQRT_2() * rho_100 * sin2theta * F::fcos(phi)
                    - rho_1n10 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fcos(F::TWO * big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * cossqtheta
                        - F::SQRT_2() * rho_101 * sin2theta * F::fcos(phi)
                        - rho_1n11 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fsin(F::TWO * big_phi)
                    * (F::SQRT_2() * rho_102 * sin2theta * F::fsin(phi)
                        + rho_1n12 * sinsqtheta * F::fsin(F::TWO * phi)),
        ))
        .c())
    }

    fn parameters(&self) -> Vec<String> {
        vec![
            "rho_000".to_string(),
            "rho_100".to_string(),
            "rho_1n10".to_string(),
            "rho_111".to_string(),
            "rho_001".to_string(),
            "rho_101".to_string(),
            "rho_1n11".to_string(),
            "rho_102".to_string(),
            "rho_1n12".to_string(),
        ]
    }
}

#[derive(Clone)]
pub struct ThreePiSDME<F: Field> {
    decay: Decay,
    frame: Frame,
    data: Vec<(F, F, F, F, F, F)>,
}

impl<F: Field> ThreePiSDME<F> {
    pub fn new(decay: Decay, frame: Frame) -> Self {
        match decay {
            Decay::ThreeBodyDecay(_) => Self {
                decay,
                frame,
                data: Vec::default(),
            },

            _ => unimplemented!(),
        }
    }
}

impl<F: Field> Node<F> for ThreePiSDME<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let res_p4 = self.decay.resonance_p4(event);
                let p1_res_p4 = self.decay.primary_p4(event).boost_along(&res_p4);
                let p2_res_p4 = self.decay.primary_p4(event).boost_along(&res_p4);
                let norm = p1_res_p4
                    .momentum()
                    .cross(&p2_res_p4.momentum())
                    .normalize();
                let (_, y, _, p) = self
                    .frame
                    .coordinates_from_boosted_vec(self.decay, &norm, event);
                let big_phi = F::fatan2(
                    y.dot(&event.eps),
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos().fpowi(2),
                    F::fsin(p.theta()).fpowi(2),
                    F::fsin(F::TWO * p.theta()),
                    p.phi(),
                    big_phi,
                    pgamma,
                )
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        let (cossqtheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let rho_000 = parameters[0];
        let rho_100 = parameters[1];
        let rho_1n10 = parameters[2];
        let rho_111 = parameters[3];
        let rho_001 = parameters[4];
        let rho_101 = parameters[5];
        let rho_1n11 = parameters[6];
        let rho_102 = parameters[7];
        let rho_1n12 = parameters[8];

        Ok(F::fsqrt(F::fabs(
            (F::THREE / (F::FOUR * F::PI()))
                * (F::f(0.5) * (F::ONE - rho_000)
                    + F::f(0.5) * (F::THREE * rho_000 - F::ONE) * cossqtheta
                    - F::SQRT_2() * rho_100 * sin2theta * F::fcos(phi)
                    - rho_1n10 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fcos(F::TWO * big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * cossqtheta
                        - F::SQRT_2() * rho_101 * sin2theta * F::fcos(phi)
                        - rho_1n11 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fsin(F::TWO * big_phi)
                    * (F::SQRT_2() * rho_102 * sin2theta * F::fsin(phi)
                        + rho_1n12 * sinsqtheta * F::fsin(F::TWO * phi)),
        ))
        .c())
    }

    fn parameters(&self) -> Vec<String> {
        vec![
            "rho_000".to_string(),
            "rho_100".to_string(),
            "rho_1n10".to_string(),
            "rho_111".to_string(),
            "rho_001".to_string(),
            "rho_101".to_string(),
            "rho_1n11".to_string(),
            "rho_102".to_string(),
            "rho_1n12".to_string(),
        ]
    }
}

#[derive(Clone)]
pub struct VecRadiativeSDME<F: Field> {
    decay: Decay,
    frame: Frame,
    data: Vec<(F, F, F, F, F, F)>,
}

impl<F: Field> VecRadiativeSDME<F> {
    pub fn new(decay: Decay, frame: Frame) -> Self {
        match decay {
            Decay::TwoBodyDecay(_) => Self {
                decay,
                frame,
                data: Vec::default(),
            },

            _ => unimplemented!(),
        }
    }
}

impl<F: Field> Node<F> for VecRadiativeSDME<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, y, _, p) = self.decay.coordinates(self.frame, 0, event);
                let big_phi = y.dot(&event.eps).fatan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos().fpowi(2),
                    F::fsin(p.theta()).fpowi(2),
                    F::fsin(F::TWO * p.theta()),
                    p.phi(),
                    big_phi,
                    pgamma,
                )
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        let (cossqtheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let rho_000 = parameters[0];
        let rho_100 = parameters[1];
        let rho_1n10 = parameters[2];
        let rho_111 = parameters[3];
        let rho_001 = parameters[4];
        let rho_101 = parameters[5];
        let rho_1n11 = parameters[6];
        let rho_102 = parameters[7];
        let rho_1n12 = parameters[8];

        Ok(F::fsqrt(F::fabs(
            (F::THREE / (F::EIGHT * F::PI()))
                * (F::ONE - sinsqtheta * F::f(0.5) * (F::ONE - rho_000) - rho_000 * cossqtheta
                    + rho_1n10 * sinsqtheta * F::fcos(F::TWO * phi)
                    + F::SQRT_2() * rho_100 * sin2theta * F::fcos(phi)
                    - pgamma
                        * F::fcos(F::TWO * big_phi)
                        * (F::TWO * rho_111
                            + (rho_001 - rho_111) * sinsqtheta
                            + rho_1n11 * sinsqtheta * F::fcos(F::TWO * phi)
                            + F::SQRT_2() * rho_101 * sin2theta * F::fcos(phi))
                    + pgamma
                        * F::fsin(F::TWO * big_phi)
                        * (rho_1n12 * sinsqtheta * F::fsin(F::TWO * phi)
                            + F::SQRT_2() * rho_102 * sin2theta * F::fsin(phi))),
        ))
        .c())
    }

    fn parameters(&self) -> Vec<String> {
        vec![
            "rho_000".to_string(),
            "rho_100".to_string(),
            "rho_1n10".to_string(),
            "rho_111".to_string(),
            "rho_001".to_string(),
            "rho_101".to_string(),
            "rho_1n11".to_string(),
            "rho_102".to_string(),
            "rho_1n12".to_string(),
        ]
    }
}
