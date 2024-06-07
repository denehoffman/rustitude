use crate::amplitude::Amplitude;
use pyo3::prelude::*;
use rustitude_gluex::dalitz as rust;

#[pyfunction(name = "OmegaDalitz")]
fn omega_dalitz(name: &str) -> Amplitude {
    Amplitude::new(name, rust::OmegaDalitz::default())
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(omega_dalitz, m)?)?;
    Ok(())
}
