use pyo3::prelude::*;
use rustitude_core::manager as rust;

use crate::{
    amplitude::{Amplitude, CohSum, Model, Parameter},
    dataset::Dataset,
};

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
    #[getter]
    fn cohsums(&self) -> Vec<CohSum> {
        self.0
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(CohSum::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude> {
        self.0
            .model
            .amplitudes
            .clone()
            .into_iter()
            .map(Amplitude::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter> {
        self.0
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter::from)
            .collect()
    }
    #[getter]
    fn bounds(&self) -> Vec<(f64, f64)> {
        self.0.get_bounds()
    }
    #[getter]
    fn initial(&self) -> Vec<f64> {
        self.0.get_initial()
    }
    #[getter]
    fn n_free(&self) -> usize {
        self.0.get_n_free()
    }
    #[new]
    fn new(model: Model, dataset: Dataset) -> PyResult<Self> {
        rust::Manager::new(
            &rustitude_core::amplitude::Model::from(model),
            &rustitude_core::dataset::Dataset::from(dataset),
        )
        .map(Manager::from)
        .map_err(PyErr::from)
    }
    #[pyo3(name = "__call__")]
    fn call(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.par_evaluate(&parameters).map_err(PyErr::from)
    }
    fn evaluate(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.evaluate(&parameters).map_err(PyErr::from)
    }
    fn par_evaluate(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.par_evaluate(&parameters).map_err(PyErr::from)
    }
    fn norm_int(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.norm_int(&parameters).map_err(PyErr::from)
    }
    fn par_norm_int(&self, parameters: Vec<f64>) -> PyResult<Vec<f64>> {
        self.0.par_norm_int(&parameters).map_err(PyErr::from)
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter::from)
            .map_err(PyErr::from)
    }
    fn print_parameters(&self) {
        self.0.print_parameters()
    }
    fn constrain(
        &mut self,
        amplitude_1: &str,
        parameter_1: &str,
        amplitude_2: &str,
        parameter_2: &str,
    ) -> PyResult<()> {
        self.0
            .constrain(amplitude_1, parameter_1, amplitude_2, parameter_2)
            .map_err(PyErr::from)
    }
    fn fix(&mut self, amplitude: &str, parameter: &str, value: f64) -> PyResult<()> {
        self.0.fix(amplitude, parameter, value).map_err(PyErr::from)
    }
    fn free(&mut self, amplitude: &str, parameter: &str) -> PyResult<()> {
        self.0.free(amplitude, parameter).map_err(PyErr::from)
    }
    fn set_bounds(&mut self, amplitude: &str, parameter: &str, bounds: (f64, f64)) -> PyResult<()> {
        self.0
            .set_bounds(amplitude, parameter, bounds)
            .map_err(PyErr::from)
    }
    fn set_initial(&mut self, amplitude: &str, parameter: &str, value: f64) -> PyResult<()> {
        self.0
            .set_initial(amplitude, parameter, value)
            .map_err(PyErr::from)
    }
    fn activate(&mut self, amplitude: &str) {
        self.0.activate(amplitude)
    }
    fn deactivate(&mut self, amplitude: &str) {
        self.0.deactivate(amplitude)
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
    #[getter]
    fn cohsums(&self) -> Vec<CohSum> {
        self.0
            .data_manager
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(CohSum::from)
            .collect()
    }

    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude> {
        self.0
            .data_manager
            .model
            .amplitudes
            .clone()
            .into_iter()
            .map(Amplitude::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter> {
        self.0
            .data_manager
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter::from)
            .collect()
    }
    #[getter]
    fn bounds(&self) -> Vec<(f64, f64)> {
        self.0.get_bounds()
    }
    #[getter]
    fn initial(&self) -> Vec<f64> {
        self.0.get_initial()
    }
    #[getter]
    fn n_free(&self) -> usize {
        self.0.get_n_free()
    }
    #[new]
    fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        rust::ExtendedLogLikelihood::new(data_manager.into(), mc_manager.into()).into()
    }
    #[pyo3(signature = (parameters, *, weighted = true))]
    fn norm_int(&self, parameters: Vec<f64>, weighted: bool) -> PyResult<f64> {
        self.0.norm_int(&parameters, weighted).map_err(PyErr::from)
    }
    #[pyo3(signature = (parameters, *, num_threads = 1, weighted = true))]
    fn par_norm_int(
        &self,
        parameters: Vec<f64>,
        num_threads: usize,
        weighted: bool,
    ) -> PyResult<f64> {
        self.0
            .par_norm_int(&parameters, num_threads, weighted)
            .map_err(PyErr::from)
    }
    fn evaluate(&self, parameters: Vec<f64>) -> PyResult<f64> {
        self.0.evaluate(&parameters).map_err(PyErr::from)
    }
    #[pyo3(signature = (parameters, *, num_threads = 1))]
    fn par_evaluate(&self, parameters: Vec<f64>, num_threads: usize) -> PyResult<f64> {
        self.0
            .par_evaluate(&parameters, num_threads)
            .map_err(PyErr::from)
    }
    #[pyo3(name = "__call__", signature = (parameters, *, num_threads = 1))]
    fn call(&self, parameters: Vec<f64>, num_threads: usize) -> PyResult<f64> {
        self.0
            .par_evaluate(&parameters, num_threads)
            .map_err(PyErr::from)
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter::from)
            .map_err(PyErr::from)
    }
    fn print_parameters(&self) {
        self.0.print_parameters()
    }
    fn constrain(
        &mut self,
        amplitude_1: &str,
        parameter_1: &str,
        amplitude_2: &str,
        parameter_2: &str,
    ) -> PyResult<()> {
        self.0
            .constrain(amplitude_1, parameter_1, amplitude_2, parameter_2)
            .map_err(PyErr::from)
    }
    fn fix(&mut self, amplitude: &str, parameter: &str, value: f64) -> PyResult<()> {
        self.0.fix(amplitude, parameter, value).map_err(PyErr::from)
    }
    fn free(&mut self, amplitude: &str, parameter: &str) -> PyResult<()> {
        self.0.free(amplitude, parameter).map_err(PyErr::from)
    }
    fn set_bounds(&mut self, amplitude: &str, parameter: &str, bounds: (f64, f64)) -> PyResult<()> {
        self.0
            .set_bounds(amplitude, parameter, bounds)
            .map_err(PyErr::from)
    }
    fn set_initial(&mut self, amplitude: &str, parameter: &str, value: f64) -> PyResult<()> {
        self.0
            .set_initial(amplitude, parameter, value)
            .map_err(PyErr::from)
    }
    fn activate(&mut self, amplitude: &str) {
        self.0.activate(amplitude)
    }
    fn deactivate(&mut self, amplitude: &str) {
        self.0.deactivate(amplitude)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager>()?;
    m.add_class::<ExtendedLogLikelihood>()?;
    Ok(())
}
