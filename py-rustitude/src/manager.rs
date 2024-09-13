use nalgebra::Storage;
use pyo3::{
    exceptions::{PyException, PyRuntimeError},
    prelude::*,
};
use rustitude_core as rust;

use crate::{
    amplitude::{
        Amplitude_32, Amplitude_64, Model_32, Model_64, NormSqr_32, NormSqr_64, Parameter_32,
        Parameter_64,
    },
    dataset::{Dataset_32, Dataset_64},
    impl_convert,
};

#[pyclass]
#[derive(Clone)]
pub struct Manager_64(rust::manager::Manager<f64>);
impl_convert!(Manager_64, rust::manager::Manager<f64>);

#[pymethods]
impl Manager_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_64> {
        self.0
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(NormSqr_64::from)
            .collect()
    }
    #[getter]
    fn model(&self) -> Model_64 {
        self.0.model.clone().into()
    }
    #[getter]
    fn dataset(&self) -> Dataset_64 {
        self.0.dataset.clone().into()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_64> {
        self.0
            .model
            .amplitudes
            .read()
            .clone()
            .into_iter()
            .map(Amplitude_64::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter_64> {
        self.0
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter_64::from)
            .collect()
    }
    #[getter]
    fn free_parameters(&self) -> Vec<Parameter_64> {
        self.0
            .model
            .free_parameters()
            .into_iter()
            .map(Parameter_64::from)
            .collect()
    }
    #[getter]
    fn fixed_parameters(&self) -> Vec<Parameter_64> {
        self.0
            .model
            .fixed_parameters()
            .into_iter()
            .map(Parameter_64::from)
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
    fn new(model: Model_64, dataset: Dataset_64) -> PyResult<Self> {
        rust::manager::Manager::new(
            &rustitude_core::amplitude::Model::from(model),
            &rustitude_core::dataset::Dataset::from(dataset),
        )
        .map(Manager_64::from)
        .map_err(PyErr::from)
    }
    #[pyo3(name = "__call__", signature = (parameters, *, indices = None, parallel = true))]
    fn call(
        &self,
        parameters: Vec<f64>,
        indices: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f64>> {
        self.evaluate(parameters, indices, parallel)
    }
    #[pyo3(signature = (parameters, *, indices = None, parallel = true))]
    fn evaluate(
        &self,
        parameters: Vec<f64>,
        indices: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f64>> {
        if parallel {
            if self.0.model.contains_python_amplitudes {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            if let Some(inds) = indices {
                self.0
                    .par_evaluate_indexed(&parameters, &inds)
                    .map_err(PyErr::from)
            } else {
                self.0.par_evaluate(&parameters).map_err(PyErr::from)
            }
        } else if let Some(inds) = indices {
            self.0
                .evaluate_indexed(&parameters, &inds)
                .map_err(PyErr::from)
        } else {
            self.0.evaluate(&parameters).map_err(PyErr::from)
        }
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude_64> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude_64::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter_64> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter_64::from)
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
    fn activate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.activate(amplitude).map_err(PyErr::from)
    }
    fn activate_all(&mut self) {
        self.0.activate_all()
    }
    fn isolate(&mut self, amplitudes: Vec<String>) -> PyResult<()> {
        self.0
            .isolate(amplitudes.iter().map(|s| s.as_ref()).collect())
            .map_err(PyErr::from)
    }
    fn deactivate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.deactivate(amplitude).map_err(PyErr::from)
    }
    fn deactivate_all(&mut self) {
        self.0.deactivate_all()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Manager_32(rust::manager::Manager<f32>);
impl_convert!(Manager_32, rust::manager::Manager<f32>);

#[pymethods]
impl Manager_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_32> {
        self.0
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(NormSqr_32::from)
            .collect()
    }
    #[getter]
    fn model(&self) -> Model_32 {
        self.0.model.clone().into()
    }
    #[getter]
    fn dataset(&self) -> Dataset_32 {
        self.0.dataset.clone().into()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_32> {
        self.0
            .model
            .amplitudes
            .read()
            .clone()
            .into_iter()
            .map(Amplitude_32::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter_32> {
        self.0
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn free_parameters(&self) -> Vec<Parameter_32> {
        self.0
            .model
            .free_parameters()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn fixed_parameters(&self) -> Vec<Parameter_32> {
        self.0
            .model
            .fixed_parameters()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn bounds(&self) -> Vec<(f32, f32)> {
        self.0.get_bounds()
    }
    #[getter]
    fn initial(&self) -> Vec<f32> {
        self.0.get_initial()
    }
    #[getter]
    fn n_free(&self) -> usize {
        self.0.get_n_free()
    }
    #[new]
    fn new(model: Model_32, dataset: Dataset_32) -> PyResult<Self> {
        rust::manager::Manager::new(
            &rustitude_core::amplitude::Model::from(model),
            &rustitude_core::dataset::Dataset::from(dataset),
        )
        .map(Manager_32::from)
        .map_err(PyErr::from)
    }
    #[pyo3(name = "__call__", signature = (parameters, *, indices = None, parallel = true))]
    fn call(
        &self,
        parameters: Vec<f32>,
        indices: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f32>> {
        self.evaluate(parameters, indices, parallel)
    }
    #[pyo3(signature = (parameters, *, indices = None, parallel = true))]
    fn evaluate(
        &self,
        parameters: Vec<f32>,
        indices: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f32>> {
        if parallel {
            if self.0.model.contains_python_amplitudes {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            if let Some(inds) = indices {
                self.0
                    .par_evaluate_indexed(&parameters, &inds)
                    .map_err(PyErr::from)
            } else {
                self.0.par_evaluate(&parameters).map_err(PyErr::from)
            }
        } else if let Some(inds) = indices {
            self.0
                .evaluate_indexed(&parameters, &inds)
                .map_err(PyErr::from)
        } else {
            self.0.evaluate(&parameters).map_err(PyErr::from)
        }
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude_32> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude_32::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter_32> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter_32::from)
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
    fn fix(&mut self, amplitude: &str, parameter: &str, value: f32) -> PyResult<()> {
        self.0.fix(amplitude, parameter, value).map_err(PyErr::from)
    }
    fn free(&mut self, amplitude: &str, parameter: &str) -> PyResult<()> {
        self.0.free(amplitude, parameter).map_err(PyErr::from)
    }
    fn set_bounds(&mut self, amplitude: &str, parameter: &str, bounds: (f32, f32)) -> PyResult<()> {
        self.0
            .set_bounds(amplitude, parameter, bounds)
            .map_err(PyErr::from)
    }
    fn set_initial(&mut self, amplitude: &str, parameter: &str, value: f32) -> PyResult<()> {
        self.0
            .set_initial(amplitude, parameter, value)
            .map_err(PyErr::from)
    }
    fn activate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.activate(amplitude).map_err(PyErr::from)
    }
    fn activate_all(&mut self) {
        self.0.activate_all()
    }
    fn isolate(&mut self, amplitudes: Vec<String>) -> PyResult<()> {
        self.0
            .isolate(amplitudes.iter().map(|s| s.as_ref()).collect())
            .map_err(PyErr::from)
    }
    fn deactivate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.deactivate(amplitude).map_err(PyErr::from)
    }
    fn deactivate_all(&mut self) {
        self.0.deactivate_all()
    }
}

#[pyclass]
pub struct ExtendedLogLikelihood_64(rust::manager::ExtendedLogLikelihood<f64>);
impl_convert!(
    ExtendedLogLikelihood_64,
    rust::manager::ExtendedLogLikelihood<f64>
);

#[pymethods]
impl ExtendedLogLikelihood_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn data_manager(&self) -> Manager_64 {
        self.0.data_manager.clone().into()
    }

    #[getter]
    fn mc_manager(&self) -> Manager_64 {
        self.0.mc_manager.clone().into()
    }

    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_64> {
        self.0
            .data_manager
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(NormSqr_64::from)
            .collect()
    }

    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_64> {
        self.0
            .data_manager
            .model
            .amplitudes
            .read()
            .clone()
            .into_iter()
            .map(Amplitude_64::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter_64> {
        self.0
            .data_manager
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter_64::from)
            .collect()
    }
    #[getter]
    fn free_parameters(&self) -> Vec<Parameter_64> {
        self.0
            .free_parameters()
            .into_iter()
            .map(Parameter_64::from)
            .collect()
    }
    #[getter]
    fn fixed_parameters(&self) -> Vec<Parameter_64> {
        self.0
            .fixed_parameters()
            .into_iter()
            .map(Parameter_64::from)
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
    fn new(data_manager: Manager_64, mc_manager: Manager_64) -> Self {
        rust::manager::ExtendedLogLikelihood::new(data_manager.into(), mc_manager.into()).into()
    }
    #[pyo3(signature = (parameters, *, indices_data = None, indices_mc = None, parallel = true))]
    fn evaluate(
        &self,
        parameters: Vec<f64>,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<f64> {
        if parallel {
            if self.0.data_manager.model.contains_python_amplitudes
                || self.0.mc_manager.model.contains_python_amplitudes
            {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            match (indices_data, indices_mc) {
                (None, None) => self.0.par_evaluate(&parameters),
                (None, Some(i_mc)) => self.0.par_evaluate_indexed(
                    &parameters,
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.par_evaluate_indexed(
                    &parameters,
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0.par_evaluate_indexed(&parameters, &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        } else {
            match (indices_data, indices_mc) {
                (None, None) => self.0.evaluate(&parameters),
                (None, Some(i_mc)) => self.0.evaluate_indexed(
                    &parameters,
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.evaluate_indexed(
                    &parameters,
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => self.0.evaluate_indexed(&parameters, &i_data, &i_mc),
            }
            .map_err(PyErr::from)
        }
    }
    #[pyo3(signature = (parameters, dataset, *, indices_data = None, indices_mc = None, parallel = true))]
    fn intensity(
        &self,
        parameters: Vec<f64>,
        dataset: Dataset_64,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f64>> {
        if parallel {
            if self.0.data_manager.model.contains_python_amplitudes
                || self.0.mc_manager.model.contains_python_amplitudes
            {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            match (indices_data, indices_mc) {
                (None, None) => self.0.par_intensity(&parameters, &dataset.into()),
                (None, Some(i_mc)) => self.0.par_intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.par_intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0
                        .par_intensity_indexed(&parameters, &dataset.into(), &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        } else {
            match (indices_data, indices_mc) {
                (None, None) => self.0.intensity(&parameters, &dataset.into()),
                (None, Some(i_mc)) => self.0.intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0
                        .intensity_indexed(&parameters, &dataset.into(), &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        }
    }
    #[pyo3(name = "__call__", signature = (parameters, *, indices_data = None, indices_mc = None, parallel = true))]
    fn call(
        &self,
        parameters: Vec<f64>,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<f64> {
        self.evaluate(parameters, indices_data, indices_mc, parallel)
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude_64> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude_64::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter_64> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter_64::from)
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
    fn activate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.activate(amplitude).map_err(PyErr::from)
    }
    fn activate_all(&mut self) {
        self.0.activate_all()
    }
    fn isolate(&mut self, amplitudes: Vec<String>) -> PyResult<()> {
        self.0
            .isolate(amplitudes.iter().map(|s| s.as_ref()).collect())
            .map_err(PyErr::from)
    }
    fn deactivate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.deactivate(amplitude).map_err(PyErr::from)
    }
    fn deactivate_all(&mut self) {
        self.0.deactivate_all()
    }
    #[pyo3(signature=(indices_data=None, indices_mc=None))]
    pub fn minimize(
        &self,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
    ) -> PyResult<Status_64> {
        let mut indices_tuple = match (indices_data, indices_mc) {
            (None, None) => None,
            (None, Some(i_mc)) => Some((
                (0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>(),
                i_mc,
            )),
            (Some(i_data), None) => Some((
                i_data,
                (0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>(),
            )),
            (Some(i_data), Some(i_mc)) => Some((i_data, i_mc)),
        };
        let algo = ganesh::algorithms::LBFGSB::default();
        let mut m = ganesh::Minimizer::new(algo, self.0.get_n_free())
            .with_bounds(Some(self.0.get_bounds()));
        m.minimize(&self.0, &self.0.get_initial(), &mut indices_tuple)
            .map_err(PyErr::from)?;
        Ok(m.status.into())
    }
}

#[pyclass]
pub struct ExtendedLogLikelihood_32(rust::manager::ExtendedLogLikelihood<f32>);
impl_convert!(
    ExtendedLogLikelihood_32,
    rust::manager::ExtendedLogLikelihood<f32>
);

#[pymethods]
impl ExtendedLogLikelihood_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn data_manager(&self) -> Manager_32 {
        self.0.data_manager.clone().into()
    }

    #[getter]
    fn mc_manager(&self) -> Manager_32 {
        self.0.mc_manager.clone().into()
    }

    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_32> {
        self.0
            .data_manager
            .model
            .cohsums
            .clone()
            .into_iter()
            .map(NormSqr_32::from)
            .collect()
    }

    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_32> {
        self.0
            .data_manager
            .model
            .amplitudes
            .read()
            .clone()
            .into_iter()
            .map(Amplitude_32::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter_32> {
        self.0
            .data_manager
            .model
            .parameters
            .clone()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn free_parameters(&self) -> Vec<Parameter_32> {
        self.0
            .free_parameters()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn fixed_parameters(&self) -> Vec<Parameter_32> {
        self.0
            .fixed_parameters()
            .into_iter()
            .map(Parameter_32::from)
            .collect()
    }
    #[getter]
    fn bounds(&self) -> Vec<(f32, f32)> {
        self.0.get_bounds()
    }
    #[getter]
    fn initial(&self) -> Vec<f32> {
        self.0.get_initial()
    }
    #[getter]
    fn n_free(&self) -> usize {
        self.0.get_n_free()
    }
    #[new]
    fn new(data_manager: Manager_32, mc_manager: Manager_32) -> Self {
        rust::manager::ExtendedLogLikelihood::new(data_manager.into(), mc_manager.into()).into()
    }
    #[pyo3(signature = (parameters, *, indices_data = None, indices_mc = None, parallel = true))]
    fn evaluate(
        &self,
        parameters: Vec<f32>,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<f32> {
        if parallel {
            if self.0.data_manager.model.contains_python_amplitudes
                || self.0.mc_manager.model.contains_python_amplitudes
            {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            match (indices_data, indices_mc) {
                (None, None) => self.0.par_evaluate(&parameters),
                (None, Some(i_mc)) => self.0.par_evaluate_indexed(
                    &parameters,
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.par_evaluate_indexed(
                    &parameters,
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0.par_evaluate_indexed(&parameters, &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        } else {
            match (indices_data, indices_mc) {
                (None, None) => self.0.evaluate(&parameters),
                (None, Some(i_mc)) => self.0.evaluate_indexed(
                    &parameters,
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.evaluate_indexed(
                    &parameters,
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => self.0.evaluate_indexed(&parameters, &i_data, &i_mc),
            }
            .map_err(PyErr::from)
        }
    }
    #[pyo3(signature = (parameters, dataset, *, indices_data = None, indices_mc = None, parallel = true))]
    fn intensity(
        &self,
        parameters: Vec<f32>,
        dataset: Dataset_32,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<Vec<f32>> {
        if parallel {
            if self.0.data_manager.model.contains_python_amplitudes
                || self.0.mc_manager.model.contains_python_amplitudes
            {
                return Err(PyRuntimeError::new_err(
                    "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!",
                ));
            }
            match (indices_data, indices_mc) {
                (None, None) => self.0.par_intensity(&parameters, &dataset.into()),
                (None, Some(i_mc)) => self.0.par_intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.par_intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0
                        .par_intensity_indexed(&parameters, &dataset.into(), &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        } else {
            match (indices_data, indices_mc) {
                (None, None) => self.0.intensity(&parameters, &dataset.into()),
                (None, Some(i_mc)) => self.0.intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &((0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>()),
                    &i_mc,
                ),
                (Some(i_data), None) => self.0.intensity_indexed(
                    &parameters,
                    &dataset.into(),
                    &i_data,
                    &((0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>()),
                ),
                (Some(i_data), Some(i_mc)) => {
                    self.0
                        .intensity_indexed(&parameters, &dataset.into(), &i_data, &i_mc)
                }
            }
            .map_err(PyErr::from)
        }
    }
    #[pyo3(name = "__call__", signature = (parameters, *, indices_data = None, indices_mc = None, parallel = true))]
    fn call(
        &self,
        parameters: Vec<f32>,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
        parallel: bool,
    ) -> PyResult<f32> {
        self.evaluate(parameters, indices_data, indices_mc, parallel)
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude_32> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude_32::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter_32> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter_32::from)
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
    fn fix(&mut self, amplitude: &str, parameter: &str, value: f32) -> PyResult<()> {
        self.0.fix(amplitude, parameter, value).map_err(PyErr::from)
    }
    fn free(&mut self, amplitude: &str, parameter: &str) -> PyResult<()> {
        self.0.free(amplitude, parameter).map_err(PyErr::from)
    }
    fn set_bounds(&mut self, amplitude: &str, parameter: &str, bounds: (f32, f32)) -> PyResult<()> {
        self.0
            .set_bounds(amplitude, parameter, bounds)
            .map_err(PyErr::from)
    }
    fn set_initial(&mut self, amplitude: &str, parameter: &str, value: f32) -> PyResult<()> {
        self.0
            .set_initial(amplitude, parameter, value)
            .map_err(PyErr::from)
    }
    fn activate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.activate(amplitude).map_err(PyErr::from)
    }
    fn activate_all(&mut self) {
        self.0.activate_all()
    }
    fn isolate(&mut self, amplitudes: Vec<String>) -> PyResult<()> {
        self.0
            .isolate(amplitudes.iter().map(|s| s.as_ref()).collect())
            .map_err(PyErr::from)
    }
    fn deactivate(&mut self, amplitude: &str) -> PyResult<()> {
        self.0.deactivate(amplitude).map_err(PyErr::from)
    }
    fn deactivate_all(&mut self) {
        self.0.deactivate_all()
    }
    #[pyo3(signature=(method="L-BFGS-B", *, indices_data=None, indices_mc=None))]
    pub fn minimize(
        &self,
        method: &str,
        indices_data: Option<Vec<usize>>,
        indices_mc: Option<Vec<usize>>,
    ) -> PyResult<Status_32> {
        let mut indices_tuple = match (indices_data, indices_mc) {
            (None, None) => None,
            (None, Some(i_mc)) => Some((
                (0..self.0.data_manager.dataset.len()).collect::<Vec<usize>>(),
                i_mc,
            )),
            (Some(i_data), None) => Some((
                i_data,
                (0..self.0.mc_manager.dataset.len()).collect::<Vec<usize>>(),
            )),
            (Some(i_data), Some(i_mc)) => Some((i_data, i_mc)),
        };
        match method {
            "L-BFGS-B" => {
                let algo = ganesh::algorithms::LBFGSB::default();
                let mut m = ganesh::Minimizer::new(algo, self.0.get_n_free())
                    .with_bounds(Some(self.0.get_bounds()));
                m.minimize(&self.0, &self.0.get_initial(), &mut indices_tuple)
                    .map_err(PyErr::from)?;
                Ok(m.status.into())
            }
            "Nelder-Mead" => {
                let algo = ganesh::algorithms::NelderMead::default();
                let mut m = ganesh::Minimizer::new(algo, self.0.get_n_free())
                    .with_bounds(Some(self.0.get_bounds()));
                m.minimize(&self.0, &self.0.get_initial(), &mut indices_tuple)
                    .map_err(PyErr::from)?;
                Ok(m.status.into())
            }
            "Adaptive Nelder-Mead" => {
                let algo =
                    ganesh::algorithms::NelderMead::default().with_adaptive(self.0.get_n_free());
                let mut m = ganesh::Minimizer::new(algo, self.0.get_n_free())
                    .with_bounds(Some(self.0.get_bounds()));
                m.minimize(&self.0, &self.0.get_initial(), &mut indices_tuple)
                    .map_err(PyErr::from)?;
                Ok(m.status.into())
            }
            _ => Err(PyException::new_err(format!("Unknown method: {}", method))),
        }
    }
}

#[pyclass]
pub struct Status_64(ganesh::Status<f64>);
impl_convert!(Status_64, ganesh::Status<f64>);

#[pymethods]
impl Status_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!(
            "<{}>",
            if self.0.converged {
                "Status: Converged"
            } else {
                "Status"
            }
        )
    }
    #[getter]
    fn x(&self) -> Vec<f64> {
        self.0.x.data.as_vec().to_vec()
    }
    #[getter]
    fn fx(&self) -> f64 {
        self.0.fx
    }
    #[getter]
    fn message(&self) -> String {
        self.0.message.clone()
    }
    #[getter]
    fn converged(&self) -> bool {
        self.0.converged
    }
    #[getter]
    fn err(&self) -> Option<Vec<f64>> {
        self.0.err.clone().map(|e| e.data.as_vec().to_vec())
    }
    #[getter]
    fn n_f_evals(&self) -> usize {
        self.0.n_f_evals
    }
    #[getter]
    fn n_g_evals(&self) -> usize {
        self.0.n_g_evals
    }
    #[getter]
    fn cov(&self) -> Option<Vec<Vec<f64>>> {
        self.0.cov.clone().map(|c| {
            c.row_iter()
                .map(|row| row.data.into_owned().as_vec().to_vec())
                .collect()
        })
    }
    #[getter]
    fn hess(&self) -> Option<Vec<Vec<f64>>> {
        self.0.hess.clone().map(|c| {
            c.row_iter()
                .map(|row| row.data.into_owned().as_vec().to_vec())
                .collect()
        })
    }
}

#[pyclass]
pub struct Status_32(ganesh::Status<f32>);
impl_convert!(Status_32, ganesh::Status<f32>);

#[pymethods]
impl Status_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!(
            "<{}>",
            if self.0.converged {
                "Status: Converged"
            } else {
                "Status"
            }
        )
    }
    #[getter]
    fn x(&self) -> Vec<f32> {
        self.0.x.data.as_vec().to_vec()
    }
    #[getter]
    fn fx(&self) -> f32 {
        self.0.fx
    }
    #[getter]
    fn message(&self) -> String {
        self.0.message.clone()
    }
    #[getter]
    fn converged(&self) -> bool {
        self.0.converged
    }
    #[getter]
    fn err(&self) -> Option<Vec<f32>> {
        self.0.err.clone().map(|e| e.data.as_vec().to_vec())
    }
    #[getter]
    fn n_f_evals(&self) -> usize {
        self.0.n_f_evals
    }
    #[getter]
    fn n_g_evals(&self) -> usize {
        self.0.n_g_evals
    }
    #[getter]
    fn cov(&self) -> Option<Vec<Vec<f32>>> {
        self.0.cov.clone().map(|c| {
            c.row_iter()
                .map(|row| row.data.into_owned().as_vec().to_vec())
                .collect()
        })
    }
    #[getter]
    fn hess(&self) -> Option<Vec<Vec<f32>>> {
        self.0.hess.clone().map(|c| {
            c.row_iter()
                .map(|row| row.data.into_owned().as_vec().to_vec())
                .collect()
        })
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager_64>()?;
    m.add_class::<Manager_32>()?;
    m.add_class::<ExtendedLogLikelihood_64>()?;
    m.add_class::<ExtendedLogLikelihood_32>()?;
    m.add_class::<Status_64>()?;
    m.add_class::<Status_32>()?;
    Ok(())
}
