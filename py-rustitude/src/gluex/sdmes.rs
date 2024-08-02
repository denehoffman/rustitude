use std::str::FromStr;

use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude::prelude::RustitudeError;
use rustitude_gluex::sdmes as rust;
use rustitude_gluex::utils::{Decay, Frame};

#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn TwoPiSDME(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::TwoPiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn TwoPiSDME_64(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::TwoPiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn TwoPiSDME_32(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::TwoPiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]", frame="helicity"))]
fn ThreePiSDME(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::ThreePiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]", frame="helicity"))]
fn ThreePiSDME_64(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::ThreePiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1, 2]", frame="helicity"))]
fn ThreePiSDME_32(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::ThreePiSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn VecRadiativeSDME(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::VecRadiativeSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn VecRadiativeSDME_64(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::VecRadiativeSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, decay="[0, 1]", frame="helicity"))]
fn VecRadiativeSDME_32(name: &str, decay: &str, frame: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::VecRadiativeSDME::new(
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Frame::from_str(frame)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(TwoPiSDME, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPiSDME_64, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPiSDME_32, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME_64, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME_32, m)?)?;
    m.add_function(wrap_pyfunction!(VecRadiativeSDME, m)?)?;
    m.add_function(wrap_pyfunction!(VecRadiativeSDME_64, m)?)?;
    m.add_function(wrap_pyfunction!(VecRadiativeSDME_32, m)?)?;
    Ok(())
}
