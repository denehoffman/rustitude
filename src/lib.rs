use pyo3::prelude::*;

#[pymodule]
fn rustitude(m: &Bound<'_, PyModule>) -> PyResult<()> {
    rustitude_core::dataset::register_module(m)?;
    rustitude_core::four_momentum::register_module(m)?;
    rustitude_core::amplitude::register_module(m)?;
    rustitude_core::manager::register_module(m)?;

    #[cfg(feature = "gluex")]
    rustitude_gluex::register_module(m)?;

    Ok(())
}
