use pyo3::prelude::*;
use rustitude_core::amplitude as rust;
use std::mem::transmute;

#[pyclass]
#[derive(Clone)]
pub struct Parameter(rust::Parameter);

#[pymethods]
impl Parameter {
    #[new]
    fn new(amplitude: &str, name: &str, index: usize) -> Self {
        Self(rust::Parameter::new(amplitude, name, index))
    }
    #[getter]
    fn amplitude(&self) -> String {
        self.0.amplitude.clone()
    }
    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }
    #[getter]
    fn index(&self) -> Option<usize> {
        self.0.index
    }
    #[getter]
    fn fixed_index(&self) -> Option<usize> {
        self.0.fixed_index
    }
    #[getter]
    fn initial(&self) -> f64 {
        self.0.initial
    }
    #[getter]
    fn bounds(&self) -> (f64, f64) {
        self.0.bounds
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AmpOp(rust::AmpOp);

#[pymethods]
impl AmpOp {
    fn print_tree(&self) {
        self.0.print_tree()
    }
    fn real(&self) -> Self {
        self.0.real().into()
    }
    fn imag(&self) -> Self {
        self.0.imag().into()
    }
    fn norm_sqr(&self) -> Self {
        self.0.norm_sqr().into()
    }
    fn __add__(&self, other: Self) -> Self {
        (self.0.clone() + other.0).into()
    }
    fn __mul__(&self, other: Self) -> Self {
        (self.0.clone() * other.0).into()
    }
}

impl From<rust::AmpOp> for AmpOp {
    fn from(r_ampop: rust::AmpOp) -> Self {
        unsafe { transmute(r_ampop) }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude(rust::Amplitude);

impl Amplitude {
    pub fn new(name: &str, node: impl rust::Node + 'static) -> Self {
        Self(rust::Amplitude::new(name, node))
    }
}

impl From<Amplitude> for AmpOp {
    fn from(amp: Amplitude) -> Self {
        unsafe { transmute(rust::AmpOp::Amplitude(transmute(amp))) }
    }
}

#[pymethods]
impl Amplitude {
    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }
    #[getter]
    fn active(&self) -> bool {
        self.0.active
    }
    #[getter]
    fn cache_position(&self) -> usize {
        self.0.cache_position
    }
    #[getter]
    fn parameter_index_start(&self) -> usize {
        self.0.parameter_index_start
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Model(rust::Model);

#[pymethods]
impl Model {
    #[getter]
    fn root(&self) -> AmpOp {
        unsafe { transmute(self.0.root.clone()) }
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude> {
        unsafe { transmute(self.0.amplitudes.clone()) }
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter> {
        unsafe { transmute(self.0.parameters.clone()) }
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
    fn new(root: AmpOp) -> Self {
        unsafe { Self(rust::Model::new(transmute(root))) }
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> Option<Parameter> {
        unsafe { transmute(self.0.get_parameter(amplitude_name, parameter_name)) }
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

#[pyfunction(name = "Scalar")]
fn scalar(name: &str) -> AmpOp {
    rust::scalar(name).into()
}
#[pyfunction(name = "CScalar")]
fn cscalar(name: &str) -> AmpOp {
    rust::cscalar(name).into()
}
#[pyfunction(name = "PCScalar")]
fn pcscalar(name: &str) -> AmpOp {
    rust::pcscalar(name).into()
}
#[pyfunction(name = "PiecewiseM")]
pub fn piecewise_m(name: &str, bins: usize, range: (f64, f64)) -> AmpOp {
    rust::piecewise_m(name, bins, range).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AmpOp>()?;
    m.add_class::<Parameter>()?;
    m.add_class::<Amplitude>()?;
    m.add_class::<Model>()?;
    m.add_function(wrap_pyfunction!(scalar, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar, m)?)?;
    m.add_function(wrap_pyfunction!(piecewise_m, m)?)?;
    Ok(())
}
