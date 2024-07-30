use crate::impl_convert;
use pyo3::{prelude::*, types::PyList};
use rustitude_core::{self as rust, amplitude::AmpLike as RustAmpLike};
use std::ops::{Add, Mul};

#[pyclass]
#[derive(Clone)]
pub struct Parameter_64(rust::amplitude::Parameter<f64>);
impl_convert!(Parameter_64, rust::amplitude::Parameter<f64>);

#[pymethods]
impl Parameter_64 {
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
        self.0.is_free()
    }
    #[getter]
    fn fixed(&self) -> bool {
        self.0.is_fixed()
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
pub struct Parameter_32(rust::amplitude::Parameter<f32>);
impl_convert!(Parameter_32, rust::amplitude::Parameter<f32>);

#[pymethods]
impl Parameter_32 {
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

#[pyclass(name = "Node_64")]
#[derive(Clone)]
struct PyNode_64 {
    node: Py<PyAny>,
}
#[pymethods]
impl PyNode_64 {
    #[new]
    pub fn new(node: Py<PyAny>) -> Self {
        PyNode_64 { node }
    }
    pub fn precalculate(&mut self, dataset: crate::dataset::Dataset_64) -> Result<(), PyErr> {
        rust::amplitude::Node::precalculate(self, &dataset.into()).map_err(PyErr::from)
    }
    pub fn calculate(
        &self,
        parameters: Vec<f64>,
        event: crate::dataset::Event_64,
    ) -> Result<rust::prelude::Complex<f64>, PyErr> {
        rust::amplitude::Node::calculate(self, &parameters, &event.into()).map_err(PyErr::from)
    }
    pub fn parameters(&self) -> Vec<String> {
        rust::amplitude::Node::parameters(self)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn into_amplitude(&self, name: &str) -> Amplitude_64 {
        Amplitude_64(rust::amplitude::Node::into_amplitude(self.clone(), name))
    }
}

impl rust::amplitude::Node<f64> for PyNode_64 {
    fn precalculate(
        &mut self,
        dataset: &rust::dataset::Dataset<f64>,
    ) -> Result<(), rust::errors::RustitudeError> {
        Python::with_gil(|py| {
            let py_dataset = crate::dataset::Dataset_64::from(dataset.clone());
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
            let py_event = crate::dataset::Event_64::from(event.clone());
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

#[pyclass(name = "Node_32")]
#[derive(Clone)]
struct PyNode_32 {
    node: Py<PyAny>,
}
#[pymethods]
impl PyNode_32 {
    #[new]
    pub fn new(node: Py<PyAny>) -> Self {
        PyNode_32 { node }
    }
    pub fn precalculate(&mut self, dataset: crate::dataset::Dataset_32) -> Result<(), PyErr> {
        rust::amplitude::Node::precalculate(self, &dataset.into()).map_err(PyErr::from)
    }
    pub fn calculate(
        &self,
        parameters: Vec<f32>,
        event: crate::dataset::Event_32,
    ) -> Result<rust::prelude::Complex<f32>, PyErr> {
        rust::amplitude::Node::calculate(self, &parameters, &event.into()).map_err(PyErr::from)
    }
    pub fn parameters(&self) -> Vec<String> {
        rust::amplitude::Node::parameters(self)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn into_amplitude(&self, name: &str) -> Amplitude_32 {
        Amplitude_32(rust::amplitude::Node::into_amplitude(self.clone(), name))
    }
}

impl rust::amplitude::Node<f32> for PyNode_32 {
    fn precalculate(
        &mut self,
        dataset: &rust::dataset::Dataset<f32>,
    ) -> Result<(), rust::errors::RustitudeError> {
        Python::with_gil(|py| {
            let py_dataset = crate::dataset::Dataset_32::from(dataset.clone());
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
            let py_event = crate::dataset::Event_32::from(event.clone());
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
enum AmpLike_64 {
    Amplitude(Amplitude_64),
    Real(Real_64),
    Imag(Imag_64),
    Product(Product_64),
    Sum(Sum_64),
}

#[derive(FromPyObject)]
enum AmpLike_32 {
    Amplitude(Amplitude_32),
    Real(Real_32),
    Imag(Imag_32),
    Product(Product_32),
    Sum(Sum_32),
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude_64(rust::amplitude::Amplitude<f64>);
impl_convert!(Amplitude_64, rust::amplitude::Amplitude<f64>);
impl Amplitude_64 {
    pub fn new(name: &str, node: impl rust::amplitude::Node<f64> + 'static) -> Self {
        Self(rust::amplitude::Amplitude::<f64>::new(name, node))
    }
}
#[pymethods]
impl Amplitude_64 {
    #[new]
    fn from_pynode(name: &str, pynode: PyNode_64) -> Self {
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
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Sum(al) => Ok(Sum_64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Real(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Imag(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Product(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Sum(al) => Ok(Bound::new(py, Sum_64(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_64 {
        Real_64(self.0.real())
    }
    fn imag(&self) -> Imag_64 {
        Imag_64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Amplitude_32(rust::amplitude::Amplitude<f32>);
impl_convert!(Amplitude_32, rust::amplitude::Amplitude<f32>);
impl Amplitude_32 {
    pub fn new(name: &str, node: impl rust::amplitude::Node<f32> + 'static) -> Self {
        Self(rust::amplitude::Amplitude::<f32>::new(name, node))
    }
}
#[pymethods]
impl Amplitude_32 {
    #[new]
    fn from_pynode(name: &str, pynode: PyNode_32) -> Self {
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
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Sum(al) => Ok(Sum_32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Real(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Imag(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Product(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Sum(al) => Ok(Bound::new(py, Sum_32(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_32 {
        Real_32(self.0.real())
    }
    fn imag(&self) -> Imag_32 {
        Imag_32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Real_64(rust::amplitude::Real<f64>);
impl_convert!(Real_64, rust::amplitude::Real<f64>);
#[pymethods]
impl Real_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Sum(al) => Ok(Sum_64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Real(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Imag(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Product(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Sum(al) => Ok(Bound::new(py, Sum_64(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_64 {
        Real_64(self.0.real())
    }
    fn imag(&self) -> Imag_64 {
        Imag_64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Real_32(rust::amplitude::Real<f32>);
impl_convert!(Real_32, rust::amplitude::Real<f32>);
#[pymethods]
impl Real_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Sum(al) => Ok(Sum_32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Real(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Imag(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Product(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Sum(al) => Ok(Bound::new(py, Sum_32(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_32 {
        Real_32(self.0.real())
    }
    fn imag(&self) -> Imag_32 {
        Imag_32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Imag_64(rust::amplitude::Imag<f64>);
impl_convert!(Imag_64, rust::amplitude::Imag<f64>);
#[pymethods]
impl Imag_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Sum(al) => Ok(Sum_64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Real(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Imag(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Product(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Sum(al) => Ok(Bound::new(py, Sum_64(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_64 {
        Real_64(self.0.real())
    }
    fn imag(&self) -> Imag_64 {
        Imag_64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Imag_32(rust::amplitude::Imag<f32>);
impl_convert!(Imag_32, rust::amplitude::Imag<f32>);
#[pymethods]
impl Imag_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Sum(al) => Ok(Sum_32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Real(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Imag(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Product(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Sum(al) => Ok(Bound::new(py, Sum_32(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_32 {
        Real_32(self.0.real())
    }
    fn imag(&self) -> Imag_32 {
        Imag_32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Product_64(rust::amplitude::Product<f64>);
impl_convert!(Product_64, rust::amplitude::Product<f64>);
#[pymethods]
impl Product_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Sum(al) => Ok(Sum_64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Real(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Imag(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Product(al) => {
                Ok(Bound::new(py, Product_64(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_64::Sum(al) => Ok(Bound::new(py, Sum_64(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_64 {
        Real_64(self.0.real())
    }
    fn imag(&self) -> Imag_64 {
        Imag_64(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Product_32(rust::amplitude::Product<f32>);
impl_convert!(Product_32, rust::amplitude::Product<f32>);
#[pymethods]
impl Product_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Sum(al) => Ok(Sum_32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__<'a>(&self, py: Python<'a>, other: &Bound<PyAny>) -> PyResult<Bound<'a, PyAny>> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Real(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Imag(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Product(al) => {
                Ok(Bound::new(py, Product_32(self.0.clone().mul(al.0)))?.into_any())
            }
            AmpLike_32::Sum(al) => Ok(Bound::new(py, Sum_32(self.0.clone().mul(al.0)))?.into_any()),
        }
    }
    fn real(&self) -> Real_32 {
        Real_32(self.0.real())
    }
    fn imag(&self) -> Imag_32 {
        Imag_32(self.0.imag())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sum_64(rust::amplitude::Sum<f64>);
impl_convert!(Sum_64, rust::amplitude::Sum<f64>);

#[pymethods]
impl Sum_64 {
    #[new]
    pub fn new(terms: Vec<Bound<PyAny>>) -> PyResult<Self> {
        let mut rust_terms: Vec<Box<dyn rust::amplitude::AmpLike<f64>>> = Vec::default();
        for term in &terms {
            let term = AmpLike_64::extract_bound(term)?;
            match term {
                AmpLike_64::Amplitude(amplitude) => rust_terms.push(Box::new(amplitude.0)),
                AmpLike_64::Real(real) => rust_terms.push(Box::new(real.0)),
                AmpLike_64::Imag(imag) => rust_terms.push(Box::new(imag.0)),
                AmpLike_64::Product(product) => rust_terms.push(Box::new(product.0)),
                AmpLike_64::Sum(sum) => rust_terms.extend((sum.0).0),
            }
        }
        Ok(Self(rust::amplitude::Sum(rust_terms)))
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().add(al.0))),
            AmpLike_64::Sum(al) => Ok(Sum_64(self.0.clone().add(al.0))),
        }
    }
    fn __mul__(&self, other: &Bound<PyAny>) -> PyResult<Sum_64> {
        let other: AmpLike_64 = AmpLike_64::extract_bound(other)?;
        match other {
            AmpLike_64::Amplitude(al) => Ok(Sum_64(self.0.clone().mul(al.0))),
            AmpLike_64::Real(al) => Ok(Sum_64(self.0.clone().mul(al.0))),
            AmpLike_64::Imag(al) => Ok(Sum_64(self.0.clone().mul(al.0))),
            AmpLike_64::Product(al) => Ok(Sum_64(self.0.clone().mul(al.0))),
            AmpLike_64::Sum(_) => unimplemented!(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sum_32(rust::amplitude::Sum<f32>);
impl_convert!(Sum_32, rust::amplitude::Sum<f32>);

#[pymethods]
impl Sum_32 {
    #[new]
    pub fn new(terms: Vec<Bound<PyAny>>) -> PyResult<Self> {
        let mut rust_terms: Vec<Box<dyn rust::amplitude::AmpLike<f32>>> = Vec::default();
        for term in &terms {
            let term = AmpLike_32::extract_bound(term)?;
            match term {
                AmpLike_32::Amplitude(amplitude) => rust_terms.push(Box::new(amplitude.0)),
                AmpLike_32::Real(real) => rust_terms.push(Box::new(real.0)),
                AmpLike_32::Imag(imag) => rust_terms.push(Box::new(imag.0)),
                AmpLike_32::Product(product) => rust_terms.push(Box::new(product.0)),
                AmpLike_32::Sum(cohsum) => rust_terms.extend((cohsum.0).0),
            }
        }
        Ok(Self(rust::amplitude::Sum(rust_terms)))
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    fn __add__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().add(al.0))),
            AmpLike_32::Sum(al) => Ok(Sum_32(self.0.clone().add(al.0))),
        }
    }
    fn __mul__(&self, other: &Bound<PyAny>) -> PyResult<Sum_32> {
        let other: AmpLike_32 = AmpLike_32::extract_bound(other)?;
        match other {
            AmpLike_32::Amplitude(al) => Ok(Sum_32(self.0.clone().mul(al.0))),
            AmpLike_32::Real(al) => Ok(Sum_32(self.0.clone().mul(al.0))),
            AmpLike_32::Imag(al) => Ok(Sum_32(self.0.clone().mul(al.0))),
            AmpLike_32::Product(al) => Ok(Sum_32(self.0.clone().mul(al.0))),
            AmpLike_32::Sum(_) => unimplemented!(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct NormSqr_64(rust::amplitude::NormSqr<f64>);
impl_convert!(NormSqr_64, rust::amplitude::NormSqr<f64>);

#[pymethods]
impl NormSqr_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct NormSqr_32(rust::amplitude::NormSqr<f32>);
impl_convert!(NormSqr_32, rust::amplitude::NormSqr<f32>);

#[pymethods]
impl NormSqr_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Model_64(rust::amplitude::Model<f64>);
impl_convert!(Model_64, rust::amplitude::Model<f64>);

#[pymethods]
impl Model_64 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_64> {
        self.0
            .clone()
            .cohsums
            .into_iter()
            .map(NormSqr_64::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_64> {
        self.0
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
    fn new(amps: Vec<PyObject>) -> PyResult<Self> {
        Python::with_gil(|py| {
            let rust_amps: PyResult<Vec<Box<dyn rust::amplitude::AmpLike<f64>>>> = amps
                .into_iter()
                .map(|obj| {
                    if let Ok(amp) = obj.extract::<Amplitude_64>(py) {
                        Ok(Box::new(rust::amplitude::Amplitude::<f64>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f64>>)
                    } else if let Ok(amp) = obj.extract::<Real_64>(py) {
                        Ok(Box::new(rust::amplitude::Real::<f64>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f64>>)
                    } else if let Ok(amp) = obj.extract::<Imag_64>(py) {
                        Ok(Box::new(rust::amplitude::Imag::<f64>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f64>>)
                    } else if let Ok(amp) = obj.extract::<Product_64>(py) {
                        Ok(Box::new(rust::amplitude::Product::<f64>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f64>>)
                    } else if let Ok(amp) = obj.extract::<Sum_64>(py) {
                        Ok(Box::new(rust::amplitude::Sum::<f64>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f64>>)
                    } else {
                        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                            "Unsupported AmpLike type",
                        ))
                    }
                })
                .collect();
            let rust_amps = rust_amps?;
            let model = rust::amplitude::Model::new(&rust_amps);
            Ok(Self(model))
        })
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
pub struct Model_32(rust::amplitude::Model<f32>);
impl_convert!(Model_32, rust::amplitude::Model<f32>);

#[pymethods]
impl Model_32 {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    #[getter]
    fn cohsums(&self) -> Vec<NormSqr_32> {
        self.0
            .clone()
            .cohsums
            .into_iter()
            .map(NormSqr_32::from)
            .collect()
    }
    #[getter]
    fn amplitudes(&self) -> Vec<Amplitude_32> {
        self.0
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
    fn new(amps: Vec<PyObject>) -> PyResult<Self> {
        Python::with_gil(|py| {
            let rust_amps: PyResult<Vec<Box<dyn rust::amplitude::AmpLike<f32>>>> = amps
                .into_iter()
                .map(|obj| {
                    if let Ok(amp) = obj.extract::<Amplitude_32>(py) {
                        Ok(Box::new(rust::amplitude::Amplitude::<f32>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f32>>)
                    } else if let Ok(amp) = obj.extract::<Real_32>(py) {
                        Ok(Box::new(rust::amplitude::Real::<f32>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f32>>)
                    } else if let Ok(amp) = obj.extract::<Imag_32>(py) {
                        Ok(Box::new(rust::amplitude::Imag::<f32>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f32>>)
                    } else if let Ok(amp) = obj.extract::<Product_32>(py) {
                        Ok(Box::new(rust::amplitude::Product::<f32>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f32>>)
                    } else if let Ok(amp) = obj.extract::<Sum_32>(py) {
                        Ok(Box::new(rust::amplitude::Sum::<f32>::from(amp))
                            as Box<dyn rust::amplitude::AmpLike<f32>>)
                    } else {
                        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                            "Unsupported AmpLike type",
                        ))
                    }
                })
                .collect();
            let rust_amps = rust_amps?;
            let model = rust::amplitude::Model::new(&rust_amps);
            Ok(Self(model))
        })
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

#[pyfunction]
fn Scalar_64(name: &str) -> Amplitude_64 {
    rust::amplitude::scalar(name).into()
}
#[pyfunction]
fn CScalar_64(name: &str) -> Amplitude_64 {
    rust::amplitude::cscalar(name).into()
}
#[pyfunction]
fn PCScalar_64(name: &str) -> Amplitude_64 {
    rust::amplitude::pcscalar(name).into()
}
#[pyfunction]
pub fn PiecewiseM_64(name: &str, bins: usize, range: (f64, f64)) -> Amplitude_64 {
    rust::amplitude::piecewise_m(name, bins, range).into()
}
#[pyfunction]
fn Scalar_32(name: &str) -> Amplitude_32 {
    rust::amplitude::scalar(name).into()
}
#[pyfunction]
fn CScalar_32(name: &str) -> Amplitude_32 {
    rust::amplitude::cscalar(name).into()
}
#[pyfunction]
fn PCScalar_32(name: &str) -> Amplitude_32 {
    rust::amplitude::pcscalar(name).into()
}
#[pyfunction]
pub fn PiecewiseM_32(name: &str, bins: usize, range: (f32, f32)) -> Amplitude_32 {
    rust::amplitude::piecewise_m(name, bins, range).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Amplitude_64>()?;
    m.add_class::<Amplitude_32>()?;
    m.add_class::<Real_64>()?;
    m.add_class::<Real_32>()?;
    m.add_class::<Imag_64>()?;
    m.add_class::<Imag_32>()?;
    m.add_class::<Product_64>()?;
    m.add_class::<Product_32>()?;
    m.add_class::<Parameter_64>()?;
    m.add_class::<Parameter_32>()?;
    m.add_class::<Sum_64>()?;
    m.add_class::<Sum_32>()?;
    m.add_class::<NormSqr_64>()?;
    m.add_class::<NormSqr_32>()?;
    m.add_class::<Model_64>()?;
    m.add_class::<Model_32>()?;
    m.add_class::<PyNode_64>()?;
    m.add_class::<PyNode_32>()?;
    m.add_function(wrap_pyfunction!(Scalar_64, m)?)?;
    m.add_function(wrap_pyfunction!(Scalar_32, m)?)?;
    m.add_function(wrap_pyfunction!(CScalar_64, m)?)?;
    m.add_function(wrap_pyfunction!(CScalar_32, m)?)?;
    m.add_function(wrap_pyfunction!(PCScalar_64, m)?)?;
    m.add_function(wrap_pyfunction!(PCScalar_32, m)?)?;
    m.add_function(wrap_pyfunction!(PiecewiseM_64, m)?)?;
    m.add_function(wrap_pyfunction!(PiecewiseM_32, m)?)?;
    Ok(())
}
