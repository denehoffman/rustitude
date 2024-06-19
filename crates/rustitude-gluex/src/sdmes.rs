use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::SHCoordinates;
use std::f64::consts::PI;

use crate::utils::Frame;

#[derive(Clone)]
pub struct TwoPiSDME {
    frame: Frame,
    data: Vec<(f64, f64, f64, f64, f64, f64)>,
}

impl TwoPiSDME {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            data: Vec::default(),
        }
    }
}

impl Node for TwoPiSDME {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .read()
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
                let big_phi = y.dot(&event.eps).atan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos(),
                    p.theta().powi(2),
                    f64::sin(2.0 * p.theta()),
                    p.phi(),
                    big_phi,
                    pgamma,
                )
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let (costheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let pol_angle = event.eps[0].acos();
        let r_big_phi = pol_angle * 0.017453293 + big_phi;
        let rho_000 = parameters[0];
        let rho_100 = parameters[1];
        let rho_1n10 = parameters[2];
        let rho_111 = parameters[3];
        let rho_001 = parameters[4];
        let rho_101 = parameters[5];
        let rho_1n11 = parameters[6];
        let rho_102 = parameters[7];
        let rho_1n12 = parameters[8];

        Ok(f64::sqrt(f64::abs(
            (3.0 / (4.0 * PI))
                * (0.5 * (1.0 - rho_000) + 0.5 * (3.0 * rho_000 - 1.0) * costheta * costheta
                    - f64::sqrt(2.0) * rho_100 * sin2theta * f64::cos(phi)
                    - rho_1n10 * f64::cos(2.0 * phi))
                - pgamma
                    * f64::cos(2.0 * r_big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * costheta * costheta
                        - f64::sqrt(2.0) * rho_101 * sin2theta * f64::cos(phi)
                        - rho_1n11 * sinsqtheta * f64::cos(2.0 * phi))
                - pgamma
                    * f64::sin(2.0 * r_big_phi)
                    * (f64::sqrt(2.0) * rho_102 * sin2theta * f64::sin(phi)
                        + rho_1n12 * sinsqtheta * f64::sin(2.0 * phi)),
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
pub struct ThreePiSDME {
    frame: Frame,
    data: Vec<(f64, f64, f64, f64, f64, f64)>,
}

impl ThreePiSDME {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            data: Vec::default(),
        }
    }
}

impl Node for ThreePiSDME {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .read()
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

                let big_phi = y.dot(&event.eps).atan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                (
                    p.theta_cos(),
                    p.theta().powi(2),
                    f64::sin(2.0 * p.theta()),
                    p.phi(),
                    big_phi,
                    pgamma,
                )
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let (costheta, sinsqtheta, sin2theta, phi, big_phi, pgamma) = self.data[event.index];
        let pol_angle = event.eps[0].acos();
        let r_big_phi = pol_angle * 0.017453293 + big_phi;
        let rho_000 = parameters[0];
        let rho_100 = parameters[1];
        let rho_1n10 = parameters[2];
        let rho_111 = parameters[3];
        let rho_001 = parameters[4];
        let rho_101 = parameters[5];
        let rho_1n11 = parameters[6];
        let rho_102 = parameters[7];
        let rho_1n12 = parameters[8];

        Ok(f64::sqrt(f64::abs(
            (3.0 / (4.0 * PI))
                * (0.5 * (1.0 - rho_000) + 0.5 * (3.0 * rho_000 - 1.0) * costheta * costheta
                    - f64::sqrt(2.0) * rho_100 * sin2theta * f64::cos(phi)
                    - rho_1n10 * f64::cos(2.0 * phi))
                - pgamma
                    * f64::cos(2.0 * r_big_phi)
                    * (rho_111 * sinsqtheta + rho_001 * costheta * costheta
                        - f64::sqrt(2.0) * rho_101 * sin2theta * f64::cos(phi)
                        - rho_1n11 * sinsqtheta * f64::cos(2.0 * phi))
                - pgamma
                    * f64::sin(2.0 * r_big_phi)
                    * (f64::sqrt(2.0) * rho_102 * sin2theta * f64::sin(phi)
                        + rho_1n12 * sinsqtheta * f64::sin(2.0 * phi)),
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
