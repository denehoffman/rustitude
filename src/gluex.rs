use crate::PyAmplitude;
use pyo3::prelude::*;
use rustitude_core::amplitude::Amplitude;
use rustitude_gluex::harmonics::{OnePS, TwoPS, Ylm, Zlm};
use rustitude_gluex::resonances::{
    BreitWigner, KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2, KMatrixPi1, KMatrixRho,
};
use rustitude_gluex::utils::{Frame, Part, Reflectivity, Wave};

#[pyfunction]
fn kmatrix_a0(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixA0::new(channel))))
}
#[pyfunction]
fn kmatrix_a2(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixA2::new(channel))))
}
#[pyfunction]
fn kmatrix_f0(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixF0::new(channel))))
}
#[pyfunction]
fn kmatrix_f2(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixF2::new(channel))))
}
#[pyfunction]
fn kmatrix_pi1(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixPi1::new(channel))))
}
#[pyfunction]
fn kmatrix_rho(name: &str, channel: usize) -> PyAmplitude {
    PyAmplitude(Amplitude::new(name, Box::new(KMatrixRho::new(channel))))
}
#[pyfunction]
fn breit_wigner(
    name: &str,
    p1_indices: Vec<usize>,
    p2_indices: Vec<usize>,
    l: usize,
) -> PyAmplitude {
    PyAmplitude(Amplitude::new(
        name,
        Box::new(BreitWigner::new(&p1_indices, &p2_indices, l)),
    ))
}

#[pymodule]
pub fn resonances(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kmatrix_a0, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_a2, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f0, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_f2, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_pi1, m)?)?;
    m.add_function(wrap_pyfunction!(kmatrix_rho, m)?)?;
    m.add_function(wrap_pyfunction!(breit_wigner, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (name, l, m, frame="helicity"))]
fn ylm(name: &str, l: usize, m: isize, frame: &str) -> PyAmplitude {
    PyAmplitude(Amplitude::new(
        name,
        Box::new(Ylm::new(
            Wave::new(l, m),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        )),
    ))
}

#[pyfunction]
#[pyo3(signature = (name, l, m, reflectivity="positive", part="real", frame="helicity"))]
fn zlm(name: &str, l: usize, m: isize, reflectivity: &str, part: &str, frame: &str) -> PyAmplitude {
    PyAmplitude(Amplitude::new(
        name,
        Box::new(Zlm::new(
            Wave::new(l, m),
            <Reflectivity as std::str::FromStr>::from_str(reflectivity).unwrap(),
            <Part as std::str::FromStr>::from_str(part).unwrap(),
            <Frame as std::str::FromStr>::from_str(frame).unwrap(),
        )),
    ))
}

#[pymodule]
pub fn harmonics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ylm, m)?)?;
    m.add_function(wrap_pyfunction!(zlm, m)?)?;
    Ok(())
}

#[pymodule]
pub fn gluex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let m_resonances = PyModule::new_bound(m.py(), "rustitude.gluex.resonances")?;
    resonances(&m_resonances)?;
    m.add("resonances", &m_resonances)?;
    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("rustitude.gluex.resonances", &m_resonances)?;
    let m_harmonics = PyModule::new_bound(m.py(), "rustitude.gluex.harmonics")?;
    harmonics(&m_harmonics)?;
    m.add("harmonics", &m_harmonics)?;
    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("rustitude.gluex.harmonics", &m_harmonics)?;
    Ok(())
}
