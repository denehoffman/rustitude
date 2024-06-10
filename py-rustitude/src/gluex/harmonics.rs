use crate::amplitude::Amplitude;
use pyo3::prelude::*;
use rustitude_gluex::harmonics as rust;
use rustitude_gluex::utils::{Frame, Reflectivity, Wave};

#[pyfunction]
#[pyo3(name = "Ylm", signature = (name, l, m, frame="helicity"))]
fn ylm(name: &str, l: usize, m: isize, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "Zlm", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn zlm(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "OnePS", signature = (name, reflectivity="positive", frame="helicity"))]
fn one_ps(name: &str, reflectivity: &str, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "TwoPS", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn two_ps(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude {
    Amplitude::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ylm, m)?)?;
    m.add_function(wrap_pyfunction!(zlm, m)?)?;
    m.add_function(wrap_pyfunction!(one_ps, m)?)?;
    m.add_function(wrap_pyfunction!(two_ps, m)?)?;
    Ok(())
}
