use crate::amplitude::Amplitude;
use pyo3::prelude::*;
use rustitude_gluex::sdmes as rust;
use rustitude_gluex::utils::Frame;

#[pyfunction]
#[pyo3(name = "TwoPiSDME", signature = (name, frame="helicity"))]
fn two_pi_sdme(name: &str, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::TwoPiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}
#[pyfunction]
#[pyo3(name = "ThreePiSDME", signature = (name, frame="helicity"))]
fn three_pi_sdme(name: &str, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::ThreePiSDME::new(<Frame as std::str::FromStr>::from_str(frame).unwrap()),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_pi_sdme, m)?)?;
    m.add_function(wrap_pyfunction!(three_pi_sdme, m)?)?;
    Ok(())
}
