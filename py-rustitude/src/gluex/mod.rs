use pyo3::prelude::*;
mod dalitz;
mod harmonics;
mod resonances;
mod sdmes;
use crate::add_submodule;

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_submodule(m, "rustitude.gluex.sdmes", sdmes::pyo3_module)?;
    add_submodule(m, "rustitude.gluex.resonances", resonances::pyo3_module)?;
    add_submodule(m, "rustitude.gluex.harmonics", harmonics::pyo3_module)?;
    add_submodule(m, "rustitude.gluex.dalitz", dalitz::pyo3_module)?;
    m.add_class::<rustitude_gluex::utils::Wave>()?;
    m.add_class::<rustitude_gluex::utils::Frame>()?;
    m.add_class::<rustitude_gluex::utils::Reflectivity>()?;
    m.add_class::<rustitude_gluex::utils::Decay>()?;
    Ok(())
}
