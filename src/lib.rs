use std::collections::HashMap;

use pyo3::prelude::*;
use rayon::prelude::*;
use rustitude_core::manager::Manage;
use rustitude_core::manager::Manager;
use rustitude_core::{amplitude::Amplitude, manager::Parameter};
use rustitude_core::{dataset::Dataset, manager::ExtendedLogLikelihood};
use rustitude_core::{dataset::Event, four_momentum::FourMomentum};

#[pyclass(name = "Amplitude")]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyAmplitude(Amplitude);

#[pyclass(name = "Event")]
#[repr(transparent)]
pub struct PyEvent(Event);

#[pymethods]
impl PyEvent {
    pub fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

#[pyclass(name = "Dataset")]
#[repr(transparent)]
pub struct PyDataset(Dataset);

#[pymethods]
impl PyDataset {
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }

    pub fn __getitem__(&self, idx: isize) -> PyResult<Py<PyEvent>> {
        Ok(
            Python::with_gil(|py| Py::new(py, PyEvent(self.0.events.read()[idx as usize].clone())))
                .unwrap(),
        )
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
        nbins: usize,
        p1: Option<Vec<usize>>,
        p2: Option<Vec<usize>>,
    ) -> (Vec<PyDataset>, PyDataset, PyDataset) {
        let mass = |e: &Event| {
            let p1_p4 = p1
                .clone()
                .unwrap_or(vec![0])
                .iter()
                .map(|i| &e.daughter_p4s[*i])
                .sum::<FourMomentum>();
            let p2_p4 = p2
                .clone()
                .unwrap_or(vec![1])
                .iter()
                .map(|i| &e.daughter_p4s[*i])
                .sum::<FourMomentum>();
            (p1_p4 + p2_p4).m()
        };
        let (datasets, underflow, overflow) = self.0.clone().split(mass, range, nbins); // TODO: Clone is not the right thign to do here
        (
            datasets.into_iter().map(PyDataset).collect(),
            PyDataset(underflow),
            PyDataset(overflow),
        )
    }

    #[staticmethod]
    pub fn from_parquet(path: &str) -> PyDataset {
        PyDataset(Dataset::from_parquet(path))
    }

    #[staticmethod]
    pub fn from_dict(py: Python, data: HashMap<String, PyObject>) -> PyResult<PyDataset> {
        let e_beam_vec: Vec<f64> = data["E_Beam"].extract(py)?;
        let px_beam_vec: Vec<f64> = data["Px_Beam"].extract(py)?;
        let py_beam_vec: Vec<f64> = data["Py_Beam"].extract(py)?;
        let pz_beam_vec: Vec<f64> = data["Pz_Beam"].extract(py)?;
        let weight_vec: Vec<f64> = data["Weight"].extract(py)?;
        let e_finalstate_vec: Vec<Vec<f64>> = data["E_FinalState"].extract(py)?;
        let px_finalstate_vec: Vec<Vec<f64>> = data["Px_FinalState"].extract(py)?;
        let py_finalstate_vec: Vec<Vec<f64>> = data["Py_FinalState"].extract(py)?;
        let pz_finalstate_vec: Vec<Vec<f64>> = data["Pz_FinalState"].extract(py)?;
        Ok(PyDataset(Dataset::new(
            (
                e_beam_vec,
                px_beam_vec,
                py_beam_vec,
                pz_beam_vec,
                weight_vec,
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
                            ..Default::default()
                        }
                    },
                )
                .collect(),
        )))
    }
}

#[pyclass(name = "Parameter")]
#[repr(transparent)]
pub struct PyParameter(Parameter);

#[pyclass(name = "Manager")]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyManager(Manager);

#[pymethods]
impl PyManager {
    #[new]
    pub fn new(dataset: &PyDataset) -> PyManager {
        PyManager(Manager::new(&dataset.0))
    }
    pub fn register(&mut self, sum_name: &str, group_name: &str, amplitude: PyAmplitude) {
        self.0.register(sum_name, group_name, &amplitude.0);
    }
    pub fn constrain(
        &mut self,
        parameter_1: (String, String, String, String),
        parameter_2: (String, String, String, String),
    ) {
        let p1_0 = parameter_1.0;
        let p1_1 = parameter_1.1;
        let p1_2 = parameter_1.2;
        let p1_3 = parameter_1.3;
        let p2_0 = parameter_2.0;
        let p2_1 = parameter_2.1;
        let p2_2 = parameter_2.2;
        let p2_3 = parameter_2.3;
        self.0
            .constrain((&p1_0, &p1_1, &p1_2, &p1_3), (&p2_0, &p2_1, &p2_2, &p2_3));
    }
    pub fn constrain_amplitude(
        &mut self,
        amplitude_1: (String, String, String),
        amplitude_2: (String, String, String),
    ) {
        let a1_0 = amplitude_1.0;
        let a1_1 = amplitude_1.1;
        let a1_2 = amplitude_1.2;
        let a2_0 = amplitude_2.0;
        let a2_1 = amplitude_2.1;
        let a2_2 = amplitude_2.2;
        self.0
            .constrain_amplitude((&a1_0, &a1_1, &a1_2), (&a2_0, &a2_1, &a2_2));
    }
    pub fn activate(&mut self, amplitude: (String, String, String)) {
        self.0.activate((&amplitude.0, &amplitude.1, &amplitude.2));
    }
    pub fn deactivate(&mut self, amplitude: (String, String, String)) {
        self.0
            .deactivate((&amplitude.0, &amplitude.1, &amplitude.2));
    }
    pub fn fix(&mut self, parameter: (String, String, String, String), value: f64) {
        self.0.fix(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            value,
        );
    }

    pub fn free(&mut self, parameter: (String, String, String, String), initial_value: f64) {
        self.0.free(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            initial_value,
        );
    }
    pub fn set_bounds(
        &mut self,
        parameter: (String, String, String, String),
        lower_bound: f64,
        upper_bound: f64,
    ) {
        self.0.set_bounds(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            lower_bound,
            upper_bound,
        );
    }
    pub fn set_initial(&mut self, parameter: (String, String, String, String), initial_value: f64) {
        self.0.set_initial(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            initial_value,
        );
    }
    pub fn get_lower_bounds(&self) -> Vec<f64> {
        self.0.get_lower_bounds()
    }
    pub fn get_upper_bounds(&self) -> Vec<f64> {
        self.0.get_upper_bounds()
    }
    pub fn get_initial_values(&self) -> Vec<f64> {
        self.0.get_initial_values()
    }
    pub fn compute(&self, parameters: Vec<f64>) -> Vec<f64> {
        self.0.compute(&parameters)
    }
    pub fn parameters(&self) -> Vec<(String, String, String, String)> {
        self.0
            .parameters()
            .into_iter()
            .map(|p| (p.get_sum(), p.get_group(), p.get_amplitude(), p.get_name()))
            .collect()
    }
}

// Built-In Amplitudes:
#[pyfunction]
fn scalar(name: &str) -> PyAmplitude {
    PyAmplitude(rustitude_core::amplitude::scalar(name))
}
#[pyfunction]
fn cscalar(name: &str) -> PyAmplitude {
    PyAmplitude(rustitude_core::amplitude::cscalar(name))
}
#[pyfunction]
fn pcscalar(name: &str) -> PyAmplitude {
    PyAmplitude(rustitude_core::amplitude::pcscalar(name))
}

#[pyclass(name = "ExtendedLogLikelihood")]
#[repr(transparent)]
pub struct PyExtendedLogLikelihood(ExtendedLogLikelihood);

#[pymethods]
impl PyExtendedLogLikelihood {
    #[new]
    pub fn new(dataset_data: &PyDataset, dataset_mc: &PyDataset) -> PyExtendedLogLikelihood {
        PyExtendedLogLikelihood(ExtendedLogLikelihood::new(&dataset_data.0, &dataset_mc.0))
    }
    pub fn register(&mut self, sum_name: &str, group_name: &str, amplitude: PyAmplitude) {
        self.0.register(sum_name, group_name, &amplitude.0);
    }
    pub fn constrain(
        &mut self,
        parameter_1: (String, String, String, String),
        parameter_2: (String, String, String, String),
    ) {
        let p1_0 = parameter_1.0;
        let p1_1 = parameter_1.1;
        let p1_2 = parameter_1.2;
        let p1_3 = parameter_1.3;
        let p2_0 = parameter_2.0;
        let p2_1 = parameter_2.1;
        let p2_2 = parameter_2.2;
        let p2_3 = parameter_2.3;
        self.0
            .constrain((&p1_0, &p1_1, &p1_2, &p1_3), (&p2_0, &p2_1, &p2_2, &p2_3));
    }
    pub fn constrain_amplitude(
        &mut self,
        amplitude_1: (String, String, String),
        amplitude_2: (String, String, String),
    ) {
        let a1_0 = amplitude_1.0;
        let a1_1 = amplitude_1.1;
        let a1_2 = amplitude_1.2;
        let a2_0 = amplitude_2.0;
        let a2_1 = amplitude_2.1;
        let a2_2 = amplitude_2.2;
        self.0
            .constrain_amplitude((&a1_0, &a1_1, &a1_2), (&a2_0, &a2_1, &a2_2));
    }
    pub fn activate(&mut self, amplitude: (String, String, String)) {
        self.0.activate((&amplitude.0, &amplitude.1, &amplitude.2));
    }
    pub fn deactivate(&mut self, amplitude: (String, String, String)) {
        self.0
            .deactivate((&amplitude.0, &amplitude.1, &amplitude.2));
    }
    pub fn fix(&mut self, parameter: (String, String, String, String), value: f64) {
        self.0.fix(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            value,
        );
    }

    pub fn free(&mut self, parameter: (String, String, String, String), initial_value: f64) {
        self.0.free(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            initial_value,
        );
    }
    pub fn set_bounds(
        &mut self,
        parameter: (String, String, String, String),
        lower_bound: f64,
        upper_bound: f64,
    ) {
        self.0.set_bounds(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            lower_bound,
            upper_bound,
        );
    }
    pub fn set_initial(&mut self, parameter: (String, String, String, String), initial_value: f64) {
        self.0.set_initial(
            (&parameter.0, &parameter.1, &parameter.2, &parameter.3),
            initial_value,
        );
    }
    pub fn get_lower_bounds(&self) -> Vec<f64> {
        self.0.get_lower_bounds()
    }
    pub fn get_upper_bounds(&self) -> Vec<f64> {
        self.0.get_upper_bounds()
    }
    pub fn get_initial_values(&self) -> Vec<f64> {
        self.0.get_initial_values()
    }
    pub fn compute(&self, parameters: Vec<f64>) -> f64 {
        self.0.compute(&parameters)
    }
    pub fn parameters(&self) -> Vec<(String, String, String, String)> {
        self.0
            .parameters()
            .into_iter()
            .map(|p| (p.get_sum(), p.get_group(), p.get_amplitude(), p.get_name()))
            .collect()
    }
}

// GlueX
mod gluex;

#[pymodule]
fn rustitude(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Classes
    m.add_class::<PyAmplitude>()?;
    m.add_function(wrap_pyfunction!(scalar, m)?)?;
    m.add_function(wrap_pyfunction!(cscalar, m)?)?;
    m.add_function(wrap_pyfunction!(pcscalar, m)?)?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyDataset>()?;
    m.add_class::<PyParameter>()?;

    let m_managers = PyModule::new_bound(m.py(), "rustitude.managers")?;
    m_managers.add_class::<PyManager>()?;
    m_managers.add_class::<PyExtendedLogLikelihood>()?;
    m.add("managers", &m_managers)?;
    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("rustitude.managers", &m_managers)?;

    // GlueX
    let m_gluex = PyModule::new_bound(m.py(), "rustitude.gluex")?;
    gluex::gluex(&m_gluex)?;
    m.add("gluex", &m_gluex)?;
    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("rustitude.gluex", &m_gluex)?;

    Ok(())
}
