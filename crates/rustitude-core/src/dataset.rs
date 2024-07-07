//! This module contains all the resources needed to load and examine datasets.
//!
//! A [`Dataset`] is, in essence, a list of [`Event`]s, each of which contain all the pertinent
//! information about a single set of initial- and final-state particles, as well as an index
//! and weight within the [`Dataset`].
//!
//! This crate currently supports loading [`Dataset`]s from ROOT and Parquet files (see
//! [`Dataset::from_root`] and [`Dataset::from_parquet`]. These methods require the following
//! "branches" or "columns" to be present in the file:
//!
//! | Branch Name | Data Type | Notes |
//! |---|---|---|
//! | `Weight` | Float32 |  |
//! | `E_Beam` | Float32 |  |
//! | `Px_Beam` | Float32 |  |
//! | `Py_Beam` | Float32 |  |
//! | `Pz_Beam` | Float32 |  |
//! | `E_FinalState` | \[Float32\] | \[recoil, daughter #1, daughter #2, ...\] |
//! | `Px_FinalState` | \[Float32\] | \[recoil, daughter #1, daughter #2, ...\] |
//! | `Py_FinalState` | \[Float32\] | \[recoil, daughter #1, daughter #2, ...\] |
//! | `Pz_FinalState` | \[Float32\] | \[recoil, daughter #1, daughter #2, ...\] |
//! | `EPS` | \[Float32\] | \[$`P_\gamma \cos(\Phi)`$, $`P_\gamma \sin(\Phi)`$, $`0.0`$\] for linear polarization with magnitude $`P_\gamma`$ and angle $`\Phi`$ |
//!
//! The `EPS` branch is optional and files without such a branch can be loaded under the
//! following conditions. First, if we don't care about polarization, and wish to set `EPS` =
//! `[0.0, 0.0, 0.0]`, we can do so using the methods [`Dataset::from_root_unpolarized`] or
//! [`Dataset::from_parquet_unpolarized`]. If a data file contains events with only one
//! polarization, we can compute the `EPS` vector ourselves and use
//! [`Dataset::from_root_with_eps`] or [`Dataset::from_parquet_with_eps`] to load the same vector
//! for every event. Finally, to provide compatibility with the way polarization is sometimes
//! included in `AmpTools` files, we can note that the beam is often only moving along the
//! $`z`$-axis, so the $`x`$ and $`y`$ components are typically `0.0` anyway, so we can store
//! the $`x`$ and $`y`$ components of `EPS` in the beam's four-momentum and use the methods
//! [`Dataset::from_root_eps_in_beam`] or [`Dataset::from_parquet_eps_in_beam`] to extract it.
//!
//! There are also several methods used to split up [`Dataset`]s based on their component
//! values. The [`Dataset::select`] method takes mutable access to a dataset along with a query
//! function which takes an [`Event`] and returns a [`bool`]. For each event, if the query
//! returns `true`, the event is removed from the original dataset and added to a new dataset
//! which is then returned by the `select` function. The [`Dataset::reject`] method does the
//! opposite. For example,
//!
//! ```ignore
//! let ds_original = Dataset::from_root("path.root").unwrap();
//! let ds_a = ds_original.clone();
//! let ds_b = ds_original.clone();
//! let mass_gt_1_gev = |e: &Event| -> bool {
//!     (e.daughter_p4s[0] + e.daughter_p4s[1]).m() > 1.0
//! };
//! let ds_a_selected = ds_a.select(mass_gt_1_gev);
//! let ds_b_rejected = ds_b.reject(mass_gt_1_gev);
//! ```
//!
//! After this, `ds_a` and `ds_b_rejected` will contain events where the four-momentum of the
//! first two daughter particles combined has a mass *less than* $`1.0`$ ``GeV``. On the other hand,
//! `ds_a_selected` and `ds_b` will have events where the opposite is true and the mass is
//! *greater than* $`1.0`$ ``GeV``. The reason for this logic is two-fold. First, we might be
//! dealing with large datasets, so we don't want to create copies of events if it can be
//! avoided. If copies are needed, they should be made explicitly with [`Dataset::clone`].
//! Otherwise, we just extract the events from the dataset. The other reason is that the syntax
//! reads in a "correct" way. We expect `let selected = data.select(condition);` to put the
//! selected data into the `selected` dataset. We can then choose if we want to hold on to the
//! rejected data.
//!
//! Since it is a common operation, there is also a method [`Dataset::split`] which will bin data
//! by a query which takes an [`Event`] and returns an [`Field`] value (rather than a [`bool`]).
//! This method also takes a `range: (Field, Field)` and a number of bins `nbins: usize`, and it
//! returns a `(Vec<Dataset>, Dataset, Dataset)`. These fields correspond to the binned datasets,
//! the underflow bin, and the overflow bin respectively, so no data should ever be "lost" by this
//! operation. There is also a convenience method, [`Dataset::split_m`], to split the dataset by
//! the mass of the summed four-momentum of any of the daughter particles, specified by their
//! index.
use std::{fmt::Display, fs::File, iter::repeat_with, path::Path, sync::Arc};

use itertools::{izip, Either, Itertools};
use nalgebra::Vector3;
use oxyroot::{ReaderTree, RootFile, Slice};
use parquet::record::Field as ParquetField;
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::Row,
};
use rayon::prelude::*;
use tracing::info;

use crate::{errors::RustitudeError, prelude::FourMomentum, Field};

/// The [`Event`] struct contains all the information concerning a single interaction between
/// particles in the experiment. See the individual fields for additional information.
#[derive(Debug, Default, Clone)]
pub struct Event {
    /// The index of the event with the parent [`Dataset`].
    pub index: usize,
    /// The weight of the event with the parent [`Dataset`].
    pub weight: Field,
    /// The beam [`FourMomentum`].
    pub beam_p4: FourMomentum,
    /// The recoil (target after interaction) [`FourMomentum`].
    pub recoil_p4: FourMomentum,
    /// [`FourMomentum`] of each other final state particle.
    pub daughter_p4s: Vec<FourMomentum>,
    /// A vector corresponding to the polarization of the beam.
    pub eps: Vector3<Field>,
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
impl Event {
    /// Reads an [`Event`] from a single [`Row`] in a Parquet file.
    ///
    /// # Panics
    ///
    /// This method currently panics if the list-like group types don't contain floats. This
    /// eventually needs to be sorted out.
    fn read_parquet_row(
        index: usize,
        row: Result<Row, parquet::errors::ParquetError>,
    ) -> Result<Self, RustitudeError> {
        let mut event = Self {
            index,
            ..Default::default()
        };
        let mut e_fs: Vec<Field> = Vec::new();
        let mut px_fs: Vec<Field> = Vec::new();
        let mut py_fs: Vec<Field> = Vec::new();
        let mut pz_fs: Vec<Field> = Vec::new();
        for (name, field) in row?.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_e(Field::from(*value));
                }
                ("Px_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_px(Field::from(*value));
                }
                ("Py_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_py(Field::from(*value));
                }
                ("Pz_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_pz(Field::from(*value));
                }
                ("Weight", ParquetField::Float(value)) => event.weight = Field::from(*value),
                ("EPS", ParquetField::ListInternal(list)) => {
                    event.eps = Vector3::from_vec(
                        list.elements()
                            .iter()
                            .map(|field| {
                                if let ParquetField::Float(value) = field {
                                    Field::from(*value)
                                } else {
                                    panic!()
                                }
                            })
                            .collect(),
                    );
                }
                ("E_FinalState", ParquetField::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", ParquetField::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", ParquetField::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", ParquetField::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
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
        // let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        // event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        // event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        // for dp4 in event.daughter_p4s.iter_mut() {
        //     *dp4 = dp4.boost_along(&final_state_p4);
        // }
        Ok(event)
    }
    /// Reads an [`Event`] from a single [`Row`] in a Parquet file, assuming EPS is stored in the
    /// beam four-momentum.
    ///
    /// # Panics
    ///
    /// This method currently panics if the list-like group types don't contain floats. This
    /// eventually needs to be sorted out.
    fn read_parquet_row_eps_in_beam(
        index: usize,
        row: Result<Row, parquet::errors::ParquetError>,
    ) -> Result<Self, RustitudeError> {
        let mut event = Self {
            index,
            ..Default::default()
        };
        let mut e_fs: Vec<Field> = Vec::new();
        let mut px_fs: Vec<Field> = Vec::new();
        let mut py_fs: Vec<Field> = Vec::new();
        let mut pz_fs: Vec<Field> = Vec::new();
        for (name, field) in row?.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_e(Field::from(*value));
                    event.beam_p4.set_pz(Field::from(*value));
                }
                ("Px_Beam", ParquetField::Float(value)) => {
                    event.eps[0] = Field::from(*value);
                }
                ("Py_Beam", ParquetField::Float(value)) => {
                    event.eps[1] = Field::from(*value);
                }
                ("Weight", ParquetField::Float(value)) => event.weight = Field::from(*value),
                ("E_FinalState", ParquetField::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", ParquetField::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", ParquetField::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", ParquetField::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
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
        // let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        // event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        // event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        // for dp4 in event.daughter_p4s.iter_mut() {
        //     *dp4 = dp4.boost_along(&final_state_p4);
        // }
        Ok(event)
    }

    /// Reads an [`Event`] from a single [`Row`] in a Parquet file and set EPS for all events.
    ///
    /// # Panics
    ///
    /// This method currently panics if the list-like group types don't contain floats. This
    /// eventually needs to be sorted out.
    fn read_parquet_row_with_eps(
        index: usize,
        row: Result<Row, parquet::errors::ParquetError>,
        eps: Vector3<Field>,
    ) -> Result<Self, RustitudeError> {
        let mut event = Self {
            index,
            eps,
            ..Default::default()
        };
        let mut e_fs: Vec<Field> = Vec::new();
        let mut px_fs: Vec<Field> = Vec::new();
        let mut py_fs: Vec<Field> = Vec::new();
        let mut pz_fs: Vec<Field> = Vec::new();
        for (name, field) in row?.get_column_iter() {
            match (name.as_str(), field) {
                ("E_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_e(Field::from(*value));
                }
                ("Px_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_px(Field::from(*value));
                }
                ("Py_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_py(Field::from(*value));
                }
                ("Pz_Beam", ParquetField::Float(value)) => {
                    event.beam_p4.set_pz(Field::from(*value));
                }
                ("Weight", ParquetField::Float(value)) => event.weight = Field::from(*value),
                ("E_FinalState", ParquetField::ListInternal(list)) => {
                    e_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Px_FinalState", ParquetField::ListInternal(list)) => {
                    px_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Py_FinalState", ParquetField::ListInternal(list)) => {
                    py_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                }
                ("Pz_FinalState", ParquetField::ListInternal(list)) => {
                    pz_fs = list
                        .elements()
                        .iter()
                        .map(|field| {
                            if let ParquetField::Float(value) = field {
                                Field::from(*value)
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
        // let final_state_p4 = event.recoil_p4 + event.daughter_p4s.iter().sum();
        // event.beam_p4 = event.beam_p4.boost_along(&final_state_p4);
        // event.recoil_p4 = event.recoil_p4.boost_along(&final_state_p4);
        // for dp4 in event.daughter_p4s.iter_mut() {
        //     *dp4 = dp4.boost_along(&final_state_p4);
        // }
        Ok(event)
    }

    /// Reads an [`Event`] from a single [`Row`] in a Parquet file and set EPS = `[0, 0, 0]` for
    /// all events.
    ///
    /// # Panics
    ///
    /// This method currently panics if the list-like group types don't contain floats. This
    /// eventually needs to be sorted out.
    fn read_parquet_row_unpolarized(
        index: usize,
        row: Result<Row, parquet::errors::ParquetError>,
    ) -> Result<Self, RustitudeError> {
        Self::read_parquet_row_with_eps(index, row, Vector3::default())
    }
}

/// An array of [`Event`]s with some helpful methods for accessing and parsing the data they
/// contain.
///
/// A [`Dataset`] can be loaded from either Parquet and ROOT files using the corresponding
/// `Dataset::from_*` methods. Events are stored in an [`Arc<Vec<Event>>`], since we
/// rarely need to write data to a dataset (splitting/selecting/rejecting events) but often need to
/// read events from a dataset.
#[derive(Default, Debug, Clone)]
pub struct Dataset {
    /// Storage for events.
    pub events: Arc<Vec<Event>>,
}

impl Dataset {
    // TODO: can we make an events(&self) -> &Vec<Field> method that actually works without cloning?

    /// Retrieves the weights from the events in the dataset
    pub fn weights(&self) -> Vec<Field> {
        self.events.iter().map(|e| e.weight).collect()
    }

    /// Retrieves the weights from the events in the dataset which have the given indices.
    pub fn weights_indexed(&self, indices: &[usize]) -> Vec<Field> {
        indices
            .iter()
            .map(|index| self.events[*index].weight)
            .collect()
    }

    /// Splits the dataset by the mass of the combination of specified daughter particles in the
    /// event. If no daughters are given, the first and second particle are assumed to form the
    /// desired combination. This method returns [`Vec<usize>`]s corresponding to the indices of
    /// events in each bin, the underflow bin, and the overflow bin respectively. This is intended
    /// to be used in conjunction with
    /// [`Manager::evaluate_indexed`](`crate::manager::Manager::evaluate_indexed`).
    pub fn split_m(
        &self,
        range: (Field, Field),
        bins: usize,
        daughter_indices: Option<Vec<usize>>,
    ) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        let mass = |e: &Event| {
            let p4: FourMomentum = daughter_indices
                .clone()
                .unwrap_or_else(|| vec![0, 1])
                .iter()
                .map(|i| e.daughter_p4s[*i])
                .sum();
            p4.m()
        };
        self.get_binned_indices(mass, range, bins)
    }

    /// Generates a new [`Dataset`] from a Parquet file.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_parquet(path: &str) -> Result<Self, RustitudeError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let row_iter = reader.get_row_iter(None)?;
        Ok(Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row(i, row))
                .collect::<Result<Vec<Event>, RustitudeError>>()?,
        ))
    }

    /// Generates a new [`Dataset`] from a Parquet file, assuming the EPS vector can be constructed
    /// from the x and y-components of the beam.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_parquet_eps_in_beam(path: &str) -> Result<Self, RustitudeError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let row_iter = reader.get_row_iter(None)?;
        Ok(Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_eps_in_beam(i, row))
                .collect::<Result<Vec<Event>, RustitudeError>>()?,
        ))
    }

    /// Generates a new [`Dataset`] from a Parquet file with a given EPS polarization vector.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_parquet_with_eps(path: &str, eps: Vec<Field>) -> Result<Self, RustitudeError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let row_iter = reader.get_row_iter(None)?;
        let eps_vec = Vector3::from_vec(eps);
        Ok(Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_with_eps(i, row, eps_vec))
                .collect::<Result<Vec<Event>, RustitudeError>>()?,
        ))
    }

    /// Generates a new [`Dataset`] from a Parquet file with EPS = `[0, 0, 0]`.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_parquet_unpolarized(path: &str) -> Result<Self, RustitudeError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let row_iter = reader.get_row_iter(None)?;
        Ok(Self::new(
            row_iter
                .enumerate()
                .map(|(i, row)| Event::read_parquet_row_unpolarized(i, row))
                .collect::<Result<Vec<Event>, RustitudeError>>()?,
        ))
    }

    /// Extract a branch from a ROOT `TTree` containing a [`Field`] (float in C). This method
    /// converts the underlying element to an [`Field`].
    fn extract_f32(
        path: &str,
        ttree: &ReaderTree,
        branch: &str,
    ) -> Result<Vec<Field>, RustitudeError> {
        let res = ttree
            .branch(branch)
            .ok_or_else(|| {
                RustitudeError::OxyrootError(format!(
                    "Could not find {} branch in {}",
                    branch, path
                ))
            })?
            .as_iter::<Field>()
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .map(Field::from)
            .collect();
        Ok(res)
    }

    /// Extract a branch from a ROOT `TTree` containing an array of [`Field`]s (floats in C). This
    /// method converts the underlying elements to [`Field`]s.
    fn extract_vec_f32(
        path: &str,
        ttree: &ReaderTree,
        branch: &str,
    ) -> Result<Vec<Vec<Field>>, RustitudeError> {
        let res: Vec<Vec<Field>> = ttree
            .branch(branch)
            .ok_or_else(|| {
                RustitudeError::OxyrootError(format!(
                    "Could not find {} branch in {}",
                    branch, path
                ))
            })?
            .as_iter::<Slice<Field>>()
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .map(|v| v.into_vec())
            .collect();
        Ok(res)
    }

    /// Generates a new [`Dataset`] from a ROOT file.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_root(path: &str) -> Result<Self, RustitudeError> {
        let ttree = RootFile::open(path)
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .get_tree("kin")
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?;
        let weight: Vec<Field> = Self::extract_f32(path, &ttree, "Weight")?;
        let e_beam: Vec<Field> = Self::extract_f32(path, &ttree, "E_Beam")?;
        let px_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Px_Beam")?;
        let py_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Py_Beam")?;
        let pz_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Pz_Beam")?;
        let e_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "E_FinalState")?;
        let px_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Px_FinalState")?;
        let py_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Py_FinalState")?;
        let pz_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Pz_FinalState")?;
        let eps: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "EPS")?;
        Ok(Self::new(
            izip!(weight, e_beam, px_beam, py_beam, pz_beam, e_fs, px_fs, py_fs, pz_fs, eps)
                .enumerate()
                .map(
                    |(i, (w, e_b, px_b, py_b, pz_b, e_f, px_f, py_f, pz_f, eps_vec))| Event {
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
                        eps: Vector3::from_vec(eps_vec),
                    },
                )
                .collect(),
        ))
    }

    /// Generates a new [`Dataset`] from a ROOT file, assuming the EPS vector can be constructed
    /// from the x and y-components of the beam.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_root_eps_in_beam(path: &str) -> Result<Self, RustitudeError> {
        let ttree = RootFile::open(path)
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .get_tree("kin")
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?;
        let weight: Vec<Field> = Self::extract_f32(path, &ttree, "Weight")?;
        let e_beam: Vec<Field> = Self::extract_f32(path, &ttree, "E_Beam")?;
        let px_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Px_Beam")?;
        let py_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Py_Beam")?;
        let pz_beam: Vec<Field> = Self::extract_f32(path, &ttree, "E_Beam")?;
        let e_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "E_FinalState")?;
        let px_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Px_FinalState")?;
        let py_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Py_FinalState")?;
        let pz_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Pz_FinalState")?;
        Ok(Self::new(
            izip!(weight, e_beam, px_beam, py_beam, pz_beam, e_fs, px_fs, py_fs, pz_fs)
                .enumerate()
                .map(
                    |(i, (w, e_b, px_b, py_b, pz_b, e_f, px_f, py_f, pz_f))| Event {
                        index: i,
                        weight: w,
                        beam_p4: FourMomentum::new(e_b, 0.0, 0.0, pz_b),
                        recoil_p4: FourMomentum::new(e_f[0], px_f[0], py_f[0], pz_f[0]),
                        daughter_p4s: izip!(
                            e_f[1..].iter(),
                            px_f[1..].iter(),
                            py_f[1..].iter(),
                            pz_f[1..].iter()
                        )
                        .map(|(e, px, py, pz)| FourMomentum::new(*e, *px, *py, *pz))
                        .collect(),
                        eps: Vector3::from_vec(vec![px_b, py_b, 0.0]),
                    },
                )
                .collect(),
        ))
    }

    /// Generates a new [`Dataset`] from a Parquet file with a given EPS polarization vector.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_root_with_eps(path: &str, eps: Vec<Field>) -> Result<Self, RustitudeError> {
        let ttree = RootFile::open(path)
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .get_tree("kin")
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?;
        let weight: Vec<Field> = Self::extract_f32(path, &ttree, "Weight")?;
        let e_beam: Vec<Field> = Self::extract_f32(path, &ttree, "E_Beam")?;
        let px_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Px_Beam")?;
        let py_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Py_Beam")?;
        let pz_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Pz_Beam")?;
        let e_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "E_FinalState")?;
        let px_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Px_FinalState")?;
        let py_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Py_FinalState")?;
        let pz_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Pz_FinalState")?;
        let eps = Vector3::from_vec(eps);
        Ok(Self::new(
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
                        eps,
                    },
                )
                .collect(),
        ))
    }

    /// Generates a new [`Dataset`] from a Parquet file with EPS = `[0, 0, 0]`.
    ///
    /// # Errors
    ///
    /// This method will fail if any individual event is missing all of the required fields, if
    /// they have the wrong type, or if the file doesn't exist/can't be read for any reason.
    pub fn from_root_unpolarized(path: &str) -> Result<Self, RustitudeError> {
        let ttree = RootFile::open(path)
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?
            .get_tree("kin")
            .map_err(|err| RustitudeError::OxyrootError(err.to_string()))?;
        let weight: Vec<Field> = Self::extract_f32(path, &ttree, "Weight")?;
        let e_beam: Vec<Field> = Self::extract_f32(path, &ttree, "E_Beam")?;
        let px_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Px_Beam")?;
        let py_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Py_Beam")?;
        let pz_beam: Vec<Field> = Self::extract_f32(path, &ttree, "Pz_Beam")?;
        let e_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "E_FinalState")?;
        let px_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Px_FinalState")?;
        let py_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Py_FinalState")?;
        let pz_fs: Vec<Vec<Field>> = Self::extract_vec_f32(path, &ttree, "Pz_FinalState")?;
        Ok(Self::new(
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
                        eps: Vector3::default(),
                    },
                )
                .collect(),
        ))
    }

    /// Generate a new [`Dataset`] from a [`Vec<Event>`].
    pub fn new(events: Vec<Event>) -> Self {
        info!("Dataset created with {} events", events.len());
        Self {
            events: Arc::new(events),
        }
    }

    /// Checks if the dataset is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the number of events in the dataset.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns a set of indices which represent a bootstrapped [`Dataset`]. This method is to be
    /// used in conjunction with
    /// [`Manager::evaluate_indexed`](crate::manager::Manager::evaluate_indexed).
    pub fn get_bootstrap_indices(&self, seed: usize) -> Vec<usize> {
        fastrand::seed(seed as u64);
        let mut inds: Vec<usize> = repeat_with(|| fastrand::usize(0..self.len()))
            .take(self.len())
            .collect();
        inds.sort_unstable();
        inds
    }

    /// Selects indices of events in a dataset using the given query. Indices of events for which
    /// the query returns `true` will end up in the first member of the returned tuple, and indices
    /// of events which return `false` will end up in the second member.
    pub fn get_selected_indices(
        &self,
        query: impl Fn(&Event) -> bool + Sync + Send,
    ) -> (Vec<usize>, Vec<usize>) {
        let (mut indices_selected, mut indices_rejected): (Vec<usize>, Vec<usize>) =
            self.events.par_iter().partition_map(|event| {
                if query(event) {
                    Either::Left(event.index)
                } else {
                    Either::Right(event.index)
                }
            });
        indices_selected.sort_unstable();
        indices_rejected.sort_unstable();
        (indices_selected, indices_rejected)
    }

    /// Splits the dataset by the given query. This method returns [`Vec<usize>`]s corresponding to
    /// the indices of events in each bin, the underflow bin, and the overflow bin respectively.
    /// This is intended to be used in conjunction with
    /// [`Manager::evaluate_indexed`](`crate::manager::Manager::evaluate_indexed`).
    pub fn get_binned_indices(
        &self,
        variable: impl Fn(&Event) -> Field + Sync + Send,
        range: (Field, Field),
        nbins: usize,
    ) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        let mut bins: Vec<Field> = Vec::with_capacity(nbins + 1);
        let width = (range.1 - range.0) / nbins as Field;
        for m in 0..=nbins {
            bins.push(width.mul_add(m as Field, range.0));
        }
        let (underflow, _) = self.get_selected_indices(|event| variable(event) < bins[0]);
        let (overflow, _) =
            self.get_selected_indices(|event| variable(event) >= bins[bins.len() - 1]);
        let binned_indices = bins
            .into_iter()
            .tuple_windows()
            .map(|(lb, ub)| {
                let (sel, _) = self.get_selected_indices(|event| {
                    let res = variable(event);
                    lb <= res && res < ub
                });
                sel
            })
            .collect();
        (binned_indices, underflow, overflow)
    }
}
