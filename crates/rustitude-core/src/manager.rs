use rayon::prelude::*;

use crate::{
    create_pool,
    errors::RustitudeError,
    prelude::{Amplitude, Dataset, Event, Model, Parameter},
};

#[derive(Clone)]
pub struct Manager {
    pub model: Model,
    pub dataset: Dataset,
}
impl Manager {
    pub fn new(model: &Model, dataset: &Dataset) -> Result<Self, RustitudeError> {
        let mut model = model.clone();
        model.load(dataset)?;
        Ok(Self {
            model: model.clone(),
            dataset: dataset.clone(),
        })
    }
    pub fn evaluate(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        self.dataset
            .events
            .read()
            .par_iter()
            .map(|event: &Event| self.model.compute(parameters, event))
            .collect()
    }
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.model.get_amplitude(amplitude_name)
    }
    pub fn get_parameter(
        &self,
        amplitude_name: &str,
        parameter_name: &str,
    ) -> Result<Parameter, RustitudeError> {
        self.model.get_parameter(amplitude_name, parameter_name)
    }
    pub fn print_parameters(&self) {
        self.model.print_parameters()
    }
    pub fn constrain(
        &mut self,
        amplitude_1: &str,
        parameter_1: &str,
        amplitude_2: &str,
        parameter_2: &str,
    ) -> Result<(), RustitudeError> {
        self.model
            .constrain(amplitude_1, parameter_1, amplitude_2, parameter_2)
    }

    pub fn fix(
        &mut self,
        amplitude: &str,
        parameter: &str,
        value: f64,
    ) -> Result<(), RustitudeError> {
        self.model.fix(amplitude, parameter, value)
    }
    pub fn free(&mut self, amplitude: &str, parameter: &str) -> Result<(), RustitudeError> {
        self.model.free(amplitude, parameter)
    }
    pub fn set_bounds(
        &mut self,
        amplitude: &str,
        parameter: &str,
        bounds: (f64, f64),
    ) -> Result<(), RustitudeError> {
        self.model.set_bounds(amplitude, parameter, bounds)
    }
    pub fn set_initial(
        &mut self,
        amplitude: &str,
        parameter: &str,
        initial: f64,
    ) -> Result<(), RustitudeError> {
        self.model.set_initial(amplitude, parameter, initial)
    }
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        self.model.get_bounds()
    }
    pub fn get_initial(&self) -> Vec<f64> {
        self.model.get_initial()
    }
    pub fn get_n_free(&self) -> usize {
        self.model.get_n_free()
    }
    pub fn activate(&mut self, amplitude: &str) {
        self.model.activate(amplitude)
    }
    pub fn deactivate(&mut self, amplitude: &str) {
        self.model.deactivate(amplitude)
    }
}

pub struct ExtendedLogLikelihood {
    pub data_manager: Manager,
    pub mc_manager: Manager,
}
impl ExtendedLogLikelihood {
    pub const fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        Self {
            data_manager,
            mc_manager,
        }
    }
    #[allow(clippy::suboptimal_flops)]
    pub fn evaluate(
        &self,
        parameters: Vec<f64>,
        num_threads: usize,
    ) -> Result<f64, RustitudeError> {
        create_pool(num_threads)?.install(|| {
            let data_res = self.data_manager.evaluate(&parameters)?;
            let data_weights = self.data_manager.dataset.weights();
            let n_data = self.data_manager.dataset.len() as f64;
            let mc_res = self.mc_manager.evaluate(&parameters)?;
            let mc_weights = self.mc_manager.dataset.weights();
            let n_mc = self.mc_manager.dataset.len() as f64;
            let ln_l = (data_res
                .iter()
                .zip(data_weights)
                .map(|(l, w)| w * l.ln())
                .sum::<f64>())
                - (n_data / n_mc)
                    * (mc_res
                        .iter()
                        .zip(mc_weights)
                        .map(|(l, w)| w * l)
                        .sum::<f64>());
            Ok(-2.0 * ln_l)
        })
    }
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.data_manager.get_amplitude(amplitude_name)
    }
    pub fn get_parameter(
        &self,
        amplitude_name: &str,
        parameter_name: &str,
    ) -> Result<Parameter, RustitudeError> {
        self.data_manager
            .get_parameter(amplitude_name, parameter_name)
    }
    pub fn print_parameters(&self) {
        self.data_manager.print_parameters()
    }
    pub fn constrain(
        &mut self,
        amplitude_1: &str,
        parameter_1: &str,
        amplitude_2: &str,
        parameter_2: &str,
    ) -> Result<(), RustitudeError> {
        self.data_manager
            .constrain(amplitude_1, parameter_1, amplitude_2, parameter_2)?;
        self.mc_manager
            .constrain(amplitude_1, parameter_1, amplitude_2, parameter_2)
    }

    pub fn fix(
        &mut self,
        amplitude: &str,
        parameter: &str,
        value: f64,
    ) -> Result<(), RustitudeError> {
        self.data_manager.fix(amplitude, parameter, value)?;
        self.mc_manager.fix(amplitude, parameter, value)
    }
    pub fn free(&mut self, amplitude: &str, parameter: &str) -> Result<(), RustitudeError> {
        self.data_manager.free(amplitude, parameter)?;
        self.mc_manager.free(amplitude, parameter)
    }
    pub fn set_bounds(
        &mut self,
        amplitude: &str,
        parameter: &str,
        bounds: (f64, f64),
    ) -> Result<(), RustitudeError> {
        self.data_manager.set_bounds(amplitude, parameter, bounds)?;
        self.mc_manager.set_bounds(amplitude, parameter, bounds)
    }
    pub fn set_initial(
        &mut self,
        amplitude: &str,
        parameter: &str,
        initial: f64,
    ) -> Result<(), RustitudeError> {
        self.data_manager
            .set_initial(amplitude, parameter, initial)?;
        self.mc_manager.set_initial(amplitude, parameter, initial)
    }
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        self.data_manager.get_bounds();
        self.mc_manager.get_bounds()
    }
    pub fn get_initial(&self) -> Vec<f64> {
        self.data_manager.get_initial();
        self.mc_manager.get_initial()
    }
    pub fn get_n_free(&self) -> usize {
        self.data_manager.get_n_free();
        self.mc_manager.get_n_free()
    }
    pub fn activate(&mut self, amplitude: &str) {
        self.data_manager.activate(amplitude);
        self.mc_manager.activate(amplitude)
    }
    pub fn deactivate(&mut self, amplitude: &str) {
        self.data_manager.deactivate(amplitude);
        self.mc_manager.deactivate(amplitude)
    }
}
