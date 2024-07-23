use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::sdmes as rust;
use rustitude_gluex::utils::Frame;

#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn TwoPiSDME(name: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn TwoPiSDME_64(name: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn TwoPiSDME_32(name: &str, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn ThreePiSDME(name: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn ThreePiSDME_64(name: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(signature = (name, frame="helicity"))]
fn ThreePiSDME_32(name: &str, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(TwoPiSDME, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPiSDME_64, m)?)?;
    m.add_function(wrap_pyfunction!(TwoPiSDME_32, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME_64, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiSDME_32, m)?)?;
    Ok(())
}
