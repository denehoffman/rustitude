use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::harmonics as rust;
use rustitude_gluex::utils::{Frame, Reflectivity, Wave};

#[pyfunction]
#[pyo3(signature = (name, l, m, frame="helicity"))]
fn Ylm(name: &str, l: usize, m: isize, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}
#[pyfunction]
#[pyo3(signature = (name, l, m, frame="helicity"))]
fn Ylm_64(name: &str, l: usize, m: isize, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, frame="helicity"))]
fn Ylm_32(name: &str, l: usize, m: isize, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn Zlm(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn Zlm_64(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn Zlm_32(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity="positive", frame="helicity"))]
fn OnePS(name: &str, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity="positive", frame="helicity"))]
fn OnePS_64(name: &str, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity="positive", frame="helicity"))]
fn OnePS_32(name: &str, reflectivity: &str, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::OnePS::new(
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn TwoPS(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn TwoPS_64(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", frame="helicity"))]
fn TwoPS_32(name: &str, l: usize, m: isize, reflectivity: &str, frame: &str) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::TwoPS::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        ),
    )
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
