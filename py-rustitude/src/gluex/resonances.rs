use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::{resonances as rust, utils::Decay};

#[pyfunction]
#[pyo3(signature = (name, l, decay=Decay::default()))]
fn BreitWigner(name: &str, l: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::BreitWigner::new(l, decay))
}
#[pyfunction]
#[pyo3(signature = (name, l, decay=Decay::default()))]
fn BreitWigner_64(name: &str, l: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::BreitWigner::new(l, decay))
}
#[pyfunction]
#[pyo3(signature = (name, l, decay=Decay::default()))]
fn BreitWigner_32(name: &str, l: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::BreitWigner::new(l, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA0(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixA0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA0_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixA0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA0_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixA0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA2(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixA2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA2_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixA2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixA2_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixA2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF0(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixF0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF0_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixF0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF0_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixF0::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF2(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixF2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF2_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixF2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixF2_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixF2::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixPi1(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixPi1::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixPi1_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixPi1::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixPi1_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixPi1::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixRho(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixRho::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixRho_64(name: &str, channel: usize, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::KMatrixRho::new(channel, decay))
}
#[pyfunction]
#[pyo3(signature = (name, channel, decay=Decay::default()))]
fn KMatrixRho_32(name: &str, channel: usize, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::KMatrixRho::new(channel, decay))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(BreitWigner, m)?)?;
    m.add_function(wrap_pyfunction!(BreitWigner_64, m)?)?;
    m.add_function(wrap_pyfunction!(BreitWigner_32, m)?)?;
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
