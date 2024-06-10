use rayon::prelude::*;
use rustitude_core::prelude::*;

#[derive(Default, Clone)]
pub struct OmegaDalitz {
    dalitz_z: Vec<f64>,
    dalitz_sin3theta: Vec<f64>,
    lambda: Vec<f64>,
}

impl Node for OmegaDalitz {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let pi0 = event.daughter_p4s[0];
                let pip = event.daughter_p4s[1];
                let pim = event.daughter_p4s[2];
                let omega = pi0 + pip + pim;

                let dalitz_s = (pip + pim).m2();
                let dalitz_t = (pip + pi0).m2();
                let dalitz_u = (pim + pi0).m2();

                let m3pi = (2.0 * pip.m()) + pi0.m();
                let dalitz_d = 2.0 * omega.m() * (omega.m() - m3pi);
                let dalitz_sc = (1.0 / 3.0) * (omega.m2() + pip.m2() + pim.m2() + pi0.m2());
                let dalitz_x = f64::sqrt(3.0) * (dalitz_t - dalitz_u) / dalitz_d;
                let dalitz_y = 3.0 * (dalitz_sc - dalitz_s) / dalitz_d;

                let dalitz_z = dalitz_x * dalitz_x + dalitz_y * dalitz_y;
                let dalitz_sin3theta = f64::sin(3.0 * f64::asin(dalitz_y / f64::sqrt(dalitz_z)));

                let pip_omega = pip.boost_along(&omega);
                let pim_omega = pim.boost_along(&omega);
                let pi_cross = pip_omega.momentum().cross(&pim_omega.momentum());

                let lambda = (4.0 / 3.0) * f64::abs(pi_cross.dot(&pi_cross))
                    / ((1.0 / 9.0) * (omega.m2() - (2.0 * pip.m() + pi0.m()).powi(2)).powi(2));

                (dalitz_z, (dalitz_sin3theta, lambda))
            })
            .unzip();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let dalitz_z = self.dalitz_z[event.index];
        let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
        let lambda = self.lambda[event.index];
        let alpha = parameters[0];
        let beta = parameters[1];
        let gamma = parameters[2];
        let delta = parameters[3];
        Ok(f64::sqrt(f64::abs(
            lambda
                * (1.0
                    + 2.0 * alpha * dalitz_z
                    + 2.0 * beta * dalitz_z.powf(3.0 / 2.0) * dalitz_sin3theta
                    + 2.0 * gamma * dalitz_z.powi(2)
                    + 2.0 * delta * dalitz_z.powf(5.0 / 2.0) * dalitz_sin3theta),
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
