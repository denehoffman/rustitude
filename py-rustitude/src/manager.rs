use crate::{amplitude::Model, dataset::Dataset};
use pyo3::prelude::*;
use rustitude_core::manager as rust;

#[pyclass]
#[derive(Clone)]
pub struct Manager(rust::Manager);

impl From<rust::Manager> for Manager {
    fn from(manager: rust::Manager) -> Self {
        Manager(manager)
    }
}
impl From<Manager> for rust::Manager {
    fn from(manager: Manager) -> Self {
        manager.0
    }
}

#[pymethods]
impl Manager {
    #[new]
    pub fn new(model: Model, dataset: Dataset) -> PyResult<Self> {
        rust::Manager::new(
            &rustitude_core::amplitude::Model::from(model),
            &rustitude_core::dataset::Dataset::from(dataset),
        )
        .map(Manager::from)
        .map_err(PyErr::from)
    }
    #[pyo3(name = "__call__")]
    fn evaluate(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.evaluate(&parameters).map_err(PyErr::from)
    }
}

#[pyclass]
pub struct ExtendedLogLikelihood(rust::ExtendedLogLikelihood);

impl From<rust::ExtendedLogLikelihood> for ExtendedLogLikelihood {
    fn from(ell: rust::ExtendedLogLikelihood) -> Self {
        ExtendedLogLikelihood(ell)
    }
}
impl From<ExtendedLogLikelihood> for rust::ExtendedLogLikelihood {
    fn from(ell: ExtendedLogLikelihood) -> Self {
        ell.0
    }
}

#[pymethods]
impl ExtendedLogLikelihood {
    #[new]
    pub fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        rust::ExtendedLogLikelihood::new(data_manager.into(), mc_manager.into()).into()
    }
    #[pyo3(name = "__call__", signature = (parameters, *, num_threads = 1))]
    fn evaluate(&self, parameters: Vec<f64>, num_threads: usize) -> PyResult<f64> {
        self.0
            .evaluate(parameters, num_threads)
            .map_err(PyErr::from)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager>()?;
    m.add_class::<ExtendedLogLikelihood>()?;
    Ok(())
}
