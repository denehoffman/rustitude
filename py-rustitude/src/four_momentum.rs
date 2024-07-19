use crate::impl_convert;
use pyo3::prelude::*;
use rustitude_core::four_momentum as rust;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct FourMomentum_64(rust::FourMomentum<f64>);
impl_convert!(FourMomentum_64, rust::FourMomentum<f64>);

#[pymethods]
impl FourMomentum_64 {
    #[new]
    pub fn new(e: f64, px: f64, py: f64, pz: f64) -> Self {
        Self(rust::FourMomentum::new(e, px, py, pz))
    }
    fn __repr__(&self) -> String {
        format!("<FourMomentum (64-bit) ({})>", self.0)
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

#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct FourMomentum_32(rust::FourMomentum<f32>);
impl_convert!(FourMomentum_32, rust::FourMomentum<f32>);

#[pymethods]
impl FourMomentum_32 {
    #[new]
    pub fn new(e: f32, px: f32, py: f32, pz: f32) -> Self {
        Self(rust::FourMomentum::new(e, px, py, pz))
    }
    fn __repr__(&self) -> String {
        format!("<FourMomentum (32-bit) ({})>", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    #[getter]
    fn e(&self) -> f32 {
        self.0.e()
    }
    #[getter]
    fn px(&self) -> f32 {
        self.0.px()
    }
    #[getter]
    fn py(&self) -> f32 {
        self.0.py()
    }
    #[getter]
    fn pz(&self) -> f32 {
        self.0.pz()
    }
    #[getter]
    fn m(&self) -> f32 {
        self.0.m()
    }
    #[getter]
    fn m2(&self) -> f32 {
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
    m.add_class::<FourMomentum_64>()?;
    m.add_class::<FourMomentum_32>()?;
    Ok(())
}
