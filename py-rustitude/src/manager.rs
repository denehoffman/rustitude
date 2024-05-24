use crate::{amplitude::Model, dataset::Dataset};
use pyo3::prelude::*;
use rustitude_core::manager as rust;
use std::mem::transmute;

#[pyclass]
#[derive(Clone)]
pub struct Manager(rust::Manager);

#[pymethods]
impl Manager {
    #[new]
    pub fn new(model: Model, dataset: Dataset) -> PyResult<Self> {
        unsafe {
            transmute(Self(rust::Manager::new(
                &transmute(model),
                transmute(dataset),
            )?))
        }
    }
    #[pyo3(name = "__call__")]
    fn evaluate(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.evaluate(&parameters).map_err(PyErr::from)
    }
}

#[pyclass]
pub struct ExtendedLogLikelihood(rust::ExtendedLogLikelihood);

#[pymethods]
impl ExtendedLogLikelihood {
    #[new]
    pub fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        unsafe {
            Self(rust::ExtendedLogLikelihood::new(
                transmute(data_manager),
                transmute(mc_manager),
            ))
        }
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
