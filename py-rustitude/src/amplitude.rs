use pyo3::prelude::*;
use rustitude_core::amplitude as rust;

#[pyclass]
#[derive(Clone)]
pub struct Parameter(rust::Parameter);

impl From<rust::Parameter> for Parameter {
    fn from(par: rust::Parameter) -> Self {
        Parameter(par)
    }
}
impl From<Parameter> for rust::Parameter {
    fn from(par: Parameter) -> Self {
        par.0
    }
}

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
    fn __add__(&self, other: Self) -> CohSum {
        (self.0.clone() + other.0).into()
    }
    fn __mul__(&self, other: Self) -> Self {
        (self.0.clone() * other.0).into()
    }
}

impl From<rust::AmpOp> for AmpOp {
    fn from(ampop: rust::AmpOp) -> Self {
        AmpOp(ampop)
    }
}
impl From<AmpOp> for rust::AmpOp {
    fn from(ampop: AmpOp) -> Self {
        ampop.0
    }
}

#[pyclass]
#[derive(Clone)]
pub struct CohSum(rust::CohSum);

#[pymethods]
impl CohSum {
    #[new]
    pub fn new(terms: Vec<AmpOp>) -> Self {
        Self(rust::CohSum::new(
            terms.into_iter().map(rust::AmpOp::from).collect(),
        ))
    }
    fn print_tree(&self) {
        self.0.print_tree()
    }
    fn __add__(&self, other: Self) -> CohSum {
        (self.0.clone() + other.0).into()
    }
    fn __mul__(&self, other: AmpOp) -> Self {
        (self.0.clone() * other.0).into()
    }
}

impl From<rust::CohSum> for CohSum {
    fn from(cohsum: rust::CohSum) -> Self {
        CohSum(cohsum)
    }
}
impl From<CohSum> for rust::CohSum {
    fn from(cohsum: CohSum) -> Self {
        cohsum.0
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude(rust::Amplitude);

impl From<rust::Amplitude> for Amplitude {
    fn from(amp: rust::Amplitude) -> Self {
        Amplitude(amp)
    }
}
impl From<Amplitude> for rust::Amplitude {
    fn from(amp: Amplitude) -> Self {
        amp.0
    }
}

impl Amplitude {
    pub fn new(name: &str, node: impl rust::Node + 'static) -> Self {
        Self(rust::Amplitude::new(name, node))
    }
}

impl From<Amplitude> for AmpOp {
    fn from(amp: Amplitude) -> Self {
        rust::AmpOp::Amplitude(amp.into()).into()
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

impl From<rust::Model> for Model {
    fn from(model: rust::Model) -> Self {
        Model(model)
    }
}
impl From<Model> for rust::Model {
    fn from(model: Model) -> Self {
        model.0
    }
}

#[pymethods]
impl Model {
    fn print_tree(&self) {
        self.0.print_tree()
    }
    #[getter]
    fn cohsums(&self) -> Vec<CohSum> {
        self.0
            .clone()
            .cohsums
            .into_iter()
            .map(CohSum::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude> {
        self.0
            .amplitudes
            .clone()
            .into_iter()
            .map(Amplitude::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter> {
        self.0
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
    fn new(cohsums: Vec<CohSum>) -> Self {
        Self(rust::Model::new(
            cohsums.into_iter().map(rust::CohSum::from).collect(),
        ))
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
    m.add_class::<CohSum>()?;
    m.add_class::<Model>()?;
    m.add_function(wrap_pyfunction!(scalar, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar, m)?)?;
    m.add_function(wrap_pyfunction!(piecewise_m, m)?)?;
    Ok(())
}
