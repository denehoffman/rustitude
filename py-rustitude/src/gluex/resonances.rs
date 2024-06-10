use crate::amplitude::Amplitude;
use pyo3::prelude::*;
use rustitude_gluex::resonances as rust;

#[pyfunction(name = "BreitWigner")]
fn breit_wigner(name: &str, p1_indices: Vec<usize>, p2_indices: Vec<usize>, l: usize) -> Amplitude {
    Amplitude::new(name, rust::BreitWigner::new(&p1_indices, &p2_indices, l))
}
#[pyfunction(name = "KMatrixA0")]
fn kmatrix_a0(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixA0::new(channel))
}
#[pyfunction(name = "KMatrixA2")]
fn kmatrix_a2(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixA2::new(channel))
}
#[pyfunction(name = "KMatrixF0")]
fn kmatrix_f0(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixF0::new(channel))
}
#[pyfunction(name = "KMatrixF2")]
fn kmatrix_f2(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixF2::new(channel))
}
#[pyfunction(name = "KMatrixPi1")]
fn kmatrix_pi1(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixPi1::new(channel))
}
#[pyfunction(name = "KMatrixRho")]
fn kmatrix_rho(name: &str, channel: usize) -> Amplitude {
    Amplitude::new(name, rust::KMatrixRho::new(channel))
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
