use rayon::prelude::*;

use crate::{
    create_pool,
    errors::RustitudeError,
    prelude::{Dataset, Event, Model},
};

#[derive(Clone)]
pub struct Manager {
    model: Model,
    dataset: Dataset,
}
impl Manager {
    pub fn new(model: &Model, dataset: &Dataset) -> Result<Self, RustitudeError> {
        let mut model = model.clone();
        model.load(dataset)?;
        Ok(Self {
            model: model.clone(),
            dataset: dataset.clone(),
        })
    }
    pub fn evaluate(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        self.dataset
            .events
            .read()
            .par_iter()
            .map(|event: &Event| self.model.compute(parameters, event))
            .collect()
    }
}

pub struct ExtendedLogLikelihood {
    data_manager: Manager,
    mc_manager: Manager,
}
impl ExtendedLogLikelihood {
    pub const fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        Self {
            data_manager,
            mc_manager,
        }
    }
    #[allow(clippy::suboptimal_flops)]
    pub fn evaluate(
        &self,
        parameters: Vec<f64>,
        num_threads: usize,
    ) -> Result<f64, RustitudeError> {
        create_pool(num_threads)?.install(|| {
            let data_res = self.data_manager.evaluate(&parameters)?;
            let data_weights = self.data_manager.dataset.weights();
            let n_data = self.data_manager.dataset.len() as f64;
            let mc_res = self.mc_manager.evaluate(&parameters)?;
            let mc_weights = self.mc_manager.dataset.weights();
            let n_mc = self.mc_manager.dataset.len() as f64;
            let ln_l = (data_res
                .iter()
                .zip(data_weights)
                .map(|(l, w)| w * l.ln())
                .sum::<f64>())
                - (n_data / n_mc)
                    * (mc_res
                        .iter()
                        .zip(mc_weights)
                        .map(|(l, w)| w * l)
                        .sum::<f64>());
            Ok(-2.0 * ln_l)
        })
    }
}
