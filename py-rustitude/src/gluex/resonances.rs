use std::str::FromStr;

use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude::prelude::RustitudeError;
use rustitude_gluex::{resonances as rust, utils::Decay};

#[pyfunction]
#[pyo3(signature = (name, l, decay="[0, 1]"))]
fn BreitWigner(name: &str, l: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::BreitWigner::new(
            l,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, l, decay="[0, 1]"))]
fn BreitWigner_64(name: &str, l: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::BreitWigner::new(
            l,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, l, decay="[0, 1]"))]
fn BreitWigner_32(name: &str, l: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::BreitWigner::new(
            l,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, m1s, m2s, decay="[0, 1]"))]
fn Flatte(
    name: &str,
    channel: usize,
    m1s: [f64; 2],
    m2s: [f64; 2],
    decay: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Flatte::new(
            channel,
            m1s,
            m2s,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, m1s, m2s, decay="[0, 1]"))]
fn Flatte_64(
    name: &str,
    channel: usize,
    m1s: [f64; 2],
    m2s: [f64; 2],
    decay: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Flatte::new(
            channel,
            m1s,
            m2s,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, m1s, m2s, decay="[0, 1]"))]
fn Flatte_32(
    name: &str,
    channel: usize,
    m1s: [f32; 2],
    m2s: [f32; 2],
    decay: &str,
) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::Flatte::new(
            channel,
            m1s,
            m2s,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA0(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixA0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA0_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixA0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA0_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixA0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA2(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixA2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA2_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixA2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixA2_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixA2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF0(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixF0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF0_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixF0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF0_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixF0::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF2(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixF2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF2_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixF2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixF2_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixF2::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixPi1(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixPi1::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixPi1_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixPi1::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixPi1_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixPi1::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixRho(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixRho::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixRho_64(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::KMatrixRho::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay="[0, 1]"))]
fn KMatrixRho_32(name: &str, channel: usize, decay: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::KMatrixRho::new(
            channel,
            Decay::from_str(decay)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(BreitWigner, m)?)?;
    m.add_function(wrap_pyfunction!(BreitWigner_64, m)?)?;
    m.add_function(wrap_pyfunction!(BreitWigner_32, m)?)?;
    m.add_function(wrap_pyfunction!(Flatte, m)?)?;
    m.add_function(wrap_pyfunction!(Flatte_64, m)?)?;
    m.add_function(wrap_pyfunction!(Flatte_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA0, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA0_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA0_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA2, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA2_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixA2_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF0, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF0_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF0_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF2, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF2_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixF2_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixPi1, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixPi1_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixPi1_32, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixRho, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixRho_64, m)?)?;
    m.add_function(wrap_pyfunction!(KMatrixRho_32, m)?)?;
    Ok(())
}
