use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::SHCoordinates;

use crate::utils::Frame;

#[derive(Clone)]
pub struct TwoPiSDME<F: Field> {
    frame: Frame,
    data: Vec<(F, F, F, F, F, F)>,
}

impl<F: Field> TwoPiSDME<F> {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            data: Vec::default(),
        }
    }
}

impl<F: Field> Node<F> for TwoPiSDME<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let resonance = event.daughter_p4s[0] + event.daughter_p4s[1];
                let beam_res_vec = event.beam_p4.boost_along(&resonance).momentum();
                let recoil_res_vec = event.recoil_p4.boost_along(&resonance).momentum();
                let daughter_res_vec = event.daughter_p4s[0].boost_along(&resonance).momentum();
                let (_, y, _, p) = self.frame.coordinates(
                    &beam_res_vec,
                    &recoil_res_vec,
                    &daughter_res_vec,
                    event,
                );
                let big_phi = y.dot(&event.eps).fatan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos(),
                    p.theta().fpowi(2),
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
        let (costheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let pol_angle = event.eps[0].facos();
        let r_big_phi = pol_angle * F::f(0.017453293) + big_phi;
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
                    + F::f(0.5) * (F::THREE * rho_000 - F::ONE) * costheta * costheta
                    - F::SQRT_2() * rho_100 * sin2theta * F::fcos(phi)
                    - rho_1n10 * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fcos(F::TWO * r_big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * costheta * costheta
                        - F::SQRT_2() * rho_101 * sin2theta * F::fcos(phi)
                        - rho_1n11 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fsin(F::TWO * r_big_phi)
                    * (F::SQRT_2() * rho_102 * sin2theta * F::fsin(phi)
                        + rho_1n12 * sinsqtheta * F::fsin(F::TWO * phi)),
        ))
        .into())
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
    frame: Frame,
    data: Vec<(F, F, F, F, F, F)>,
}

impl<F: Field> ThreePiSDME<F> {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            data: Vec::default(),
        }
    }
}

impl<F: Field> Node<F> for ThreePiSDME<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let resonance =
                    event.daughter_p4s[0] + event.daughter_p4s[1] + event.daughter_p4s[2];
                let daughter_res_vec = event.daughter_p4s[0].boost_along(&resonance).momentum();
                let beam_res_vec = event.beam_p4.boost_along(&resonance).momentum();
                let recoil_res_vec = event.recoil_p4.boost_along(&resonance).momentum();
                let (_, y, _, p) = self.frame.coordinates(
                    &beam_res_vec,
                    &recoil_res_vec,
                    &daughter_res_vec,
                    event,
                );

                let big_phi = y.dot(&event.eps).fatan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos(),
                    p.theta().fpowi(2),
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
        let (costheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let pol_angle = event.eps[0].facos();
        let r_big_phi = pol_angle * F::f(0.017453293) + big_phi;
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
                    + F::f(0.5) * (F::THREE * rho_000 - F::ONE) * costheta * costheta
                    - F::SQRT_2() * rho_100 * sin2theta * F::fcos(phi)
                    - rho_1n10 * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fcos(F::TWO * r_big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * costheta * costheta
                        - F::SQRT_2() * rho_101 * sin2theta * F::fcos(phi)
                        - rho_1n11 * sinsqtheta * F::fcos(F::TWO * phi))
                - pgamma
                    * F::fsin(F::TWO * r_big_phi)
                    * (F::SQRT_2() * rho_102 * sin2theta * F::fsin(phi)
                        + rho_1n12 * sinsqtheta * F::fsin(F::TWO * phi)),
        ))
        .into())
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
