use std::str::FromStr;

use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude::prelude::RustitudeError;
use rustitude_gluex::{dalitz as rust, utils::Decay};

#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]"))]
fn OmegaDalitz(name: &str, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::OmegaDalitz::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]"))]
fn OmegaDalitz_64(name: &str, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::OmegaDalitz::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]"))]
fn OmegaDalitz_32(name: &str, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::OmegaDalitz::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(OmegaDalitz, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_64, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_32, m)?)?;
    Ok(())
}
