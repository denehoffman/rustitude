use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::{ComplexSH, SHEval};

use crate::utils::{Decay, Frame, Sign, Wave};

#[derive(Clone)]
pub struct Ylm<F: Field> {
    wave: Wave,
    decay: Decay,
    frame: Frame,
    data: Vec<Complex<F>>,
}
impl<F: Field> Ylm<F> {
    pub fn new(wave: Wave, decay: Decay, frame: Frame) -> Self {
        Self {
            wave,
            decay,
            frame,
            data: Vec::default(),
        }
    }
}
impl<F: Field> Node<F> for Ylm<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, _, _, p) =
                    self.frame
                        .coordinates(self.decay, self.decay.primary_p4(event), event);
                ComplexSH::Spherical.eval(self.wave.l(), self.wave.m(), &p)
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct Zlm<F: Field> {
    wave: Wave,
    reflectivity: Sign,
    decay: Decay,
    frame: Frame,
    data: Vec<Complex<F>>,
}
impl<F: Field> Zlm<F> {
    pub fn new(wave: Wave, reflectivity: Sign, decay: Decay, frame: Frame) -> Self {
        Self {
            wave,
            reflectivity,
            decay,
            frame,
            data: Vec::default(),
        }
    }
}
impl<F: Field + num::Float> Node<F> for Zlm<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, y, _, p) = self.decay.coordinates(self.frame, 0, event);
                let ylm = ComplexSH::Spherical.eval(self.wave.l(), self.wave.m(), &p);
                let big_phi = F::fatan2(
                    y.dot(&event.eps),
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                let phase = Complex::cis(-big_phi);
                let zlm = ylm * phase;
                match self.reflectivity {
                    Sign::Positive => Complex::new(
                        F::fsqrt(F::ONE + pgamma) * zlm.re,
                        F::fsqrt(F::ONE - pgamma) * zlm.im,
                    ),
                    Sign::Negative => Complex::new(
                        F::fsqrt(F::ONE - pgamma) * zlm.re,
                        F::fsqrt(F::ONE + pgamma) * zlm.im,
                    ),
                }
            })
            .collect();
        Ok(())
    }
    fn calculate(&self, _parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct OnePS<F: Field> {
    reflectivity: Sign,
    decay: Decay,
    frame: Frame,
    data: Vec<Complex<F>>,
}
impl<F: Field> OnePS<F> {
    pub fn new(reflectivity: Sign, decay: Decay, frame: Frame) -> Self {
        Self {
            reflectivity,
            decay,
            frame,
            data: Vec::default(),
        }
    }
}
impl<F: Field> Node<F> for OnePS<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, y, _, _) = self.decay.coordinates(self.frame, 0, event);
                let pol_angle = event.eps[0].facos();
                let big_phi = y.dot(&event.eps).fatan2(
                    event
                        .beam_p4
                        .momentum()
                        .normalize()
                        .dot(&event.eps.cross(&y)),
                );
                let pgamma = event.eps.norm();
                let phase = Complex::cis(-(pol_angle + big_phi));
                match self.reflectivity {
                    Sign::Positive => Complex::new(
                        F::fsqrt(F::ONE + pgamma) * phase.re,
                        F::fsqrt(F::ONE - pgamma) * phase.im,
                    ),
                    Sign::Negative => Complex::new(
                        F::fsqrt(F::ONE - pgamma) * phase.re,
                        F::fsqrt(F::ONE + pgamma) * phase.im,
                    ),
                }
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        Ok(self.data[event.index])
    }
}

#[derive(Clone)]
pub struct TwoPS<F: Field> {
    wave: Wave,
    reflectivity: Sign,
    decay: Decay,
    frame: Frame,
    data: Vec<Complex<F>>,
}
impl<F: Field> TwoPS<F> {
    pub fn new(wave: Wave, reflectivity: Sign, decay: Decay, frame: Frame) -> Self {
        Self {
            wave,
            reflectivity,
            decay,
            frame,
            data: Vec::default(),
        }
    }
}
impl<F: Field> Node<F> for TwoPS<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let (_, _, _, p) = self.decay.coordinates(self.frame, 0, event);
                let ylm_p = ComplexSH::Spherical
                    .eval(self.wave.l(), self.wave.m(), &p)
                    .conj();
                let ylm_m = ComplexSH::Spherical
                    .eval(self.wave.l(), -self.wave.m(), &p)
                    .conj();
                let m_refl = F::convert_isize(if self.wave.m() % 2 == 0 {
                    self.reflectivity as isize
                } else {
                    -(self.reflectivity as isize)
                });
                let big_theta = match self.wave.m().cmp(&0) {
                    std::cmp::Ordering::Less => F::ZERO,
                    std::cmp::Ordering::Equal => F::f(0.5),
                    std::cmp::Ordering::Greater => F::fsqrt(F::f(0.5)),
                };
                let wigner_d_lm0_m = ylm_m.scale(F::fsqrt(
                    F::FOUR * F::PI()
                        / (F::TWO * <F as Field>::convert_usize(self.wave.l() as usize) + F::ONE),
                ));
                ylm_p.scale(big_theta) - wigner_d_lm0_m.scale(m_refl)
            })
            .collect();
        Ok(())
    }

    fn calculate(&self, _parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        Ok(self.data[event.index])
    }
}
