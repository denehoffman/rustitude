use crate::amplitude::{Amplitude32, Amplitude64};
use pyo3::prelude::*;
use rustitude_gluex::resonances as rust;

#[pyfunction(name = "BreitWigner64")]
fn breit_wigner64(
    name: &str,
    p1_indices: Vec<usize>,
    p2_indices: Vec<usize>,
    l: usize,
) -> Amplitude64 {
    Amplitude64::new(name, rust::BreitWigner::new(&p1_indices, &p2_indices, l))
}
#[pyfunction(name = "BreitWigner32")]
fn breit_wigner32(
    name: &str,
    p1_indices: Vec<usize>,
    p2_indices: Vec<usize>,
    l: usize,
) -> Amplitude32 {
    Amplitude32::new(name, rust::BreitWigner::new(&p1_indices, &p2_indices, l))
}
#[pyfunction(name = "KMatrixA064")]
fn kmatrix_a064(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixA0::new(channel))
}
#[pyfunction(name = "KMatrixA032")]
fn kmatrix_a032(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixA0::new(channel))
}
#[pyfunction(name = "KMatrixA264")]
fn kmatrix_a264(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixA2::new(channel))
}
#[pyfunction(name = "KMatrixA232")]
fn kmatrix_a232(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixA2::new(channel))
}
#[pyfunction(name = "KMatrixF064")]
fn kmatrix_f064(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixF0::new(channel))
}
#[pyfunction(name = "KMatrixF032")]
fn kmatrix_f032(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixF0::new(channel))
}
#[pyfunction(name = "KMatrixF264")]
fn kmatrix_f264(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixF2::new(channel))
}
#[pyfunction(name = "KMatrixF232")]
fn kmatrix_f232(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixF2::new(channel))
}
#[pyfunction(name = "KMatrixPi164")]
fn kmatrix_pi164(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixPi1::new(channel))
}
#[pyfunction(name = "KMatrixPi132")]
fn kmatrix_pi132(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixPi1::new(channel))
}
#[pyfunction(name = "KMatrixRho64")]
fn kmatrix_rho64(name: &str, channel: usize) -> Amplitude64 {
    Amplitude64::new(name, rust::KMatrixRho::new(channel))
}
#[pyfunction(name = "KMatrixRho32")]
fn kmatrix_rho32(name: &str, channel: usize) -> Amplitude32 {
    Amplitude32::new(name, rust::KMatrixRho::new(channel))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(breit_wigner64, m)?)?;
    m.add_function(wrap_pyfunction!(breit_wigner32, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a064, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a032, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a264, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a232, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f064, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f032, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f264, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f232, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_pi164, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_pi132, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_rho64, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_rho32, m)?)?;
    Ok(())
}
