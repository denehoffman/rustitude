#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use pyo3::prelude::*;
mod amplitude;
mod dataset;
mod four_momentum;
mod gluex;
mod manager;

#[macro_export]
macro_rules! impl_convert {
    ($a:ty, $b:ty) => {
        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                Self(value)
            }
        }
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                value.0
            }
        }
    };
}

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

#[pymodule]
#[pyo3(name = "_rustitude")]
fn rustitude(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    add_submodule(m, "rustitude.dataset", dataset::pyo3_module)?;
    add_submodule(m, "rustitude.four_momentum", four_momentum::pyo3_module)?;
    add_submodule(m, "rustitude.amplitude", amplitude::pyo3_module)?;
    add_submodule(m, "rustitude.manager", manager::pyo3_module)?;

    add_submodule(m, "rustitude.gluex", gluex::pyo3_module)?;
    Ok(())
}
