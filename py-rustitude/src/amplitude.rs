use pyo3::{prelude::*, types::PyList};
use rustitude_core::amplitude as rust;
use rustitude_core::amplitude::AmpLike as rust_AmpLike;
use std::ops::{Add, Mul};

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

macro_rules! impl_convert {
    ($a:ty, $b:ty) => {
        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                Self(value)
            }
        }
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                value.0
            }
        }
    };
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude(rust::Amplitude);
impl_convert!(Amplitude, rust::Amplitude);
impl Amplitude {
    pub fn new(name: &str, node: impl rust::Node + 'static) -> Self {
        Self(rust::Amplitude::new(name, node))
    }
}
#[pymethods]
impl Amplitude {
    #[new]
    fn from_pynode(name: &str, pynode: PyNode) -> Self {
        Self(rust::Amplitude::new(name, pynode))
    }
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
    fn as_cohsum(&self) -> CohSum {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::CohSum(al) => Ok(CohSum(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Real(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Imag(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Product(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::CohSum(al) => {
                Ok(Bound::new(py, CohSum(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real {
        Real(self.0.real())
    }
    fn imag(&self) -> Imag {
        Imag(self.0.imag())
    }
}

#[pyclass(name = "Node")]
#[derive(Clone)]
struct PyNode {
    node: Py<PyAny>,
}
#[pymethods]
impl PyNode {
    #[new]
    pub fn new(node: Py<PyAny>) -> Self {
        PyNode { node }
    }
    pub fn precalculate(&mut self, dataset: crate::dataset::Dataset) -> Result<(), PyErr> {
        rust::Node::precalculate(self, &dataset.into()).map_err(PyErr::from)
    }
    pub fn calculate(
        &self,
        parameters: Vec<f64>,
        event: crate::dataset::Event,
    ) -> Result<rustitude_core::prelude::Complex64, PyErr> {
        rust::Node::calculate(self, &parameters, &event.into()).map_err(PyErr::from)
    }
    pub fn parameters(&self) -> Vec<String> {
        rust::Node::parameters(self)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn into_amplitude(&self, name: &str) -> Amplitude {
        Amplitude(rust::Node::into_amplitude(self.clone(), name))
    }
}

impl rust::Node for PyNode {
    fn precalculate(
        &mut self,
        dataset: &rustitude::prelude::Dataset,
    ) -> Result<(), rustitude::prelude::RustitudeError> {
        Python::with_gil(|py| {
            let py_dataset = crate::dataset::Dataset::from(dataset.clone());
            let py_dataset_obj = Py::new(py, py_dataset).unwrap();
            match self
                .node
                .call_method1(py, "precalculate", (py_dataset_obj,))
            {
                Ok(_) => Ok(()),
                Err(e) => Err(rustitude_core::errors::RustitudeError::from(e)),
            }
        })
    }

    fn calculate(
        &self,
        parameters: &[f64],
        event: &rustitude::prelude::Event,
    ) -> Result<rustitude::prelude::Complex64, rustitude::prelude::RustitudeError> {
        Python::with_gil(|py| {
            let py_parameters = PyList::new_bound(py, parameters);
            let py_event = crate::dataset::Event::from(event.clone());
            let py_event_obj = Py::new(py, py_event).unwrap();
            match self
                .node
                .call_method1(py, "calculate", (py_parameters, py_event_obj))
            {
                Ok(result) => {
                    let complex: rustitude::prelude::Complex64 = result.extract(py)?;
                    Ok(complex)
                }
                Err(e) => Err(rustitude_core::errors::RustitudeError::from(e)),
            }
        })
    }

    fn parameters(&self) -> Vec<String> {
        Python::with_gil(|py| {
            self.node
                .bind(py)
                .call_method("parameters", (), None)
                .unwrap()
                .extract()
                .unwrap()
        })
    }

    fn is_python_node(&self) -> bool {
        true
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Real(rust::Real);
impl_convert!(Real, rust::Real);
#[pymethods]
impl Real {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::CohSum(al) => Ok(CohSum(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Real(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Imag(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Product(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::CohSum(al) => {
                Ok(Bound::new(py, CohSum(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real {
        Real(self.0.real())
    }
    fn imag(&self) -> Imag {
        Imag(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Imag(rust::Imag);
impl_convert!(Imag, rust::Imag);
#[pymethods]
impl Imag {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::CohSum(al) => Ok(CohSum(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Real(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Imag(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Product(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::CohSum(al) => {
                Ok(Bound::new(py, CohSum(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real {
        Real(self.0.real())
    }
    fn imag(&self) -> Imag {
        Imag(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Product(rust::Product);
impl_convert!(Product, rust::Product);
#[pymethods]
impl Product {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::CohSum(al) => Ok(CohSum(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Real(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Imag(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::Product(al) => {
                Ok(Bound::new(py, Product(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum::CohSum(al) => {
                Ok(Bound::new(py, CohSum(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real {
        Real(self.0.real())
    }
    fn imag(&self) -> Imag {
        Imag(self.0.imag())
    }
}

#[derive(FromPyObject)]
enum AmpLikeOrCohSum {
    Amplitude(Amplitude),
    Real(Real),
    Imag(Imag),
    Product(Product),
    CohSum(CohSum),
}

#[pyclass]
#[derive(Clone)]
pub struct CohSum(rust::CohSum);

#[pymethods]
impl CohSum {
    #[new]
    pub fn new(terms: Vec<Bound<PyAny>>) -> PyResult<Self> {
        let mut rust_terms: Vec<Box<dyn rust::AmpLike>> = Vec::default();
        for term in &terms {
            let term = AmpLikeOrCohSum::extract_bound(term)?;
            match term {
                AmpLikeOrCohSum::Amplitude(amplitude) => rust_terms.push(Box::new(amplitude.0)),
                AmpLikeOrCohSum::Real(real) => rust_terms.push(Box::new(real.0)),
                AmpLikeOrCohSum::Imag(imag) => rust_terms.push(Box::new(imag.0)),
                AmpLikeOrCohSum::Product(product) => rust_terms.push(Box::new(product.0)),
                AmpLikeOrCohSum::CohSum(cohsum) => rust_terms.extend((cohsum.0).0),
            }
        }
        Ok(Self(rust::CohSum(rust_terms)))
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().add(al.0))),
            AmpLikeOrCohSum::CohSum(al) => Ok(CohSum(self.0.clone().add(al.0))),
        }
    }
    fn __mul__(&self, other: &Bound<PyAny>) -> PyResult<CohSum> {
        let other: AmpLikeOrCohSum = AmpLikeOrCohSum::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum::Amplitude(al) => Ok(CohSum(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum::Real(al) => Ok(CohSum(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum::Imag(al) => Ok(CohSum(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum::Product(al) => Ok(CohSum(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum::CohSum(_) => unimplemented!(),
        }
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
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
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
    fn activate_all(&mut self) {
        self.0.activate_all()
    }
    fn deactivate(&mut self, amplitude: &str) {
        self.0.deactivate(amplitude)
    }
    fn deactivate_all(&mut self) {
        self.0.deactivate_all()
    }
}

#[pyfunction(name = "Scalar")]
fn scalar(name: &str) -> Amplitude {
    rust::scalar(name).into()
}
#[pyfunction(name = "CScalar")]
fn cscalar(name: &str) -> Amplitude {
    rust::cscalar(name).into()
}
#[pyfunction(name = "PCScalar")]
fn pcscalar(name: &str) -> Amplitude {
    rust::pcscalar(name).into()
}
#[pyfunction(name = "PiecewiseM")]
pub fn piecewise_m(name: &str, bins: usize, range: (f64, f64)) -> Amplitude {
    rust::piecewise_m(name, bins, range).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Amplitude>()?;
    m.add_class::<Real>()?;
    m.add_class::<Imag>()?;
    m.add_class::<Product>()?;
    m.add_class::<Parameter>()?;
    m.add_class::<Amplitude>()?;
    m.add_class::<CohSum>()?;
    m.add_class::<Model>()?;
    m.add_class::<PyNode>()?;
    m.add_function(wrap_pyfunction!(scalar, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar, m)?)?;
    m.add_function(wrap_pyfunction!(piecewise_m, m)?)?;
    Ok(())
}
