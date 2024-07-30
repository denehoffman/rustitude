use ganesh::algorithms::nelder_mead;
use ganesh::core::Minimizer;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
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
}

#[pyclass]
pub struct NelderMead_64(nelder_mead::NelderMead<f64, (), rust::errors::RustitudeError>);
impl_convert!(NelderMead_64, nelder_mead::NelderMead<f64, (), rust::errors::RustitudeError>);

#[pymethods]
impl NelderMead_64 {
    #[new]
    #[pyo3(signature = (ell, *, simplex_size = 1.0, reflection_coeff = 1.0, expansion_coeff = 2.0, outside_contraction_coeff = 0.5, inside_contraction_coeff = 0.5, shrink_coeff = 0.5, min_simplex_standard_deviation = 1e-8))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ell: &ExtendedLogLikelihood_64,
        simplex_size: f64,
        reflection_coeff: f64,
        expansion_coeff: f64,
        outside_contraction_coeff: f64,
        inside_contraction_coeff: f64,
        shrink_coeff: f64,
        min_simplex_standard_deviation: f64,
    ) -> Self {
        nelder_mead::NelderMead::new(
            ell.0.clone(),
            &ell.0.get_initial(),
            Some(
                nelder_mead::NelderMeadOptions::builder()
                    .simplex_size(simplex_size)
                    .reflection_coeff(reflection_coeff)
                    .expansion_coeff(expansion_coeff)
                    .outside_contraction_coeff(outside_contraction_coeff)
                    .inside_contraction_coeff(inside_contraction_coeff)
                    .shrink_coeff(shrink_coeff)
                    .min_simplex_standard_deviation(min_simplex_standard_deviation)
                    .build(),
            ),
        )
        .into()
    }
    #[staticmethod]
    #[pyo3(signature = (ell, *, simplex_size = 1.0, min_simplex_standard_deviation = 1e-8))]
    fn adaptive(
        ell: &ExtendedLogLikelihood_64,
        simplex_size: f64,
        min_simplex_standard_deviation: f64,
    ) -> Self {
        nelder_mead::NelderMead::new(
            ell.0.clone(),
            &ell.0.get_initial(),
            Some(
                nelder_mead::NelderMeadOptions::adaptive(ell.0.get_n_free())
                    .simplex_size(simplex_size)
                    .min_simplex_standard_deviation(min_simplex_standard_deviation)
                    .build(),
            ),
        )
        .into()
    }
    fn initialize(&mut self) -> PyResult<()> {
        self.0.initialize(None).map_err(PyErr::from)
    }
    fn step(&mut self) -> PyResult<()> {
        self.0.step(None).map_err(PyErr::from)?;
        self.0.update_best();
        // this is added to allow for Python users to step through the algorithm
        // without having to manually call `update_best` every step
        Ok(())
    }
    fn check_for_termination(&self) -> bool {
        self.0.check_for_termination()
    }
    fn minimize(&mut self, steps: usize) -> PyResult<()> {
        self.0.minimize(None, steps, |_| {}).map_err(PyErr::from)
    }
    fn best(&self) -> (Vec<f64>, f64) {
        let (x_best, fx_best) = self.0.best();
        (x_best.clone(), *fx_best)
    }
}

#[pyclass]
pub struct NelderMead_32(nelder_mead::NelderMead<f32, (), rust::errors::RustitudeError>);
impl_convert!(NelderMead_32, nelder_mead::NelderMead<f32, (), rust::errors::RustitudeError>);

#[pymethods]
impl NelderMead_32 {
    #[new]
    #[pyo3(signature = (ell, *, simplex_size = 1.0, reflection_coeff = 1.0, expansion_coeff = 2.0, outside_contraction_coeff = 0.5, inside_contraction_coeff = 0.5, shrink_coeff = 0.5, min_simplex_standard_deviation = 1e-8))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ell: &ExtendedLogLikelihood_32,
        simplex_size: f32,
        reflection_coeff: f32,
        expansion_coeff: f32,
        outside_contraction_coeff: f32,
        inside_contraction_coeff: f32,
        shrink_coeff: f32,
        min_simplex_standard_deviation: f32,
    ) -> Self {
        nelder_mead::NelderMead::new(
            ell.0.clone(),
            &ell.0.get_initial(),
            Some(
                nelder_mead::NelderMeadOptions::builder()
                    .simplex_size(simplex_size)
                    .reflection_coeff(reflection_coeff)
                    .expansion_coeff(expansion_coeff)
                    .outside_contraction_coeff(outside_contraction_coeff)
                    .inside_contraction_coeff(inside_contraction_coeff)
                    .shrink_coeff(shrink_coeff)
                    .min_simplex_standard_deviation(min_simplex_standard_deviation)
                    .build(),
            ),
        )
        .into()
    }
    #[staticmethod]
    #[pyo3(signature = (ell, *, simplex_size = 1.0, min_simplex_standard_deviation = 1e-8))]
    fn adaptive(
        ell: &ExtendedLogLikelihood_32,
        simplex_size: f32,
        min_simplex_standard_deviation: f32,
    ) -> Self {
        nelder_mead::NelderMead::new(
            ell.0.clone(),
            &ell.0.get_initial(),
            Some(
                nelder_mead::NelderMeadOptions::adaptive(ell.0.get_n_free())
                    .simplex_size(simplex_size)
                    .min_simplex_standard_deviation(min_simplex_standard_deviation)
                    .build(),
            ),
        )
        .into()
    }
    fn initialize(&mut self) -> PyResult<()> {
        self.0.initialize(None).map_err(PyErr::from)
    }
    fn step(&mut self) -> PyResult<()> {
        self.0.step(None).map_err(PyErr::from)?;
        self.0.update_best();
        // this is added to allow for Python users to step through the algorithm
        // without having to manually call `update_best` every step
        Ok(())
    }
    fn check_for_termination(&self) -> bool {
        self.0.check_for_termination()
    }
    fn minimize(&mut self, steps: usize) -> PyResult<()> {
        self.0.minimize(None, steps, |_| {}).map_err(PyErr::from)
    }
    fn best(&self) -> (Vec<f32>, f32) {
        let (x_best, fx_best) = self.0.best();
        (x_best.clone(), *fx_best)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manager_64>()?;
    m.add_class::<Manager_32>()?;
    m.add_class::<ExtendedLogLikelihood_64>()?;
    m.add_class::<ExtendedLogLikelihood_32>()?;
    m.add_class::<NelderMead_64>()?;
    m.add_class::<NelderMead_32>()?;
    Ok(())
}
