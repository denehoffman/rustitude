use crate::four_momentum::FourMomentum;
use nalgebra::Vector3;
use pyo3::prelude::*;
use rayon::prelude::*;
use rustitude_core::dataset as rust;
use rustitude_core::four_momentum as rust_fm;
use rustitude_core::Field;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct Event(rust::Event);

impl From<rust::Event> for Event {
    fn from(event: rust::Event) -> Self {
        Event(event)
    }
}
impl From<Event> for rust::Event {
    fn from(event: Event) -> Self {
        event.0
    }
}

#[pymethods]
impl Event {
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }
    #[getter]
    fn weight(&self) -> Field {
        self.0.weight
    }
    #[getter]
    fn beam_p4(&self) -> FourMomentum {
        self.0.beam_p4.into()
    }
    #[getter]
    fn recoil_p4(&self) -> FourMomentum {
        self.0.recoil_p4.into()
    }
    #[getter]
    fn daughter_p4s(&self) -> Vec<FourMomentum> {
        self.0
            .daughter_p4s
            .clone()
            .into_iter()
            .map(FourMomentum::from)
            .collect()
    }
    #[getter]
    fn eps(&self) -> [Field; 3] {
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
pub struct Dataset(rust::Dataset);

impl From<rust::Dataset> for Dataset {
    fn from(dataset: rust::Dataset) -> Self {
        Dataset(dataset)
    }
}
impl From<&rust::Dataset> for Dataset {
    fn from(dataset: &rust::Dataset) -> Self {
        Dataset(dataset.clone())
    }
}
impl From<Dataset> for rust::Dataset {
    fn from(dataset: Dataset) -> Self {
        dataset.0
    }
}

#[pymethods]
impl Dataset {
    #[getter]
    fn events(&self) -> Vec<Event> {
        self.0.events.iter().cloned().map(Event::from).collect()
    }
    #[getter]
    fn weights(&self) -> Vec<Field> {
        self.0.weights()
    }
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Py<Event>> {
        Ok(Python::with_gil(|py| Py::new(py, self.events()[idx as usize].clone())).unwrap())
    }

    #[pyo3(signature = (range, bins, daughter_indices=None))]
    fn split_m(
        &self,
        range: (Field, Field),
        bins: usize,
        daughter_indices: Option<Vec<usize>>,
    ) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        self.0.split_m(range, bins, daughter_indices)
    }

    fn get_bootstrap_indices(&self, seed: usize) -> Vec<usize> {
        self.0.get_bootstrap_indices(seed)
    }

    #[staticmethod]
    fn from_events(events: Vec<Event>) -> Self {
        rust::Dataset::new(events.into_iter().map(rust::Event::from).collect()).into()
    }

    #[staticmethod]
    fn from_dict(py: Python, data: HashMap<String, PyObject>) -> PyResult<Self> {
        let e_beam_vec: Vec<Field> = data["E_Beam"].extract(py)?;
        let px_beam_vec: Vec<Field> = data["Px_Beam"].extract(py)?;
        let py_beam_vec: Vec<Field> = data["Py_Beam"].extract(py)?;
        let pz_beam_vec: Vec<Field> = data["Pz_Beam"].extract(py)?;
        let weight_vec: Vec<Field> = data
            .get("Weight")
            .map_or_else(|| Ok(vec![1.0; e_beam_vec.len()]), |obj| obj.extract(py))?;
        let eps_vec: Vec<Vector3<Field>> = data.get("EPS").map_or_else(
            || Ok(vec![Vector3::default(); e_beam_vec.len()]),
            |obj| {
                obj.extract::<Vec<Vec<Field>>>(py)
                    .map(|vvf: Vec<Vec<Field>>| {
                        vvf.into_iter()
                            .map(Vector3::from_vec)
                            .collect::<Vec<Vector3<Field>>>()
                    })
            },
        )?;
        let e_finalstate_vec: Vec<Vec<Field>> = data["E_FinalState"].extract(py)?;
        let px_finalstate_vec: Vec<Vec<Field>> = data["Px_FinalState"].extract(py)?;
        let py_finalstate_vec: Vec<Vec<Field>> = data["Py_FinalState"].extract(py)?;
        let pz_finalstate_vec: Vec<Vec<Field>> = data["Pz_FinalState"].extract(py)?;
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
        rust::Dataset::from_parquet(path)
            .map(Dataset::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_eps_in_beam(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet_eps_in_beam(path)
            .map(Dataset::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_with_eps(path: &str, eps: Vec<Field>) -> PyResult<Self> {
        rust::Dataset::from_parquet_with_eps(path, eps)
            .map(Dataset::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_parquet_unpolarized(path: &str) -> PyResult<Self> {
        rust::Dataset::from_parquet_unpolarized(path)
            .map(Dataset::from)
            .map_err(PyErr::from)
    }
    #[staticmethod]
    fn from_root(path: &str) -> PyResult<Self> {
        rust::Dataset::from_root(path)
            .map(Dataset::from)
            .map_err(PyErr::from)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Event>()?;
    m.add_class::<Dataset>()?;
    Ok(())
}
