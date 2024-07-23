use crate::four_momentum::{FourMomentum_32, FourMomentum_64};
use crate::impl_convert;
use nalgebra::Vector3;
use pyo3::prelude::*;
use rayon::prelude::*;
use rustitude_core::dataset as rust;
use rustitude_core::four_momentum as rust_fm;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct Event_64(rust::Event<f64>);
impl_convert!(Event_64, rust::Event<f64>);

#[pymethods]
impl Event_64 {
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }
    #[getter]
    fn weight(&self) -> f64 {
        self.0.weight
    }
    #[getter]
    fn beam_p4(&self) -> FourMomentum_64 {
        self.0.beam_p4.into()
    }
    #[getter]
    fn recoil_p4(&self) -> FourMomentum_64 {
        self.0.recoil_p4.into()
    }
    #[getter]
    fn daughter_p4s(&self) -> Vec<FourMomentum_64> {
        self.0
            .daughter_p4s
            .clone()
            .into_iter()
            .map(FourMomentum_64::from)
            .collect()
    }
    #[getter]
    fn eps(&self) -> [f64; 3] {
        [self.0.eps[0], self.0.eps[1], self.0.eps[2]]
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct Event_32(rust::Event<f32>);
impl_convert!(Event_32, rust::Event<f32>);

#[pymethods]
impl Event_32 {
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }
    #[getter]
    fn weight(&self) -> f32 {
        self.0.weight
    }
    #[getter]
    fn beam_p4(&self) -> FourMomentum_32 {
        self.0.beam_p4.into()
    }
    #[getter]
    fn recoil_p4(&self) -> FourMomentum_32 {
        self.0.recoil_p4.into()
    }
    #[getter]
    fn daughter_p4s(&self) -> Vec<FourMomentum_32> {
        self.0
            .daughter_p4s
            .clone()
            .into_iter()
            .map(FourMomentum_32::from)
            .collect()
    }
    #[getter]
    fn eps(&self) -> [f32; 3] {
        [self.0.eps[0], self.0.eps[1], self.0.eps[2]]
    }
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct Dataset_64(rust::Dataset<f64>);
impl_convert!(Dataset_64, rust::Dataset<f64>);

impl From<&rust::Dataset<f64>> for Dataset_64 {
    fn from(dataset: &rust::Dataset<f64>) -> Self {
        Dataset_64(dataset.clone())
    }
}

#[pymethods]
impl Dataset_64 {
    #[getter]
    fn events(&self) -> Vec<Event_64> {
        self.0.events.iter().cloned().map(Event_64::from).collect()
    }
    #[getter]
    fn weights(&self) -> Vec<f64> {
        self.0.weights()
    }
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Py<Event_64>> {
        Ok(Python::with_gil(|py| Py::new(py, self.events()[idx as usize].clone())).unwrap())
    }

    fn __add__(&self, other: Dataset_64) -> Dataset_64 {
        (self.0.clone() + other.0).into()
    }

    #[pyo3(signature = (range, bins, daughter_indices=None))]
    fn split_m(
        &self,
        range: (f64, f64),
        bins: usize,
        daughter_indices: Option<Vec<usize>>,
    ) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        self.0.split_m(range, bins, daughter_indices)
    }

    fn get_bootstrap_indices(&self, seed: usize) -> Vec<usize> {
        self.0.get_bootstrap_indices(seed)
    }

    #[staticmethod]
    fn from_events(events: Vec<Event_64>) -> Self {
        rust::Dataset::new(events.into_iter().map(rust::Event::from).collect()).into()
    }

    #[staticmethod]
    fn from_dict(py: Python, data: HashMap<String, PyObject>) -> PyResult<Self> {
        let e_beam_vec: Vec<f64> = data["E_Beam"].extract(py)?;
        let px_beam_vec: Vec<f64> = data["Px_Beam"].extract(py)?;
        let py_beam_vec: Vec<f64> = data["Py_Beam"].extract(py)?;
        let pz_beam_vec: Vec<f64> = data["Pz_Beam"].extract(py)?;
        let weight_vec: Vec<f64> = data
            .get("Weight")
            .map_or_else(|| Ok(vec![1.0; e_beam_vec.len()]), |obj| obj.extract(py))?;
        let eps_vec: Vec<Vector3<f64>> = data.get("EPS").map_or_else(
            || Ok(vec![Vector3::default(); e_beam_vec.len()]),
            |obj| {
                obj.extract::<Vec<Vec<f64>>>(py).map(|vvf: Vec<Vec<f64>>| {
                    vvf.into_iter()
                        .map(Vector3::from_vec)
                        .collect::<Vec<Vector3<f64>>>()
                })
            },
        )?;
        let e_finalstate_vec: Vec<Vec<f64>> = data["E_FinalState"].extract(py)?;
        let px_finalstate_vec: Vec<Vec<f64>> = data["Px_FinalState"].extract(py)?;
        let py_finalstate_vec: Vec<Vec<f64>> = data["Py_FinalState"].extract(py)?;
        let pz_finalstate_vec: Vec<Vec<f64>> = data["Pz_FinalState"].extract(py)?;
        Ok(Self(rust::Dataset::new(
            (
                e_beam_vec,
                px_beam_vec,
                py_beam_vec,
                pz_beam_vec,
                weight_vec,
                eps_vec,
                e_finalstate_vec,
                px_finalstate_vec,
                py_finalstate_vec,
                pz_finalstate_vec,
            )
                .into_par_iter()
                .enumerate()
                .map(
                    |(
                        index,
                        (
                            e_beam,
                            px_beam,
                            py_beam,
                            pz_beam,
                            weight,
                            eps,
                            e_finalstate,
                            px_finalstate,
                            py_finalstate,
                            pz_finalstate,
                        ),
                    )| {
                        rust::Event {
                            index,
                            weight,
                            beam_p4: rust_fm::FourMomentum::new(e_beam, px_beam, py_beam, pz_beam),
                            recoil_p4: rust_fm::FourMomentum::new(
                                e_finalstate[0],
                                px_finalstate[0],
                                py_finalstate[0],
                                pz_finalstate[0],
                            ),
                            daughter_p4s: e_finalstate[1..]
                                .iter()
                                .zip(px_finalstate[1..].iter())
                                .zip(py_finalstate[1..].iter())
                                .zip(pz_finalstate[1..].iter())
                                .map(|(((e, px), py), pz)| {
                                    rust_fm::FourMomentum::new(*e, *px, *py, *pz)
                                })
                                .collect(),
                            eps,
                        }
                    },
                )
                .collect(),
        )))
    }

    #[staticmethod]
    fn from_parquet(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::Standard)
            .map(Dataset_64::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_eps_in_beam(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPSInBeam)
            .map(Dataset_64::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_with_eps(path: &str, eps: Vec<f64>) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPS(eps[0], eps[1], eps[2]))
            .map(Dataset_64::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_unpolarized(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPS(0.0, 0.0, 0.0))
            .map(Dataset_64::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_root(path: &str) -> PyResult<Self> {
        rust::Dataset::from_root(path, rust::ReadMethod::Standard)
            .map(Dataset_64::from)
            .map_err(PyErr::from)
    }
}

#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct Dataset_32(rust::Dataset<f32>);
impl_convert!(Dataset_32, rust::Dataset<f32>);

impl From<&rust::Dataset<f32>> for Dataset_32 {
    fn from(dataset: &rust::Dataset<f32>) -> Self {
        Dataset_32(dataset.clone())
    }
}

#[pymethods]
impl Dataset_32 {
    #[getter]
    fn events(&self) -> Vec<Event_32> {
        self.0.events.iter().cloned().map(Event_32::from).collect()
    }
    #[getter]
    fn weights(&self) -> Vec<f32> {
        self.0.weights()
    }
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Py<Event_32>> {
        Ok(Python::with_gil(|py| Py::new(py, self.events()[idx as usize].clone())).unwrap())
    }

    fn __add__(&self, other: Dataset_32) -> Dataset_32 {
        (self.0.clone() + other.0).into()
    }

    #[pyo3(signature = (range, bins, daughter_indices=None))]
    fn split_m(
        &self,
        range: (f32, f32),
        bins: usize,
        daughter_indices: Option<Vec<usize>>,
    ) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        self.0.split_m(range, bins, daughter_indices)
    }

    fn get_bootstrap_indices(&self, seed: usize) -> Vec<usize> {
        self.0.get_bootstrap_indices(seed)
    }

    #[staticmethod]
    fn from_events(events: Vec<Event_32>) -> Self {
        rust::Dataset::new(events.into_iter().map(rust::Event::from).collect()).into()
    }

    #[staticmethod]
    fn from_dict(py: Python, data: HashMap<String, PyObject>) -> PyResult<Self> {
        let e_beam_vec: Vec<f32> = data["E_Beam"].extract(py)?;
        let px_beam_vec: Vec<f32> = data["Px_Beam"].extract(py)?;
        let py_beam_vec: Vec<f32> = data["Py_Beam"].extract(py)?;
        let pz_beam_vec: Vec<f32> = data["Pz_Beam"].extract(py)?;
        let weight_vec: Vec<f32> = data
            .get("Weight")
            .map_or_else(|| Ok(vec![1.0; e_beam_vec.len()]), |obj| obj.extract(py))?;
        let eps_vec: Vec<Vector3<f32>> = data.get("EPS").map_or_else(
            || Ok(vec![Vector3::default(); e_beam_vec.len()]),
            |obj| {
                obj.extract::<Vec<Vec<f32>>>(py).map(|vvf: Vec<Vec<f32>>| {
                    vvf.into_iter()
                        .map(Vector3::from_vec)
                        .collect::<Vec<Vector3<f32>>>()
                })
            },
        )?;
        let e_finalstate_vec: Vec<Vec<f32>> = data["E_FinalState"].extract(py)?;
        let px_finalstate_vec: Vec<Vec<f32>> = data["Px_FinalState"].extract(py)?;
        let py_finalstate_vec: Vec<Vec<f32>> = data["Py_FinalState"].extract(py)?;
        let pz_finalstate_vec: Vec<Vec<f32>> = data["Pz_FinalState"].extract(py)?;
        Ok(Self(rust::Dataset::new(
            (
                e_beam_vec,
                px_beam_vec,
                py_beam_vec,
                pz_beam_vec,
                weight_vec,
                eps_vec,
                e_finalstate_vec,
                px_finalstate_vec,
                py_finalstate_vec,
                pz_finalstate_vec,
            )
                .into_par_iter()
                .enumerate()
                .map(
                    |(
                        index,
                        (
                            e_beam,
                            px_beam,
                            py_beam,
                            pz_beam,
                            weight,
                            eps,
                            e_finalstate,
                            px_finalstate,
                            py_finalstate,
                            pz_finalstate,
                        ),
                    )| {
                        rust::Event {
                            index,
                            weight,
                            beam_p4: rust_fm::FourMomentum::new(e_beam, px_beam, py_beam, pz_beam),
                            recoil_p4: rust_fm::FourMomentum::new(
                                e_finalstate[0],
                                px_finalstate[0],
                                py_finalstate[0],
                                pz_finalstate[0],
                            ),
                            daughter_p4s: e_finalstate[1..]
                                .iter()
                                .zip(px_finalstate[1..].iter())
                                .zip(py_finalstate[1..].iter())
                                .zip(pz_finalstate[1..].iter())
                                .map(|(((e, px), py), pz)| {
                                    rust_fm::FourMomentum::new(*e, *px, *py, *pz)
                                })
                                .collect(),
                            eps,
                        }
                    },
                )
                .collect(),
        )))
    }
    #[staticmethod]
    fn from_parquet(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::Standard)
            .map(Dataset_32::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_eps_in_beam(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPSInBeam)
            .map(Dataset_32::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_with_eps(path: &str, eps: Vec<f32>) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPS(eps[0], eps[1], eps[2]))
            .map(Dataset_32::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_unpolarized(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet(path, rust::ReadMethod::EPS(0.0, 0.0, 0.0))
            .map(Dataset_32::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_root(path: &str) -> PyResult<Self> {
        rust::Dataset::from_root(path, rust::ReadMethod::Standard)
            .map(Dataset_32::from)
            .map_err(PyErr::from)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Event_64>()?;
    m.add_class::<Event_32>()?;
    m.add_class::<Dataset_64>()?;
    m.add_class::<Dataset_32>()?;
    Ok(())
}
