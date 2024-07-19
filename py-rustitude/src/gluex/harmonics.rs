use crate::amplitude::{Amplitude32, Amplitude64};
use pyo3::prelude::*;
use rustitude_gluex::harmonics as rust;
use rustitude_gluex::utils::{Frame, Reflectivity, Wave};

#[pyfunction]
#[pyo3(name = "Ylm64", signature = (name, l, m, frame="helicity"))]
fn ylm64(name: &str, l: usize, m: isize, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "Ylm32", signature = (name, l, m, frame="helicity"))]
fn ylm32(name: &str, l: usize, m: isize, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "Zlm64", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn zlm64(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "Zlm32", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn zlm32(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "OnePS64", signature = (name, reflectivity="positive", frame="helicity"))]
fn one_ps64(name: &str, reflectivity: &str, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "OnePS32", signature = (name, reflectivity="positive", frame="helicity"))]
fn one_ps32(name: &str, reflectivity: &str, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "TwoPS64", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn two_ps64(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude64 {
    Amplitude64::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(name = "TwoPS32", signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn two_ps32(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude32 {
    Amplitude32::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ylm64, m)?)?;
    m.add_function(wrap_pyfunction!(ylm32, m)?)?;
    m.add_function(wrap_pyfunction!(zlm64, m)?)?;
    m.add_function(wrap_pyfunction!(zlm32, m)?)?;
    m.add_function(wrap_pyfunction!(one_ps64, m)?)?;
    m.add_function(wrap_pyfunction!(one_ps32, m)?)?;
    m.add_function(wrap_pyfunction!(two_ps64, m)?)?;
    m.add_function(wrap_pyfunction!(two_ps32, m)?)?;
    Ok(())
}
