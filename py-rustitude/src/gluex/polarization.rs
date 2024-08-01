use crate::amplitude::{Amplitude_32, Amplitude_64};
use pyo3::prelude::*;
use rustitude_gluex::{
    polarization as rust,
    utils::{Decay, Sign},
};

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance=Decay::ThreeBodyDecay([0, 1, 2]), decay_isobar=Decay::default()))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac(
    name: &str,
    beam_pol: Sign,
    j_resonance: u32,
    p_resonance: Sign,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: Decay,
    decay_isobar: Decay,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::ThreePiPolFrac::new(
            beam_pol,
            j_resonance,
            p_resonance,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            decay_resonance,
            decay_isobar,
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance=Decay::ThreeBodyDecay([0, 1, 2]), decay_isobar=Decay::default()))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac_64(
    name: &str,
    beam_pol: Sign,
    j_resonance: u32,
    p_resonance: Sign,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: Decay,
    decay_isobar: Decay,
) -> Amplitude_64 {
    Amplitude_64::new(
        name,
        rust::ThreePiPolFrac::new(
            beam_pol,
            j_resonance,
            p_resonance,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            decay_resonance,
            decay_isobar,
        ),
    )
}

#[pyfunction]
#[pyo3(signature = (name, beam_pol, j_resonance, p_resonance, i_resonance, l_resonance, j_isobar, i_isobar, iz_daughters, decay_resonance=Decay::ThreeBodyDecay([0, 1, 2]), decay_isobar=Decay::default()))]
#[allow(clippy::too_many_arguments)]
fn ThreePiPolFrac_32(
    name: &str,
    beam_pol: Sign,
    j_resonance: u32,
    p_resonance: Sign,
    i_resonance: usize,
    l_resonance: u32,
    j_isobar: u32,
    i_isobar: usize,
    iz_daughters: [usize; 3],
    decay_resonance: Decay,
    decay_isobar: Decay,
) -> Amplitude_32 {
    Amplitude_32::new(
        name,
        rust::ThreePiPolFrac::new(
            beam_pol,
            j_resonance,
            p_resonance,
            i_resonance,
            l_resonance,
            j_isobar,
            i_isobar,
            iz_daughters,
            decay_resonance,
            decay_isobar,
        ),
    )
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ThreePiPolFrac, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiPolFrac_64, m)?)?;
    m.add_function(wrap_pyfunction!(ThreePiPolFrac_32, m)?)?;
    Ok(())
}
