use std::str::FromStr;

use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude::prelude::RustitudeError;
use rustitude_gluex::harmonics as rust;
use rustitude_gluex::utils::{Decay, Frame, Sign, Wave};

#[pyfunction]
#[pyo3(signature = (name, l, m, decay="[0, 1]", frame="helicity"))]
fn Ylm(name: &str, l: usize, m: isize, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
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
#[pyo3(signature = (name, l, m, decay="[0, 1]", frame="helicity"))]
fn Ylm_64(name: &str, l: usize, m: isize, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
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
#[pyo3(signature = (name, l, m, decay="[0, 1]", frame="helicity"))]
fn Ylm_32(name: &str, l: usize, m: isize, decay: &str, frame: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn Zlm(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn Zlm_64(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn Zlm_32(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn OnePS(name: &str, reflectivity: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::OnePS::new(
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn OnePS_64(name: &str, reflectivity: &str, decay: &str, frame: &str) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::OnePS::new(
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn OnePS_32(name: &str, reflectivity: &str, decay: &str, frame: &str) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::OnePS::new(
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn TwoPS(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn TwoPS_64(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
#[pyo3(signature = (name, l, m, reflectivity="+", decay="[0, 1]", frame="helicity"))]
fn TwoPS_32(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: &str,
    decay: &str,
    frame: &str,
) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            Sign::from_str(reflectivity)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
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
    m.add_function(wrap_pyfunction!(Ylm, m)?)?;
    m.add_function(wrap_pyfunction!(Ylm_64, m)?)?;
    m.add_function(wrap_pyfunction!(Ylm_32, m)?)?;
    m.add_function(wrap_pyfunction!(Zlm, m)?)?;
    m.add_function(wrap_pyfunction!(Zlm_64, m)?)?;
    m.add_function(wrap_pyfunction!(Zlm_32, m)?)?;
    m.add_function(wrap_pyfunction!(OnePS, m)?)?;
    m.add_function(wrap_pyfunction!(OnePS_64, m)?)?;
    m.add_function(wrap_pyfunction!(OnePS_32, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPS, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPS_64, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPS_32, m)?)?;
    Ok(())
}
