use crate::amplitude::{Amplitude32, Amplitude64};
use pyo3::prelude::*;
use rustitude_gluex::sdmes as rust;
use rustitude_gluex::utils::Frame;

#[pyfunction]
#[pyo3(name = "TwoPiSDME64", signature = (name, frame="helicity"))]
fn two_pi_sdme64(name: &str, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(name = "TwoPiSDME32", signature = (name, frame="helicity"))]
fn two_pi_sdme32(name: &str, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(name = "ThreePiSDME64", signature = (name, frame="helicity"))]
fn three_pi_sdme64(name: &str, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(name = "ThreePiSDME32", signature = (name, frame="helicity"))]
fn three_pi_sdme32(name: &str, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_pi_sdme64, m)?)?;
    m.add_function(wrap_pyfunction!(two_pi_sdme32, m)?)?;
    m.add_function(wrap_pyfunction!(three_pi_sdme64, m)?)?;
    m.add_function(wrap_pyfunction!(three_pi_sdme32, m)?)?;
    Ok(())
}
