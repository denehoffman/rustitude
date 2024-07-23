use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::dalitz as rust;

#[pyfunction]
fn OmegaDalitz(name: &str) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OmegaDalitz::default())
}

#[pyfunction]
fn OmegaDalitz_64(name: &str) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OmegaDalitz::default())
}
#[pyfunction]
fn OmegaDalitz_32(name: &str) -> Amplitude_32 {
    Amplitude_32::new(name, rust::OmegaDalitz::default())
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(OmegaDalitz, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_64, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_32, m)?)?;
    Ok(())
}
