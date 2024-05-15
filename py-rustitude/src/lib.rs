use pyo3::prelude::*;

pub fn add_submodule<F>(parent: &Bound<'_, PyModule>, name: &str, mod_init: F) -> PyResult<()>
where
    F: Fn(&Bound<'_, PyModule>) -> PyResult<()>,
{
    let child_module = PyModule::new_bound(parent.py(), name)?;
    mod_init(&child_module)?;
    parent.add(name.split('.').last().unwrap(), &child_module)?;
    parent
        .py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item(name, &child_module)?;
    Ok(())
}

fn gluex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_submodule(m, "gluex.sdmes", rustitude_gluex::sdmes::pyo3_module)?;
    add_submodule(
        m,
        "gluex.resonances",
        rustitude_gluex::resonances::pyo3_module,
    )?;
    add_submodule(
        m,
        "gluex.harmonics",
        rustitude_gluex::harmonics::pyo3_module,
    )?;
    add_submodule(m, "gluex.dalitz", rustitude_gluex::dalitz::pyo3_module)?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "_rustitude")]
fn rustitude(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_submodule(m, "rustitude.dataset", rustitude_core::dataset::pyo3_module)?;
    add_submodule(
        m,
        "rustitude.four_momentum",
        rustitude_core::four_momentum::pyo3_module,
    )?;
    add_submodule(
        m,
        "rustitude.amplitude",
        rustitude_core::amplitude::pyo3_module,
    )?;
    add_submodule(m, "rustitude.manager", rustitude_core::manager::pyo3_module)?;

    add_submodule(m, "rustitude.gluex", gluex)?;
    Ok(())
}
