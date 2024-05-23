use pyo3::prelude::*;
use rustitude_core::manager as rust;
use std::mem::transmute;

#[pyclass]
#[derive(Clone)]
pub struct Manager(rust::Manager);

#[pymethods]
impl Manager {
    #[pyo3(name = "__call__")]
    fn evaluate(&self, parameters: Vec<f64>) -> Vec<f64> {
        self.0.evaluate(&parameters)
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
    fn evaluate(&self, parameters: Vec<f64>, num_threads: usize) -> f64 {
        self.0.evaluate(parameters, num_threads)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager>()?;
    m.add_class::<ExtendedLogLikelihood>()?;
    Ok(())
}
