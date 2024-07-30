use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::harmonics as rust;
use rustitude_gluex::utils::{Decay, Frame, Reflectivity, Wave};

#[pyfunction]
#[pyo3(signature = (name, l, m, decay=Decay::default(), frame=Frame::Helicity))]
fn Ylm(name: &str, l: usize, m: isize, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::Ylm::new(Wave::new(l, m), decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, l, m, decay=Decay::default(), frame=Frame::Helicity))]
fn Ylm_64(name: &str, l: usize, m: isize, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::Ylm::new(Wave::new(l, m), decay, frame))
}

#[pyfunction]
#[pyo3(signature = (name, l, m, decay=Decay::default(), frame=Frame::Helicity))]
fn Ylm_32(name: &str, l: usize, m: isize, decay: Decay, frame: Frame) -> Amplitude_32 {
    Amplitude_32::new(name, rust::Ylm::new(Wave::new(l, m), decay, frame))
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn Zlm(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Zlm::new(Wave::new(l, m), reflectivity, decay, frame),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn Zlm_64(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::Zlm::new(Wave::new(l, m), reflectivity, decay, frame),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn Zlm_32(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::Zlm::new(Wave::new(l, m), reflectivity, decay, frame),
    )
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn OnePS(name: &str, reflectivity: Reflectivity, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OnePS::new(reflectivity, decay, frame))
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn OnePS_64(name: &str, reflectivity: Reflectivity, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OnePS::new(reflectivity, decay, frame))
}

#[pyfunction]
#[pyo3(signature = (name, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn OnePS_32(name: &str, reflectivity: Reflectivity, decay: Decay, frame: Frame) -> Amplitude_32 {
    Amplitude_32::new(name, rust::OnePS::new(reflectivity, decay, frame))
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPS(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPS::new(Wave::new(l, m), reflectivity, decay, frame),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPS_64(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::TwoPS::new(Wave::new(l, m), reflectivity, decay, frame),
    )
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity=Reflectivity::Positive, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPS_32(
    name: &str,
    l: usize,
    m: isize,
    reflectivity: Reflectivity,
    decay: Decay,
    frame: Frame,
) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::TwoPS::new(Wave::new(l, m), reflectivity, decay, frame),
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
