use pyo3::prelude::*;
use rayon::prelude::*;

use crate::{
    create_pool,
    prelude::{Dataset, Event, Model},
};

#[pyclass]
#[derive(Clone)]
pub struct Manager {
    model: Model,
    dataset: Dataset,
}
#[pymethods]
impl Manager {
    #[new]
    pub fn new(model: Model, dataset: Dataset) -> Self {
        let mut model = model;
        model.load(&dataset);
        Self { model, dataset }
    }
    #[pyo3(name = "__call__")]
    fn pyevaluate(&self, parameters: Vec<f64>) -> Vec<f64> {
        self.evaluate(&parameters)
    }
}
impl Manager {
    pub fn evaluate(&self, parameters: &[f64]) -> Vec<f64> {
        self.dataset
            .events
            .read()
            .par_iter()
            .map(|event: &Event| self.model.compute(parameters, event))
            .collect()
    }
}

#[pyclass]
pub struct ExtendedLogLikelihood {
    data_manager: Manager,
    mc_manager: Manager,
}
#[pymethods]
impl ExtendedLogLikelihood {
    #[new]
    pub const fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        Self {
            data_manager,
            mc_manager,
        }
    }
    #[pyo3(name = "__call__", signature = (parameters, *, num_threads = 1))]
    #[allow(clippy::suboptimal_flops)]
    pub fn evaluate(&self, parameters: Vec<f64>, num_threads: usize) -> f64 {
        create_pool(num_threads).unwrap().install(|| {
            let data_res = self.data_manager.evaluate(&parameters);
            let data_weights = self.data_manager.dataset.weights();
            let n_data = self.data_manager.dataset.len() as f64;
            let mc_res = self.mc_manager.evaluate(&parameters);
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
            -2.0 * ln_l
        })
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager>()?;
    m.add_class::<ExtendedLogLikelihood>()?;
    Ok(())
}
