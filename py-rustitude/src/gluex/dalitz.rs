use crate::amplitude::{Amplitude32, Amplitude64};
use pyo3::prelude::*;
use rustitude_gluex::dalitz as rust;

#[pyfunction(name = "OmegaDalitz64")]
fn omega_dalitz64(name: &str) -> Amplitude64 {
    Amplitude64::new(name, rust::OmegaDalitz::default())
}
#[pyfunction(name = "OmegaDalitz32")]
fn omega_dalitz32(name: &str) -> Amplitude32 {
    Amplitude32::new(name, rust::OmegaDalitz::default())
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(omega_dalitz64, m)?)?;
    m.add_function(wrap_pyfunction!(omega_dalitz32, m)?)?;
    Ok(())
}
