use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::{dalitz as rust, utils::Decay};

#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2])))]
fn OmegaDalitz(name: &str, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OmegaDalitz::new(decay))
}

#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2])))]
fn OmegaDalitz_64(name: &str, decay: Decay) -> Amplitude_64 {
    Amplitude_64::new(name, rust::OmegaDalitz::new(decay))
}
#[pyfunction]
#[pyo3(signature = (name, decay=Decay::ThreeBodyDecay([0, 1, 2])))]
fn OmegaDalitz_32(name: &str, decay: Decay) -> Amplitude_32 {
    Amplitude_32::new(name, rust::OmegaDalitz::new(decay))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(OmegaDalitz, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_64, m)?)?;
    m.add_function(wrap_pyfunction!(OmegaDalitz_32, m)?)?;
    Ok(())
}
