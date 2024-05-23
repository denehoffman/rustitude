use pyo3::prelude::*;
mod dalitz;
mod harmonics;
mod resonances;
mod sdmes;
use crate::add_submodule;

pub fn pyo3_moudle(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_submodule(m, "gluex.sdmes", sdmes::pyo3_module)?;
    add_submodule(m, "gluex.resonances", resonances::pyo3_module)?;
    add_submodule(m, "gluex.harmonics", harmonics::pyo3_module)?;
    add_submodule(m, "gluex.dalitz", dalitz::pyo3_module)?;
    Ok(())
}
