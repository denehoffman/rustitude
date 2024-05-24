use pyo3::prelude::*;
use rustitude_core::four_momentum as rust;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct FourMomentum(rust::FourMomentum);

impl From<FourMomentum> for rust::FourMomentum {
    fn from(p4: FourMomentum) -> Self {
        p4.0
    }
}
impl From<rust::FourMomentum> for FourMomentum {
    fn from(p4: rust::FourMomentum) -> Self {
        FourMomentum(p4)
    }
}

#[pymethods]
impl FourMomentum {
    #[new]
    pub const fn new(e: f64, px: f64, py: f64, pz: f64) -> Self {
        Self(rust::FourMomentum::new(e, px, py, pz))
    }
    fn __repr__(&self) -> String {
        format!("<FourMomentum ({})>", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    #[getter]
    fn e(&self) -> f64 {
        self.0.e()
    }
    #[getter]
    fn px(&self) -> f64 {
        self.0.px()
    }
    #[getter]
    fn py(&self) -> f64 {
        self.0.py()
    }
    #[getter]
    fn pz(&self) -> f64 {
        self.0.pz()
    }
    #[getter]
    fn m(&self) -> f64 {
        self.0.m()
    }
    #[getter]
    fn m2(&self) -> f64 {
        self.0.m2()
    }
    fn boost_along(&self, other: Self) -> Self {
        self.0.boost_along(&other.into()).into()
    }
    fn __add__(&self, other: Self) -> Self {
        (self.0 + other.0).into()
    }
    fn __sub__(&self, other: Self) -> Self {
        (self.0 - other.0).into()
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FourMomentum>()?;
    Ok(())
}
