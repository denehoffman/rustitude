use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::sdmes as rust;
use rustitude_gluex::utils::{Decay, Frame};

#[pyfunction]
#[pyo3(signature = (name, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPiSDME(name: &str, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::TwoPiSDME::new(decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPiSDME_64(name: &str, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::TwoPiSDME::new(decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::default(), frame=Frame::Helicity))]
fn TwoPiSDME_32(name: &str, decay: Decay, frame: Frame) -> Amplitude_32 {
    Amplitude_32::new(name, rust::TwoPiSDME::new(decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2]), frame=Frame::Helicity))]
fn ThreePiSDME(name: &str, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::ThreePiSDME::new(decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2]), frame=Frame::Helicity))]
fn ThreePiSDME_64(name: &str, decay: Decay, frame: Frame) -> Amplitude_64 {
    Amplitude_64::new(name, rust::ThreePiSDME::new(decay, frame))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2]), frame=Frame::Helicity))]
fn ThreePiSDME_32(name: &str, decay: Decay, frame: Frame) -> Amplitude_32 {
    Amplitude_32::new(name, rust::ThreePiSDME::new(decay, frame))
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
