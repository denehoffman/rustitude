use crate::utils::{self, Decay, Frame, Sign};
use rayon::prelude::*;
use rustitude_core::prelude::*;
use sphrs::{ComplexSH, SHEval};

#[derive(Clone)]
pub struct ThreePiPolFrac<F> {
    beam_pol: F,
    j_resonance: u32,
    p_resonance: F,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: Decay,
    decay_isobar: Decay,
    data: Vec<Complex<F>>,
}

impl<F: Field> ThreePiPolFrac<F> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        beam_pol: Sign,
        j_resonance: u32,
        p_resonance: Sign,
        i_resonance: usize,
        l_resonance: u32,
        j_isobar: u32,
        i_isobar: usize,
        iz_daughters: [usize; 3],
        decay_resonance: Decay,
        decay_isobar: Decay,
    ) -> Self {
        match (decay_resonance, decay_isobar) {
            (Decay::ThreeBodyDecay(_), Decay::TwoBodyDecay(_)) => Self {
                beam_pol: match beam_pol {
                    Sign::Positive => F::ONE,
                    Sign::Negative => -F::ONE,
                },
                j_resonance,
                p_resonance: match p_resonance {
                    Sign::Positive => F::ONE,
                    Sign::Negative => -F::ONE,
                },
                i_resonance,
                l_resonance,
                j_isobar,
                i_isobar,
                iz_daughters,
                decay_resonance,
                decay_isobar,
                data: Vec::default(),
            },
            _ => unimplemented!(),
        }
    }
}

impl<F: Field> Node<F> for ThreePiPolFrac<F> {
    fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        Ok(self.data[event.index] * (F::ONE + self.beam_pol * parameters[0]) / F::FOUR)
    }

    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        self.data = dataset
            .events
            .par_iter()
            .map(|event| {
                let isobar_p4 = self.decay_isobar.resonance_p4(event);
                let resonance_p4 = self.decay_resonance.resonance_p4(event);
                let alpha = event.recoil_p4.phi();
                let (_, _, _, p3_res_coords) =
                    self.decay_resonance.coordinates(Frame::Helicity, 2, event);
                let p1_iso_p4 = self.decay_isobar.primary_p4(event).boost_along(&isobar_p4);
                let (_, _, _, p1_iso_coords) =
                    Frame::Helicity.coordinates(self.decay_resonance, &p1_iso_p4, event);
                let k = utils::breakup_momentum(
                    resonance_p4.m(),
                    isobar_p4.m(),
                    self.decay_resonance.tertiary_p4(event).m(),
                );
                let q = utils::breakup_momentum(
                    isobar_p4.m(),
                    self.decay_isobar.primary_p4(event).m(),
                    self.decay_isobar.secondary_p4(event).m(),
                );
                let neg_res_hel_prod =
                    Complex::new(F::fcos(F::TWO * alpha), F::fsin(F::TWO * alpha))
                        * self.beam_pol
                        * self.p_resonance
                        * (F::convert_u32((self.j_resonance % 2) * 2) / F::TWO);
                let mut res = F::ZERO.c();
                for m_res in -(self.l_resonance as isize)..(self.l_resonance as isize) {
                    let mut term = F::ZERO.c();
                    for m_iso in -(self.j_isobar as isize)..(self.j_isobar as isize) {
                        let ylm = ComplexSH::Spherical.eval(
                            self.j_isobar as i64,
                            m_iso as i64,
                            &p1_iso_coords,
                        );
                        let cg_neg = F::f(wigners::clebsch_gordan(
                            self.j_isobar,
                            self.l_resonance as i32,
                            m_iso as u32,
                            m_res as i32,
                            self.j_resonance,
                            -1,
                        ));
                        let cg_pos = F::f(wigners::clebsch_gordan(
                            self.j_isobar,
                            self.l_resonance as i32,
                            m_iso as u32,
                            m_res as i32,
                            self.j_resonance,
                            1,
                        ));
                        term += ylm * (neg_res_hel_prod * cg_neg + cg_pos);
                    }
                    let ylm = ComplexSH::Spherical.eval(
                        self.l_resonance as i64,
                        m_res as i64,
                        &p3_res_coords,
                    );
                    term *= ylm;
                    res += term;
                }
                res *= F::f(
                    wigners::clebsch_gordan(
                        1,
                        1,
                        self.iz_daughters[0] as u32,
                        self.iz_daughters[1] as i32,
                        self.i_isobar as u32,
                        (self.iz_daughters[0] + self.iz_daughters[1]) as i32,
                    ) * wigners::clebsch_gordan(
                        self.i_isobar as u32,
                        1,
                        (self.iz_daughters[0] + self.iz_daughters[1]) as u32,
                        self.iz_daughters[2] as i32,
                        self.i_resonance as u32,
                        (self.iz_daughters[0] + self.iz_daughters[1] + self.iz_daughters[2]) as i32,
                    ),
                ) * k.fpowi(self.l_resonance as i32)
                    * q.fpowi(self.j_isobar as i32);
                res
            })
            .collect();
        Ok(())
    }

    fn parameters(&self) -> Vec<String> {
        std::vec!["polarization fraction".to_string()]
    }
}
