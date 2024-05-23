use pyo3::prelude::*;
use rustitude_core::four_momentum as rust;
use std::mem::transmute;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct FourMomentum(rust::FourMomentum);

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
    fn boost_along(&self, other: &Self) -> Self {
        unsafe { transmute(self.0.boost_along(transmute(other))) }
    }
    fn __add__(&self, other: Self) -> Self {
        unsafe { transmute(self.0 + other.0) }
    }
    fn __sub__(&self, other: Self) -> Self {
        unsafe { transmute(self.0 - other.0) }
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FourMomentum>()?;
    Ok(())
}
