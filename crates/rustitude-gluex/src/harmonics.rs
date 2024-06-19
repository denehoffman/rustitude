use std::f64::consts::PI;

use num_complex::ComplexFloat;
use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::{ComplexSH, SHEval};

use crate::utils::{Frame, Reflectivity, Wave};

#[derive(Clone)]
pub struct Ylm {
    wave: Wave,
    frame: Frame,
    data: Vec<Complex64>,
}
impl Ylm {
    pub fn new(wave: Wave, frame: Frame) -> Self {
        Self {
            wave,
            frame,
            data: Vec::default(),
        }
    }
}
impl Node for Ylm {
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
                let (_, _, _, p) = self.frame.coordinates(
                    &beam_res_vec,
                    &recoil_res_vec,
                    &daughter_res_vec,
                    event,
                );
                ComplexSH::Spherical.eval(self.wave.l(), self.wave.m(), &p)
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct Zlm {
    wave: Wave,
    reflectivity: Reflectivity,
    frame: Frame,
    data: Vec<Complex64>,
}
impl Zlm {
    pub fn new(wave: Wave, reflectivity: Reflectivity, frame: Frame) -> Self {
        Self {
            wave,
            reflectivity,
            frame,
            data: Vec::default(),
        }
    }
}
impl Node for Zlm {
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
                let ylm = ComplexSH::Spherical.eval(self.wave.l(), self.wave.m(), &p);
                let big_phi = f64::atan2(
                    y.dot(&event.eps),
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                let phase = Complex64::cis(-big_phi);
                let zlm = ylm * phase;
                match self.reflectivity {
                    Reflectivity::Positive => Complex64::new(
                        f64::sqrt(1.0 + pgamma) * zlm.re,
                        f64::sqrt(1.0 - pgamma) * zlm.im,
                    ),
                    Reflectivity::Negative => Complex64::new(
                        f64::sqrt(1.0 - pgamma) * zlm.re,
                        f64::sqrt(1.0 + pgamma) * zlm.im,
                    ),
                }
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, _parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct OnePS {
    reflectivity: Reflectivity,
    frame: Frame,
    data: Vec<Complex64>,
}
impl OnePS {
    pub fn new(reflectivity: Reflectivity, frame: Frame) -> Self {
        Self {
            reflectivity,
            frame,
            data: Vec::default(),
        }
    }
}
impl Node for OnePS {
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
                let (_, y, _, _) = self.frame.coordinates(
                    &beam_res_vec,
                    &recoil_res_vec,
                    &daughter_res_vec,
                    event,
                );
                let pol_angle = event.eps[0].acos();
                let big_phi = y.dot(&event.eps).atan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                let phase = Complex64::cis(-(pol_angle + big_phi));
                match self.reflectivity {
                    Reflectivity::Positive => Complex64::new(
                        f64::sqrt(1.0 + pgamma) * phase.re,
                        f64::sqrt(1.0 - pgamma) * phase.im,
                    ),
                    Reflectivity::Negative => Complex64::new(
                        f64::sqrt(1.0 - pgamma) * phase.re,
                        f64::sqrt(1.0 + pgamma) * phase.im,
                    ),
                }
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct TwoPS {
    wave: Wave,
    reflectivity: Reflectivity,
    frame: Frame,
    data: Vec<Complex64>,
}
impl TwoPS {
    pub fn new(wave: Wave, reflectivity: Reflectivity, frame: Frame) -> Self {
        Self {
            wave,
            reflectivity,
            frame,
            data: Vec::default(),
        }
    }
}
impl Node for TwoPS {
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
                let (_, _, _, p) = self.frame.coordinates(
                    &beam_res_vec,
                    &recoil_res_vec,
                    &daughter_res_vec,
                    event,
                );
                let ylm_p = ComplexSH::Spherical
                    .eval(self.wave.l(), self.wave.m(), &p)
                    .conj();
                let ylm_m = ComplexSH::Spherical
                    .eval(self.wave.l(), -self.wave.m(), &p)
                    .conj();
                let m_refl = (if self.wave.m() % 2 == 0 {
                    self.reflectivity as isize
                } else {
                    -(self.reflectivity as isize)
                }) as f64;
                let big_theta = match self.wave.m().cmp(&0) {
                    std::cmp::Ordering::Less => 0.0,
                    std::cmp::Ordering::Equal => 0.5,
                    std::cmp::Ordering::Greater => f64::sqrt(0.5),
                };
                let wigner_d_lm0_m =
                    f64::sqrt(4.0 * PI / (2.0 * self.wave.l() as f64 + 1.0)) * ylm_m;
                big_theta * ylm_p - m_refl * wigner_d_lm0_m
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(self.data[event.index])
    }
}
