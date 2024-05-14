use std::{collections::HashMap, fmt::Display, fs::File, path::Path, sync::Arc};

use itertools::izip;
use nalgebra::Vector3;
use num::Zero;
use oxyroot::{RootFile, Slice};
use parking_lot::RwLock;
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::{Field, Row},
};
use pyo3::prelude::*;
use rayon::prelude::*;

use crate::prelude::FourMomentum;

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct Event {
    #[pyo3(get)]
    pub index: usize,
    #[pyo3(get)]
    pub weight: f64,
    #[pyo3(get)]
    pub beam_p4: FourMomentum,
    #[pyo3(get)]
    pub recoil_p4: FourMomentum,
    #[pyo3(get)]
    pub daughter_p4s: Vec<FourMomentum>,
    pub eps: Vector3<f64>,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Index: {}", self.index)?;
        writeln!(f, "Weight: {}", self.weight)?;
        writeln!(f, "Beam P4: {}", self.beam_p4)?;
        writeln!(f, "Recoil P4: {}", self.recoil_p4)?;
        writeln!(f, "Daughters:")?;
        for (i, p4) in self.daughter_p4s.iter().enumerate() {
            writeln!(f, "\t{i} -> {p4}")?;
        }
        writeln!(
            f,
            "EPS: [{}, {}, {}]",
            self.eps[0], self.eps[1], self.eps[2]
        )?;
        Ok(())
    }
}
#[pymethods]
impl Event {
    pub fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[getter]
    fn get_eps(&self) -> PyResult<[f64; 3]> {
        Ok([self.eps[0], self.eps[1], self.eps[2]])
    }
}
impl Event {
    pub fn read_parquet_row(index: usize, row: Row) -> Self {
        let mut event = Self {
            index,
            ..Default::default()
        };
        let mut e_fs: Vec<f64> = Vec::new();
        let mut px_fs: Vec<f64> = Vec::new();
        let mut py_fs: Vec<f64> = Vec::new();
        let mut pz_fs: Vec<f64> = Vec::new();
        for (name, field) in row.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", Field::Float(value)) => {
                    event.beam_p4.set_e(f64::from(*value));
                }
                ("Px_Beam", Field::Float(value)) => {
                    event.beam_p4.set_px(f64::from(*value));
                }
                ("Py_Beam", Field::Float(value)) => {
                    event.beam_p4.set_py(f64::from(*value));
                }
                ("Pz_Beam", Field::Float(value)) => {
                    event.beam_p4.set_pz(f64::from(*value));
                }
                ("Weight", Field::Float(value)) => event.weight = f64::from(*value),
                ("EPS", Field::ListInternal(list)) => {
                    event.eps = Vector3::from_vec(
                        list.elements()
                            .iter()
                            .map(|field| {
                                if let Field::Float(value) = field {
                                    f64::from(*value)
                                } else {
                                    panic!()
                                }
                            })
                            .collect(),
                    );
                }
                ("E_FinalState", Field::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", Field::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", Field::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", Field::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                _ => {}
            }
        }
        event.recoil_p4 = FourMomentum::new(e_fs[0], px_fs[0], py_fs[0], pz_fs[0]);
        event.daughter_p4s = e_fs[1..]
            .iter()
            .zip(px_fs[1..].iter())
            .zip(py_fs[1..].iter())
            .zip(pz_fs[1..].iter())
            .map(|(((e, px), py), pz)| FourMomentum::new(*e, *px, *py, *pz))
            .collect();
        let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        for dp4 in event.daughter_p4s.iter_mut() {
            *dp4 = dp4.boost_along(&final_state_p4);
        }
        event
    }
    pub fn read_parquet_row_eps_in_beam(index: usize, row: Row) -> Self {
        let mut event = Self {
            index,
            ..Default::default()
        };
        let mut e_fs: Vec<f64> = Vec::new();
        let mut px_fs: Vec<f64> = Vec::new();
        let mut py_fs: Vec<f64> = Vec::new();
        let mut pz_fs: Vec<f64> = Vec::new();
        for (name, field) in row.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", Field::Float(value)) => {
                    event.beam_p4.set_e(f64::from(*value));
                }
                ("Px_Beam", Field::Float(value)) => {
                    event.eps[0] = f64::from(*value);
                }
                ("Py_Beam", Field::Float(value)) => {
                    event.eps[1] = f64::from(*value);
                }
                ("Pz_Beam", Field::Float(value)) => {
                    event.beam_p4.set_pz(f64::from(*value));
                }
                ("Weight", Field::Float(value)) => event.weight = f64::from(*value),
                ("E_FinalState", Field::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", Field::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", Field::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", Field::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                _ => {}
            }
        }
        event.recoil_p4 = FourMomentum::new(e_fs[0], px_fs[0], py_fs[0], pz_fs[0]);
        event.daughter_p4s = e_fs[1..]
            .iter()
            .zip(px_fs[1..].iter())
            .zip(py_fs[1..].iter())
            .zip(pz_fs[1..].iter())
            .map(|(((e, px), py), pz)| FourMomentum::new(*e, *px, *py, *pz))
            .collect();
        let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        for dp4 in event.daughter_p4s.iter_mut() {
            *dp4 = dp4.boost_along(&final_state_p4);
        }
        event
    }

    pub fn read_parquet_row_with_eps(index: usize, row: Row, eps: Vector3<f64>) -> Self {
        let mut event = Self {
            index,
            eps,
            ..Default::default()
        };
        let mut e_fs: Vec<f64> = Vec::new();
        let mut px_fs: Vec<f64> = Vec::new();
        let mut py_fs: Vec<f64> = Vec::new();
        let mut pz_fs: Vec<f64> = Vec::new();
        for (name, field) in row.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", Field::Float(value)) => {
                    event.beam_p4.set_e(f64::from(*value));
                }
                ("Px_Beam", Field::Float(value)) => {
                    event.beam_p4.set_px(f64::from(*value));
                }
                ("Py_Beam", Field::Float(value)) => {
                    event.beam_p4.set_py(f64::from(*value));
                }
                ("Pz_Beam", Field::Float(value)) => {
                    event.beam_p4.set_pz(f64::from(*value));
                }
                ("Weight", Field::Float(value)) => event.weight = f64::from(*value),
                ("E_FinalState", Field::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", Field::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", Field::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", Field::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let Field::Float(value) = field {
                                f64::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                _ => {}
            }
        }
        event.recoil_p4 = FourMomentum::new(e_fs[0], px_fs[0], py_fs[0], pz_fs[0]);
        event.daughter_p4s = e_fs[1..]
            .iter()
            .zip(px_fs[1..].iter())
            .zip(py_fs[1..].iter())
            .zip(pz_fs[1..].iter())
            .map(|(((e, px), py), pz)| FourMomentum::new(*e, *px, *py, *pz))
            .collect();
        let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        for dp4 in event.daughter_p4s.iter_mut() {
            *dp4 = dp4.boost_along(&final_state_p4);
        }
        event
    }

    pub fn read_parquet_row_unpolarized(index: usize, row: Row) -> Self {
        Self::read_parquet_row_with_eps(index, row, Vector3::default())
    }
}

#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct Dataset {
    pub events: Arc<RwLock<Vec<Event>>>,
}

#[pymethods]
impl Dataset {
    pub fn events(&self) -> Vec<Event> {
        self.events.read().clone()
    }
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.len())
    }

    pub fn __getitem__(&self, idx: isize) -> PyResult<Py<Event>> {
        Ok(Python::with_gil(|py| Py::new(py, self.events.read()[idx as usize].clone())).unwrap())
    }

    pub fn weights(&self) -> Vec<f64> {
        self.events.read().iter().map(|e| e.weight).collect()
    }

    // TODO:
    // pub fn select(&mut self, query: impl Fn(&Event) -> bool + Sync + Send) -> PyDataset {}
    // pub fn reject(&mut self, query: impl Fn(&Event) -> bool + Sync + Send) -> PyDataset {}
    // pub fn split(
    //     mut self,
    //     variable: impl Fn(&Event) -> f64 + Sync + Send,
    //     range: (f64, f64),
    //     nbins: usize,
    // ) -> (Vec<PyDataset>, PyDataset, PyDataset) {}

    pub fn split_m(
        &self,
        range: (f64, f64),
        bins: usize,
        p1: Option<Vec<usize>>,
        p2: Option<Vec<usize>>,
    ) -> (Vec<Self>, Self, Self) {
        let mass = |e: &Event| {
            let p1_p4: FourMomentum = p1
                .clone()
                .unwrap_or_else(|| vec![0])
                .iter()
                .map(|i| &e.daughter_p4s[*i])
                .sum();
            let p2_p4 = p2
                .clone()
                .unwrap_or_else(|| vec![1])
                .iter()
                .map(|i| &e.daughter_p4s[*i])
                .sum::<FourMomentum>();
            (p1_p4 + p2_p4).m()
        };
        self.clone().split(mass, range, bins) // TODO: fix clone here eventually
    }

    #[staticmethod]
    pub fn from_events(events: Vec<Event>) -> PyResult<Self> {
        Ok(Self {
            events: Arc::new(RwLock::new(events)),
        })
    }

    #[staticmethod]
    pub fn from_dict(py: Python, data: HashMap<String, PyObject>) -> PyResult<Self> {
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
        Ok(Self::new(
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
                        Event {
                            index,
                            weight,
                            beam_p4: FourMomentum::new(e_beam, px_beam, py_beam, pz_beam),
                            recoil_p4: FourMomentum::new(
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
                                .map(|(((e, px), py), pz)| FourMomentum::new(*e, *px, *py, *pz))
                                .collect(),
                            eps,
                        }
                    },
                )
                .collect(),
        ))
    }

    #[staticmethod]
    pub fn from_parquet(path: &str) -> Self {
        let path = Path::new(path);
        let file = File::open(path).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let row_iter = reader.get_row_iter(None).unwrap();
        Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row(i, row.unwrap()))
                .collect(),
        )
    }

    #[staticmethod]
    pub fn from_parquet_eps_in_beam(path: &str) -> Self {
        let path = Path::new(path);
        let file = File::open(path).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let row_iter = reader.get_row_iter(None).unwrap();
        Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_eps_in_beam(i, row.unwrap()))
                .collect(),
        )
    }

    #[staticmethod]
    pub fn from_parquet_with_eps(path: &str, eps: Vec<f64>) -> Self {
        let path = Path::new(path);
        let file = File::open(path).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let row_iter = reader.get_row_iter(None).unwrap();
        let eps_vec = Vector3::from_vec(eps);
        Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_with_eps(i, row.unwrap(), eps_vec))
                .collect(),
        )
    }

    #[staticmethod]
    pub fn from_parquet_unpolarized(path: &str) -> Self {
        let path = Path::new(path);
        let file = File::open(path).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let row_iter = reader.get_row_iter(None).unwrap();
        Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_unpolarized(i, row.unwrap()))
                .collect(),
        )
    }

    #[staticmethod]
    pub fn from_root(path: &str) -> Self {
        let ttree = RootFile::open(path).unwrap().get_tree("kin").unwrap(); // TODO:
        let weight: Vec<f64> = ttree
            .branch("Weight")
            .unwrap()
            .as_iter::<f32>()
            .unwrap()
            .map(f64::from)
            .collect();
        let e_beam: Vec<f64> = ttree
            .branch("E_Beam")
            .unwrap()
            .as_iter::<f32>()
            .unwrap()
            .map(f64::from)
            .collect();
        let px_beam: Vec<f64> = ttree
            .branch("Px_Beam")
            .unwrap()
            .as_iter::<f32>()
            .unwrap()
            .map(f64::from)
            .collect();
        let py_beam: Vec<f64> = ttree
            .branch("Py_Beam")
            .unwrap()
            .as_iter::<f32>()
            .unwrap()
            .map(f64::from)
            .collect();
        let pz_beam: Vec<f64> = ttree
            .branch("Pz_Beam")
            .unwrap()
            .as_iter::<f32>()
            .unwrap()
            .map(f64::from)
            .collect();
        let e_fs: Vec<Vec<f64>> = ttree
            .branch("E_FinalState")
            .unwrap()
            .as_iter::<Slice<f64>>()
            .unwrap()
            .map(|v| v.into_vec())
            .collect();
        let px_fs: Vec<Vec<f64>> = ttree
            .branch("Px_FinalState")
            .unwrap()
            .as_iter::<Slice<f64>>()
            .unwrap()
            .map(|v| v.into_vec())
            .collect();
        let py_fs: Vec<Vec<f64>> = ttree
            .branch("Py_FinalState")
            .unwrap()
            .as_iter::<Slice<f64>>()
            .unwrap()
            .map(|v| v.into_vec())
            .collect();
        let pz_fs: Vec<Vec<f64>> = ttree
            .branch("Pz_FinalState")
            .unwrap()
            .as_iter::<Slice<f64>>()
            .unwrap()
            .map(|v| v.into_vec())
            .collect();
        Self::new(
            izip!(weight, e_beam, px_beam, py_beam, pz_beam, e_fs, px_fs, py_fs, pz_fs)
                .enumerate()
                .map(
                    |(i, (w, e_b, px_b, py_b, pz_b, e_f, px_f, py_f, pz_f))| Event {
                        index: i,
                        weight: w,
                        beam_p4: FourMomentum::new(e_b, px_b, py_b, pz_b),
                        recoil_p4: FourMomentum::new(e_f[0], px_f[0], py_f[0], pz_f[0]),
                        daughter_p4s: izip!(
                            e_f[1..].iter(),
                            px_f[1..].iter(),
                            py_f[1..].iter(),
                            pz_f[1..].iter()
                        )
                        .map(|(e, px, py, pz)| FourMomentum::new(*e, *px, *py, *pz))
                        .collect(),
                        eps: Vector3::zero(),
                    },
                )
                .collect(),
        )
    }
}

impl Dataset {
    pub fn new(events: Vec<Event>) -> Self {
        Self {
            events: Arc::new(RwLock::new(events)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.events.read().is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.read().len()
    }

    pub fn select(&mut self, query: impl Fn(&Event) -> bool + Sync + Send) -> Self {
        let (mut selected, mut rejected): (Vec<_>, Vec<_>) =
            self.events.write().par_drain(..).partition(query);
        selected
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, event)| event.index = i);
        rejected
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, event)| event.index = i);
        self.events = Arc::new(RwLock::new(selected));
        Self::new(rejected)
    }

    pub fn reject(&mut self, query: impl Fn(&Event) -> bool + Sync + Send) -> Self {
        self.select(|event| !query(event))
    }

    pub fn split(
        mut self,
        variable: impl Fn(&Event) -> f64 + Sync + Send,
        range: (f64, f64),
        nbins: usize,
    ) -> (Vec<Self>, Self, Self) {
        let mut bins: Vec<f64> = Vec::with_capacity(nbins + 1);
        let width = (range.1 - range.0) / nbins as f64;
        for m in 0..=nbins {
            bins.push(width.mul_add(m as f64, range.0));
        }
        let mut out: Vec<Self> = Vec::with_capacity(nbins);
        let underflow: Self = self.reject(|event: &Event| variable(event) < bins[0]);
        let overflow: Self = self.reject(|event: &Event| variable(event) > bins[bins.len() - 1]);
        // now the ends are trimmed off of self
        bins.into_iter().skip(1).for_each(|ub| {
            let bin_contents = self.reject(|event| variable(event) < ub);
            out.push(bin_contents);
        });
        (out, underflow, overflow)
    }
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Event>()?;
    m.add_class::<Dataset>()?;
    Ok(())
}
