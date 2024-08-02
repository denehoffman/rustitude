use std::str::FromStr;

use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude::prelude::RustitudeError;
use rustitude_gluex::{
    polarization as rust,
    utils::{Decay, Sign},
};

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance="[0, 1, 2]", decay_isobar="[0, 1]"))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac(
    name: &str,
    beam_pol: &str,
    j_resonance: u32,
    p_resonance: &str,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: &str,
    decay_isobar: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::ThreePiPolFrac::new(
            Sign::from_str(beam_pol)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            j_resonance,
            Sign::from_str(p_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            Decay::from_str(decay_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Decay::from_str(decay_isobar)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance="[0, 1, 2]", decay_isobar="[0, 1]"))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac_64(
    name: &str,
    beam_pol: &str,
    j_resonance: u32,
    p_resonance: &str,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: &str,
    decay_isobar: &str,
) -> PyResult<Amplitude_64> {
    Ok(Amplitude_64::new(
        name,
        rust::ThreePiPolFrac::new(
            Sign::from_str(beam_pol)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            j_resonance,
            Sign::from_str(p_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            Decay::from_str(decay_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Decay::from_str(decay_isobar)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance="[0, 1, 2]", decay_isobar="[0, 1]"))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac_32(
    name: &str,
    beam_pol: &str,
    j_resonance: u32,
    p_resonance: &str,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: &str,
    decay_isobar: &str,
) -> PyResult<Amplitude_32> {
    Ok(Amplitude_32::new(
        name,
        rust::ThreePiPolFrac::new(
            Sign::from_str(beam_pol)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            j_resonance,
            Sign::from_str(p_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            Decay::from_str(decay_resonance)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
            Decay::from_str(decay_isobar)
                .map_err(RustitudeError::from)
                .map_err(PyErr::from)?,
        ),
    ))
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ThreePiPolFrac, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiPolFrac_64, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiPolFrac_32, m)?)?;
    Ok(())
}
