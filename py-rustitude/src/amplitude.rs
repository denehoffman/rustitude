use crate::impl_convert;
use pyo3::{prelude::*, types::PyList};
use rustitude_core::{self as rust, amplitude::AmpLike as RustAmpLike};
use std::ops::{Add, Mul};

#[pyclass]
#[derive(Clone)]
pub struct Parameter64(rust::amplitude::Parameter<f64>);
impl_convert!(Parameter64, rust::amplitude::Parameter<f64>);

#[pymethods]
impl Parameter64 {
    #[new]
    fn new(amplitude: &str, name: &str, index: usize) -> Self {
        Self(rust::amplitude::Parameter::<f64>::new(
            amplitude, name, index,
        ))
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
    fn free(&self) -> bool {
        self.0.index.is_some()
    }
    #[getter]
    fn fixed(&self) -> bool {
        self.0.index.is_none()
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
pub struct Parameter32(rust::amplitude::Parameter<f32>);
impl_convert!(Parameter32, rust::amplitude::Parameter<f32>);

#[pymethods]
impl Parameter32 {
    #[new]
    fn new(amplitude: &str, name: &str, index: usize) -> Self {
        Self(rust::amplitude::Parameter::<f32>::new(
            amplitude, name, index,
        ))
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
    fn free(&self) -> bool {
        self.0.index.is_some()
    }
    #[getter]
    fn fixed(&self) -> bool {
        self.0.index.is_none()
    }
    #[getter]
    fn initial(&self) -> f32 {
        self.0.initial
    }
    #[getter]
    fn bounds(&self) -> (f32, f32) {
        self.0.bounds
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(name = "Node64")]
#[derive(Clone)]
struct PyNode64 {
    node: Py<PyAny>,
}
#[pymethods]
impl PyNode64 {
    #[new]
    pub fn new(node: Py<PyAny>) -> Self {
        PyNode64 { node }
    }
    pub fn precalculate(&mut self, dataset: crate::dataset::Dataset64) -> Result<(), PyErr> {
        rust::amplitude::Node::precalculate(self, &dataset.into()).map_err(PyErr::from)
    }
    pub fn calculate(
        &self,
        parameters: Vec<f64>,
        event: crate::dataset::Event64,
    ) -> Result<rust::prelude::Complex<f64>, PyErr> {
        rust::amplitude::Node::calculate(self, &parameters, &event.into()).map_err(PyErr::from)
    }
    pub fn parameters(&self) -> Vec<String> {
        rust::amplitude::Node::parameters(self)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn into_amplitude(&self, name: &str) -> Amplitude64 {
        Amplitude64(rust::amplitude::Node::into_amplitude(self.clone(), name))
    }
}

impl rust::amplitude::Node<f64> for PyNode64 {
    fn precalculate(
        &mut self,
        dataset: &rust::dataset::Dataset<f64>,
    ) -> Result<(), rust::errors::RustitudeError> {
        Python::with_gil(|py| {
            let py_dataset = crate::dataset::Dataset64::from(dataset.clone());
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
        event: &rust::dataset::Event<f64>,
    ) -> Result<rust::prelude::Complex<f64>, rustitude::prelude::RustitudeError> {
        Python::with_gil(|py| {
            let py_parameters = PyList::new_bound(py, parameters);
            let py_event = crate::dataset::Event64::from(event.clone());
            let py_event_obj = Py::new(py, py_event).unwrap();
            match self
                .node
                .call_method1(py, "calculate", (py_parameters, py_event_obj))
            {
                Ok(result) => {
                    let complex: rust::prelude::Complex<f64> = result.extract(py)?;
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

#[pyclass(name = "Node32")]
#[derive(Clone)]
struct PyNode32 {
    node: Py<PyAny>,
}
#[pymethods]
impl PyNode32 {
    #[new]
    pub fn new(node: Py<PyAny>) -> Self {
        PyNode32 { node }
    }
    pub fn precalculate(&mut self, dataset: crate::dataset::Dataset32) -> Result<(), PyErr> {
        rust::amplitude::Node::precalculate(self, &dataset.into()).map_err(PyErr::from)
    }
    pub fn calculate(
        &self,
        parameters: Vec<f32>,
        event: crate::dataset::Event32,
    ) -> Result<rust::prelude::Complex<f32>, PyErr> {
        rust::amplitude::Node::calculate(self, &parameters, &event.into()).map_err(PyErr::from)
    }
    pub fn parameters(&self) -> Vec<String> {
        rust::amplitude::Node::parameters(self)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn into_amplitude(&self, name: &str) -> Amplitude32 {
        Amplitude32(rust::amplitude::Node::into_amplitude(self.clone(), name))
    }
}

impl rust::amplitude::Node<f32> for PyNode32 {
    fn precalculate(
        &mut self,
        dataset: &rust::dataset::Dataset<f32>,
    ) -> Result<(), rust::errors::RustitudeError> {
        Python::with_gil(|py| {
            let py_dataset = crate::dataset::Dataset32::from(dataset.clone());
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
        parameters: &[f32],
        event: &rust::dataset::Event<f32>,
    ) -> Result<rust::prelude::Complex<f32>, rustitude::prelude::RustitudeError> {
        Python::with_gil(|py| {
            let py_parameters = PyList::new_bound(py, parameters);
            let py_event = crate::dataset::Event32::from(event.clone());
            let py_event_obj = Py::new(py, py_event).unwrap();
            match self
                .node
                .call_method1(py, "calculate", (py_parameters, py_event_obj))
            {
                Ok(result) => {
                    let complex: rust::prelude::Complex<f32> = result.extract(py)?;
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

#[derive(FromPyObject)]
enum AmpLikeOrCohSum64 {
    Amplitude(Amplitude64),
    Real(Real64),
    Imag(Imag64),
    Product(Product64),
    CohSum(CohSum64),
}

#[derive(FromPyObject)]
enum AmpLikeOrCohSum32 {
    Amplitude(Amplitude32),
    Real(Real32),
    Imag(Imag32),
    Product(Product32),
    CohSum(CohSum32),
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude64(rust::amplitude::Amplitude<f64>);
impl_convert!(Amplitude64, rust::amplitude::Amplitude<f64>);
impl Amplitude64 {
    pub fn new(name: &str, node: impl rust::amplitude::Node<f64> + 'static) -> Self {
        Self(rust::amplitude::Amplitude::<f64>::new(name, node))
    }
}
#[pymethods]
impl Amplitude64 {
    #[new]
    fn from_pynode(name: &str, pynode: PyNode64) -> Self {
        Self(rust::amplitude::Amplitude::<f64>::new(name, pynode))
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
    fn as_cohsum(&self) -> CohSum64 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::CohSum(al) => Ok(CohSum64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Real(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Imag(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Product(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::CohSum(al) => {
                Ok(Bound::new(py, CohSum64(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real64 {
        Real64(self.0.real())
    }
    fn imag(&self) -> Imag64 {
        Imag64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude32(rust::amplitude::Amplitude<f32>);
impl_convert!(Amplitude32, rust::amplitude::Amplitude<f32>);
impl Amplitude32 {
    pub fn new(name: &str, node: impl rust::amplitude::Node<f32> + 'static) -> Self {
        Self(rust::amplitude::Amplitude::<f32>::new(name, node))
    }
}
#[pymethods]
impl Amplitude32 {
    #[new]
    fn from_pynode(name: &str, pynode: PyNode32) -> Self {
        Self(rust::amplitude::Amplitude::<f32>::new(name, pynode))
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
    fn as_cohsum(&self) -> CohSum32 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::CohSum(al) => Ok(CohSum32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Real(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Imag(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Product(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::CohSum(al) => {
                Ok(Bound::new(py, CohSum32(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real32 {
        Real32(self.0.real())
    }
    fn imag(&self) -> Imag32 {
        Imag32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Real64(rust::amplitude::Real<f64>);
impl_convert!(Real64, rust::amplitude::Real<f64>);
#[pymethods]
impl Real64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum64 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::CohSum(al) => Ok(CohSum64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Real(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Imag(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Product(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::CohSum(al) => {
                Ok(Bound::new(py, CohSum64(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real64 {
        Real64(self.0.real())
    }
    fn imag(&self) -> Imag64 {
        Imag64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Real32(rust::amplitude::Real<f32>);
impl_convert!(Real32, rust::amplitude::Real<f32>);
#[pymethods]
impl Real32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum32 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::CohSum(al) => Ok(CohSum32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Real(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Imag(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Product(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::CohSum(al) => {
                Ok(Bound::new(py, CohSum32(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real32 {
        Real32(self.0.real())
    }
    fn imag(&self) -> Imag32 {
        Imag32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Imag64(rust::amplitude::Imag<f64>);
impl_convert!(Imag64, rust::amplitude::Imag<f64>);
#[pymethods]
impl Imag64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum64 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::CohSum(al) => Ok(CohSum64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Real(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Imag(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Product(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::CohSum(al) => {
                Ok(Bound::new(py, CohSum64(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real64 {
        Real64(self.0.real())
    }
    fn imag(&self) -> Imag64 {
        Imag64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Imag32(rust::amplitude::Imag<f32>);
impl_convert!(Imag32, rust::amplitude::Imag<f32>);
#[pymethods]
impl Imag32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum32 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::CohSum(al) => Ok(CohSum32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Real(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Imag(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Product(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::CohSum(al) => {
                Ok(Bound::new(py, CohSum32(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real32 {
        Real32(self.0.real())
    }
    fn imag(&self) -> Imag32 {
        Imag32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Product64(rust::amplitude::Product<f64>);
impl_convert!(Product64, rust::amplitude::Product<f64>);
#[pymethods]
impl Product64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum64 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::CohSum(al) => Ok(CohSum64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Real(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Imag(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::Product(al) => {
                Ok(Bound::new(py, Product64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum64::CohSum(al) => {
                Ok(Bound::new(py, CohSum64(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real64 {
        Real64(self.0.real())
    }
    fn imag(&self) -> Imag64 {
        Imag64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Product32(rust::amplitude::Product<f32>);
impl_convert!(Product32, rust::amplitude::Product<f32>);
#[pymethods]
impl Product32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn as_cohsum(&self) -> CohSum32 {
        self.0.as_cohsum().into()
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::CohSum(al) => Ok(CohSum32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Real(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Imag(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::Product(al) => {
                Ok(Bound::new(py, Product32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLikeOrCohSum32::CohSum(al) => {
                Ok(Bound::new(py, CohSum32(self.0.clone().mul(al.0)))?.into_any())
            }
        }
    }
    fn real(&self) -> Real32 {
        Real32(self.0.real())
    }
    fn imag(&self) -> Imag32 {
        Imag32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct CohSum64(rust::amplitude::CohSum<f64>);
impl_convert!(CohSum64, rust::amplitude::CohSum<f64>);

#[pymethods]
impl CohSum64 {
    #[new]
    pub fn new(terms: Vec<Bound<PyAny>>) -> PyResult<Self> {
        let mut rust_terms: Vec<Box<dyn rust::amplitude::AmpLike<f64>>> = Vec::default();
        for term in &terms {
            let term = AmpLikeOrCohSum64::extract_bound(term)?;
            match term {
                AmpLikeOrCohSum64::Amplitude(amplitude) => rust_terms.push(Box::new(amplitude.0)),
                AmpLikeOrCohSum64::Real(real) => rust_terms.push(Box::new(real.0)),
                AmpLikeOrCohSum64::Imag(imag) => rust_terms.push(Box::new(imag.0)),
                AmpLikeOrCohSum64::Product(product) => rust_terms.push(Box::new(product.0)),
                AmpLikeOrCohSum64::CohSum(cohsum) => rust_terms.extend((cohsum.0).0),
            }
        }
        Ok(Self(rust::amplitude::CohSum(rust_terms)))
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().add(al.0))),
            AmpLikeOrCohSum64::CohSum(al) => Ok(CohSum64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__(&self, other: &Bound<PyAny>) -> PyResult<CohSum64> {
        let other: AmpLikeOrCohSum64 = AmpLikeOrCohSum64::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum64::Amplitude(al) => Ok(CohSum64(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum64::Real(al) => Ok(CohSum64(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum64::Imag(al) => Ok(CohSum64(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum64::Product(al) => Ok(CohSum64(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum64::CohSum(_) => unimplemented!(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct CohSum32(rust::amplitude::CohSum<f32>);
impl_convert!(CohSum32, rust::amplitude::CohSum<f32>);

#[pymethods]
impl CohSum32 {
    #[new]
    pub fn new(terms: Vec<Bound<PyAny>>) -> PyResult<Self> {
        let mut rust_terms: Vec<Box<dyn rust::amplitude::AmpLike<f32>>> = Vec::default();
        for term in &terms {
            let term = AmpLikeOrCohSum32::extract_bound(term)?;
            match term {
                AmpLikeOrCohSum32::Amplitude(amplitude) => rust_terms.push(Box::new(amplitude.0)),
                AmpLikeOrCohSum32::Real(real) => rust_terms.push(Box::new(real.0)),
                AmpLikeOrCohSum32::Imag(imag) => rust_terms.push(Box::new(imag.0)),
                AmpLikeOrCohSum32::Product(product) => rust_terms.push(Box::new(product.0)),
                AmpLikeOrCohSum32::CohSum(cohsum) => rust_terms.extend((cohsum.0).0),
            }
        }
        Ok(Self(rust::amplitude::CohSum(rust_terms)))
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().add(al.0))),
            AmpLikeOrCohSum32::CohSum(al) => Ok(CohSum32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__(&self, other: &Bound<PyAny>) -> PyResult<CohSum32> {
        let other: AmpLikeOrCohSum32 = AmpLikeOrCohSum32::extract_bound(other)?;
        match other {
            AmpLikeOrCohSum32::Amplitude(al) => Ok(CohSum32(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum32::Real(al) => Ok(CohSum32(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum32::Imag(al) => Ok(CohSum32(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum32::Product(al) => Ok(CohSum32(self.0.clone().mul(al.0))),
            AmpLikeOrCohSum32::CohSum(_) => unimplemented!(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Model64(rust::amplitude::Model<f64>);
impl_convert!(Model64, rust::amplitude::Model<f64>);

#[pymethods]
impl Model64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<CohSum64> {
        self.0
            .clone()
            .cohsums
            .into_iter()
            .map(CohSum64::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude64> {
        self.0
            .amplitudes
            .clone()
            .into_iter()
            .map(Amplitude64::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter64> {
        self.0
            .parameters
            .clone()
            .into_iter()
            .map(Parameter64::from)
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
    fn new(cohsums: Vec<CohSum64>) -> Self {
        Self(rust::amplitude::Model::new(
            cohsums
                .into_iter()
                .map(rust::amplitude::CohSum::from)
                .collect(),
        ))
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude64> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude64::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter64> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter64::from)
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
pub struct Model32(rust::amplitude::Model<f32>);
impl_convert!(Model32, rust::amplitude::Model<f32>);

#[pymethods]
impl Model32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<CohSum32> {
        self.0
            .clone()
            .cohsums
            .into_iter()
            .map(CohSum32::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude32> {
        self.0
            .amplitudes
            .clone()
            .into_iter()
            .map(Amplitude32::from)
            .collect()
    }
    #[getter]
    fn parameters(&self) -> Vec<Parameter32> {
        self.0
            .parameters
            .clone()
            .into_iter()
            .map(Parameter32::from)
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
    fn new(cohsums: Vec<CohSum32>) -> Self {
        Self(rust::amplitude::Model::new(
            cohsums
                .into_iter()
                .map(rust::amplitude::CohSum::from)
                .collect(),
        ))
    }
    fn get_amplitude(&self, amplitude_name: &str) -> PyResult<Amplitude32> {
        self.0
            .get_amplitude(amplitude_name)
            .map(Amplitude32::from)
            .map_err(PyErr::from)
    }
    fn get_parameter(&self, amplitude_name: &str, parameter_name: &str) -> PyResult<Parameter32> {
        self.0
            .get_parameter(amplitude_name, parameter_name)
            .map(Parameter32::from)
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

#[pyfunction(name = "Scalar64")]
fn scalar64(name: &str) -> Amplitude64 {
    rust::amplitude::scalar(name).into()
}
#[pyfunction(name = "CScalar64")]
fn cscalar64(name: &str) -> Amplitude64 {
    rust::amplitude::cscalar(name).into()
}
#[pyfunction(name = "PCScalar64")]
fn pcscalar64(name: &str) -> Amplitude64 {
    rust::amplitude::pcscalar(name).into()
}
#[pyfunction(name = "PiecewiseM64")]
pub fn piecewise_m64(name: &str, bins: usize, range: (f64, f64)) -> Amplitude64 {
    rust::amplitude::piecewise_m(name, bins, range).into()
}
#[pyfunction(name = "Scalar32")]
fn scalar32(name: &str) -> Amplitude32 {
    rust::amplitude::scalar(name).into()
}
#[pyfunction(name = "CScalar32")]
fn cscalar32(name: &str) -> Amplitude32 {
    rust::amplitude::cscalar(name).into()
}
#[pyfunction(name = "PCScalar32")]
fn pcscalar32(name: &str) -> Amplitude32 {
    rust::amplitude::pcscalar(name).into()
}
#[pyfunction(name = "PiecewiseM32")]
pub fn piecewise_m32(name: &str, bins: usize, range: (f32, f32)) -> Amplitude32 {
    rust::amplitude::piecewise_m(name, bins, range).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Amplitude64>()?;
    m.add_class::<Amplitude32>()?;
    m.add_class::<Real64>()?;
    m.add_class::<Real32>()?;
    m.add_class::<Imag64>()?;
    m.add_class::<Imag32>()?;
    m.add_class::<Product64>()?;
    m.add_class::<Product32>()?;
    m.add_class::<Parameter64>()?;
    m.add_class::<Parameter32>()?;
    m.add_class::<CohSum64>()?;
    m.add_class::<CohSum32>()?;
    m.add_class::<Model64>()?;
    m.add_class::<Model32>()?;
    m.add_class::<PyNode64>()?;
    m.add_class::<PyNode32>()?;
    m.add_function(wrap_pyfunction!(scalar64, m)?)?;
    m.add_function(wrap_pyfunction!(scalar32, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar64, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar32, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar64, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar32, m)?)?;
    m.add_function(wrap_pyfunction!(piecewise_m64, m)?)?;
    m.add_function(wrap_pyfunction!(piecewise_m32, m)?)?;
    Ok(())
}
