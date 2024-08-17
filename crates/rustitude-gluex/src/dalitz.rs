use rayon::prelude::*;
use rustitude_core::{convert, prelude::*};

use crate::utils::Decay;

#[derive(Default, Clone)]
pub struct OmegaDalitz<F: Field> {
    decay: Decay,
    dalitz_z: Vec<F>,
    dalitz_sin3theta: Vec<F>,
    lambda: Vec<F>,
}

impl<F: Field> OmegaDalitz<F> {
    pub fn new(decay: Decay) -> Self {
        match decay {
            Decay::ThreeBodyDecay(_) => Self {
                decay,
                ..Default::default()
            },
            _ => unimplemented!(),
        }
    }
}

impl<F: Field> Node<F> for OmegaDalitz<F> {
    fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
        (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
            .events
            .par_iter()
            .map(|event| {
                let pi0 = self.decay.primary_p4(event);
                let pip = self.decay.secondary_p4(event);
                let pim = self.decay.tertiary_p4(event);
                let omega = pi0 + pip + *pim;

                let dalitz_s = (pip + pim).m2();
                let dalitz_t = (pip + pi0).m2();
                let dalitz_u = (pim + pi0).m2();

                let m3pi = (convert!(2.0, F) * pip.m()) + pi0.m();
                let dalitz_d = convert!(2.0, F) * omega.m() * (omega.m() - m3pi);
                let dalitz_sc =
                    (F::one() / convert!(3.0, F)) * (omega.m2() + pip.m2() + pim.m2() + pi0.m2());
                let dalitz_x = F::sqrt(convert!(3.0, F)) * (dalitz_t - dalitz_u) / dalitz_d;
                let dalitz_y = convert!(3.0, F) * (dalitz_sc - dalitz_s) / dalitz_d;

                let dalitz_z = dalitz_x * dalitz_x + dalitz_y * dalitz_y;
                let dalitz_sin3theta =
                    F::sin(convert!(3.0, F) * F::asin(dalitz_y / F::sqrt(dalitz_z)));

                let pip_omega = pip.boost_along(&omega);
                let pim_omega = pim.boost_along(&omega);
                let pi_cross = pip_omega.momentum().cross(&pim_omega.momentum());

                let lambda = (convert!(4.0, F) / convert!(3.0, F))
                    * F::abs(pi_cross.dot(&pi_cross))
                    / ((F::one() / convert!(9.0, F))
                        * (omega.m2() - (convert!(2.0, F) * pip.m() + pi0.m()).powi(2)).powi(2));

                (dalitz_z, (dalitz_sin3theta, lambda))
            })
            .unzip();
        Ok(())
    }

    fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
        let dalitz_z = self.dalitz_z[event.index];
        let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
        let lambda = self.lambda[event.index];
        let alpha = parameters[0];
        let beta = parameters[1];
        let gamma = parameters[2];
        let delta = parameters[3];
        Ok(F::sqrt(F::abs(
            lambda
                * (F::one()
                    + convert!(2.0, F) * alpha * dalitz_z
                    + convert!(2.0, F)
                        * beta
                        * dalitz_z.powf(convert!(3.0, F) / convert!(2.0, F))
                        * dalitz_sin3theta
                    + convert!(2.0, F) * gamma * dalitz_z.powi(2)
                    + convert!(2.0, F)
                        * delta
                        * dalitz_z.powf(convert!(5.0, F) / convert!(2.0, F))
                        * dalitz_sin3theta),
        ))
        .into())
    }

    fn parameters(&self) -> Vec<String> {
        vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "delta".to_string(),
        ]
    }
}
