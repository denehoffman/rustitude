use crate::amplitude::{AmpOp, Amplitude};
use pyo3::prelude::*;
use rustitude_gluex::resonances as rust;

#[pyfunction(name = "BreitWigner")]
fn breit_wigner(name: &str, p1_indices: Vec<usize>, p2_indices: Vec<usize>, l: usize) -> AmpOp {
    Amplitude::new(name, rust::BreitWigner::new(&p1_indices, &p2_indices, l)).into()
}
#[pyfunction(name = "KMatrixA0")]
fn kmatrix_a0(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixA0::new(channel)).into()
}
#[pyfunction(name = "KMatrixA2")]
fn kmatrix_a2(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixA2::new(channel)).into()
}
#[pyfunction(name = "KMatrixF0")]
fn kmatrix_f0(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixF0::new(channel)).into()
}
#[pyfunction(name = "KMatrixF2")]
fn kmatrix_f2(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixF2::new(channel)).into()
}
#[pyfunction(name = "KMatrixPi1")]
fn kmatrix_pi1(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixPi1::new(channel)).into()
}
#[pyfunction(name = "KMatrixRho")]
fn kmatrix_rho(name: &str, channel: usize) -> AmpOp {
    Amplitude::new(name, rust::KMatrixRho::new(channel)).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(breit_wigner, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a0, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a2, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f0, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f2, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_pi1, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_rho, m)?)?;
    Ok(())
}
