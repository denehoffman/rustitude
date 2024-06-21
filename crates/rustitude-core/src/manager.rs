//! This module contains methods to link [`Model`]s with [`Dataset`]s via a [`Manager::evaluate`]
//! method. This module also holds a [`ExtendedLogLikelihood`] struct which holds two [`Manager`]s
//! and, as the name suggests, calculates an extended log-likelihood using a very basic method over
//! data and (accepted) Monte-Carlo.
use rayon::prelude::*;

use crate::{
    create_pool,
    errors::RustitudeError,
    prelude::{Amplitude, Dataset, Event, Model, Parameter},
};

/// The [`Manager`] struct links a [`Model`] to a [`Dataset`] and provides methods to manipulate
/// the [`Model`] and evaluate it over the [`Dataset`].
#[derive(Clone)]
pub struct Manager {
    /// The associated [`Model`].
    pub model: Model,
    /// The associated [`Dataset`].
    pub dataset: Dataset,
}
impl Manager {
    /// Generates a new [`Manager`] from a [`Model`] and [`Dataset`].
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the precaluclation phase of the [`Model`]
    /// fails for any events in the [`Dataset`]. See [`Model::load`] for more information.
    pub fn new(model: &Model, dataset: &Dataset) -> Result<Self, RustitudeError> {
        let mut model = model.clone();
        model.load(dataset)?;
        Ok(Self {
            model: model.clone(),
            dataset: dataset.clone(),
        })
    }

    /// Evaluate the [`Model`] over the [`Dataset`] with the given free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    pub fn evaluate(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        let pars: Vec<f64> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        self.dataset
            .events
            .read_arc()
            .iter()
            .map(|event: &Event| self.model.compute(&pars, event))
            .collect()
    }
    /// Evaluate the [`Model`] over the [`Dataset`] with the given free parameters.
    /// This version uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    pub fn par_evaluate(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        if self.model.contains_python_amplitudes {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let mut output = Vec::with_capacity(self.dataset.len());
        let pars: Vec<f64> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        self.dataset
            .events
            .read_arc()
            .par_iter()
            .map(|event| self.model.compute(&pars, event))
            .collect_into_vec(&mut output);
        output.into_iter().collect()
    }

    /// Find the normalization integral for the [`Model`] over the [`Dataset`] with the given
    /// free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::norm_int`] for more information.
    #[deprecated(
        since = "0.7.1",
        note = "Manager::evaluate is faster and should give equivalent results"
    )]
    pub fn norm_int(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        self.dataset
            .events
            .read_arc()
            .iter()
            .map(|event: &Event| self.model.norm_int(parameters, event))
            .collect()
    }
    /// Find the normalization integral for the [`Model`] over the [`Dataset`] with the given
    /// free parameters. This version uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::norm_int`] for more information.
    #[deprecated(
        since = "0.7.1",
        note = "Manager::par_evaluate is faster and should give equivalent results"
    )]
    pub fn par_norm_int(&self, parameters: &[f64]) -> Result<Vec<f64>, RustitudeError> {
        if self.model.contains_python_amplitudes {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let mut output = Vec::with_capacity(self.dataset.len());
        self.dataset
            .events
            .read_arc()
            .par_iter()
            .map(|event: &Event| self.model.norm_int(parameters, event))
            .collect_into_vec(&mut output);
        output.into_iter().collect()
    }

    /// Get a copy of an [`Amplitude`] in the [`Model`] by name.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if there is no amplitude found with the given
    /// name in the parent [`Model`]. See [`Model::get_amplitude`] for more information.
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.model.get_amplitude(amplitude_name)
    }

    /// Get a copy of a [`Parameter`] in a [`Model`] by name and the name of the parent
    /// [`Amplitude`].
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if there is no parameter found with the given
    /// name in the parent [`Model`]. It will also first check if the given amplitude exists, and
    /// this method can also fail in the same way (see [`Model::get_amplitude`] and
    /// [`Model::get_parameter`]).
    pub fn get_parameter(
        &self,
        amplitude_name: &str,
        parameter_name: &str,
    ) -> Result<Parameter, RustitudeError> {
        self.model.get_parameter(amplitude_name, parameter_name)
    }

    /// Print the free parameters in the [`Model`]. See [`Model::print_parameters`] for more
    /// information.
    pub fn print_parameters(&self) {
        self.model.print_parameters()
    }

    /// Constrain two parameters by name, reducing the number of free parameters by one.
    ///
    /// # Errors
    ///
    /// This method will fail if any of the given amplitude or parameter names don't correspond to
    /// a valid amplitude-parameter pair. See [`Model::constrain`] for more information.
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

    /// Fix a parameter by name to the given value.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::fix`] for more information.
    pub fn fix(
        &mut self,
        amplitude: &str,
        parameter: &str,
        value: f64,
    ) -> Result<(), RustitudeError> {
        self.model.fix(amplitude, parameter, value)
    }

    /// Free a fixed parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::free`] for more information.
    pub fn free(&mut self, amplitude: &str, parameter: &str) -> Result<(), RustitudeError> {
        self.model.free(amplitude, parameter)
    }

    /// Set the bounds of a parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::set_bounds`] for more information.
    pub fn set_bounds(
        &mut self,
        amplitude: &str,
        parameter: &str,
        bounds: (f64, f64),
    ) -> Result<(), RustitudeError> {
        self.model.set_bounds(amplitude, parameter, bounds)
    }

    /// Set the initial value of a parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::set_initial`] for more information.
    pub fn set_initial(
        &mut self,
        amplitude: &str,
        parameter: &str,
        initial: f64,
    ) -> Result<(), RustitudeError> {
        self.model.set_initial(amplitude, parameter, initial)
    }

    /// Get a list of bounds for all free parameters in the [`Model`]. See
    /// [`Model::get_bounds`] for more information.
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        self.model.get_bounds()
    }

    /// Get a list of initial values for all free parameters in the [`Model`]. See
    /// [`Model::get_initial`] for more information.
    pub fn get_initial(&self) -> Vec<f64> {
        self.model.get_initial()
    }

    /// Get the number of free parameters in the [`Model`] See [`Model::get_n_free`] for
    /// more information.
    pub fn get_n_free(&self) -> usize {
        self.model.get_n_free()
    }

    /// Activate an [`Amplitude`] by name. See [`Model::activate`] for more information.
    pub fn activate(&mut self, amplitude: &str) {
        self.model.activate(amplitude)
    }
    /// Activate all [`Amplitude`]s by name. See [`Model::activate_all`] for more information.
    pub fn activate_all(&mut self) {
        self.model.activate_all()
    }
    /// Deactivate an [`Amplitude`] by name. See [`Model::deactivate`] for more information.
    pub fn deactivate(&mut self, amplitude: &str) {
        self.model.deactivate(amplitude)
    }
    /// Deactivate all [`Amplitude`]s by name. See [`Model::deactivate_all`] for more information.
    pub fn deactivate_all(&mut self) {
        self.model.deactivate_all()
    }
}

/// The [`ExtendedLogLikelihood`] stores two [`Manager`]s, one for data and one for a Monte-Carlo
/// dataset used for acceptance correction. These should probably have the same [`Manager`] in
/// practice, but this is left to the user.
pub struct ExtendedLogLikelihood {
    /// [`Manager`] for data
    pub data_manager: Manager,
    /// [`Manager`] for Monte-Carlo
    pub mc_manager: Manager,
}
impl ExtendedLogLikelihood {
    /// Create a new [`ExtendedLogLikelihood`] from a data and Monte-Carlo [`Manager`]s.
    pub const fn new(data_manager: Manager, mc_manager: Manager) -> Self {
        Self {
            data_manager,
            mc_manager,
        }
    }

    /// Evaluate the [`ExtendedLogLikelihood`] over the [`Dataset`] with the given free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn evaluate(&self, parameters: &[f64]) -> Result<f64, RustitudeError> {
        let data_res = self.data_manager.evaluate(parameters)?;
        let data_weights = self.data_manager.dataset.weights();
        let n_data = self.data_manager.dataset.len() as f64;
        let mc_norm_int = self.mc_manager.evaluate(parameters)?;
        let mc_weights = self.mc_manager.dataset.weights();
        let n_mc = self.mc_manager.dataset.len() as f64;
        let ln_l = (data_res
            .iter()
            .zip(data_weights)
            .map(|(l, w)| w * l.ln())
            .sum::<f64>())
            - (n_data / n_mc)
                * (mc_norm_int
                    .iter()
                    .zip(mc_weights)
                    .map(|(l, w)| w * l)
                    .sum::<f64>());
        Ok(-2.0 * ln_l)
    }
    /// Evaluate the [`ExtendedLogLikelihood`] over the [`Dataset`] with the given free parameters
    /// This method also allows the user to input a maximum number of threads to use in the
    /// calculation, as it uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_evaluate(
        &self,
        parameters: &[f64],
        num_threads: usize,
    ) -> Result<f64, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        create_pool(num_threads)?.install(|| {
            let data_res = self.data_manager.par_evaluate(parameters)?;
            let data_weights = self.data_manager.dataset.weights();
            let n_data = self.data_manager.dataset.len() as f64;
            let mc_norm_int = self.mc_manager.par_evaluate(parameters)?;
            let mc_weights = self.mc_manager.dataset.weights();
            let n_mc = self.mc_manager.dataset.len() as f64;
            let ln_l = (data_res
                .par_iter()
                .zip(data_weights)
                .map(|(l, w)| w * l.ln())
                .sum::<f64>())
                - (n_data / n_mc)
                    * (mc_norm_int
                        .par_iter()
                        .zip(mc_weights)
                        .map(|(l, w)| w * l)
                        .sum::<f64>());
            Ok(-2.0 * ln_l)
        })
    }

    /// Evaluate the unnormalized intensity function over the given [`Dataset`] with the given
    /// free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn intensity(
        &self,
        parameters: &[f64],
        dataset: &Dataset,
    ) -> Result<Vec<f64>, RustitudeError> {
        let manager = Manager::new(&self.data_manager.model, dataset)?;
        manager.evaluate(parameters)
    }
    /// Evaluate the unnormalized intensity function over the given [`Dataset`] with the given
    /// free parameters. This method also allows the user to input a maximum number of threads
    /// to use in the calculation. This version uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_intensity(
        &self,
        parameters: &[f64],
        dataset: &Dataset,
        num_threads: usize,
    ) -> Result<Vec<f64>, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let manager = Manager::new(&self.data_manager.model, dataset)?;
        create_pool(num_threads)?.install(|| manager.par_evaluate(parameters))
    }

    /// Find the normalization integral for the [`Model`] over the [`Dataset`] with the given
    /// free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::norm_int`] for more information.
    #[deprecated(
        since = "0.7.1",
        note = "ExtendedLogLikelihood::evaluate is faster and should give equivalent results"
    )]
    pub fn norm_int(&self, parameters: &[f64], weighted: bool) -> Result<f64, RustitudeError> {
        let mc_norm_int = self.mc_manager.norm_int(parameters)?;
        if weighted {
            let mc_weights = self.mc_manager.dataset.weights();
            Ok(mc_norm_int.iter().zip(mc_weights).map(|(r, w)| r * w).sum())
        } else {
            Ok(mc_norm_int.iter().sum())
        }
    }
    /// Find the normalization integral for the [`Model`] over the [`Dataset`] with the given
    /// free parameters.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::norm_int`] for more information.
    #[deprecated(
        since = "0.7.1",
        note = "ExtendedLogLikelihood::par_evaluate is faster and should give equivalent results"
    )]
    pub fn par_norm_int(
        &self,
        parameters: &[f64],
        num_threads: usize,
        weighted: bool,
    ) -> Result<f64, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        create_pool(num_threads)?.install(|| {
            let mc_norm_int = self.mc_manager.par_norm_int(parameters)?;
            if weighted {
                let mc_weights = self.mc_manager.dataset.weights();
                Ok(mc_norm_int.iter().zip(mc_weights).map(|(r, w)| r * w).sum())
            } else {
                Ok(mc_norm_int.iter().sum())
            }
        })
    }
    /// Get a copy of an [`Amplitude`] in the [`Model`] by name.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if there is no amplitude found with the given
    /// name in the parent [`Model`]. See [`Model::get_amplitude`] for more information.
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.data_manager.get_amplitude(amplitude_name)
    }

    /// Get a copy of a [`Parameter`] in a [`Model`] by name and the name of the parent
    /// [`Amplitude`].
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if there is no parameter found with the given
    /// name in the parent [`Model`]. It will also first check if the given amplitude exists, and
    /// this method can also fail in the same way (see [`Model::get_amplitude`] and
    /// [`Model::get_parameter`]).
    pub fn get_parameter(
        &self,
        amplitude_name: &str,
        parameter_name: &str,
    ) -> Result<Parameter, RustitudeError> {
        self.data_manager
            .get_parameter(amplitude_name, parameter_name)
    }

    /// Print the free parameters in the [`Model`]. See [`Model::print_parameters`] for more
    /// information.
    pub fn print_parameters(&self) {
        self.data_manager.print_parameters()
    }

    /// Constrain two parameters by name, reducing the number of free parameters by one.
    ///
    /// # Errors
    ///
    /// This method will fail if any of the given amplitude or parameter names don't correspond to
    /// a valid amplitude-parameter pair. See [`Model::constrain`] for more information.
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

    /// Fix a parameter by name to the given value.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::fix`] for more information.
    pub fn fix(
        &mut self,
        amplitude: &str,
        parameter: &str,
        value: f64,
    ) -> Result<(), RustitudeError> {
        self.data_manager.fix(amplitude, parameter, value)?;
        self.mc_manager.fix(amplitude, parameter, value)
    }

    /// Free a fixed parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::free`] for more information.
    pub fn free(&mut self, amplitude: &str, parameter: &str) -> Result<(), RustitudeError> {
        self.data_manager.free(amplitude, parameter)?;
        self.mc_manager.free(amplitude, parameter)
    }

    /// Set the bounds of a parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::set_bounds`] for more information.
    pub fn set_bounds(
        &mut self,
        amplitude: &str,
        parameter: &str,
        bounds: (f64, f64),
    ) -> Result<(), RustitudeError> {
        self.data_manager.set_bounds(amplitude, parameter, bounds)?;
        self.mc_manager.set_bounds(amplitude, parameter, bounds)
    }

    /// Set the initial value of a parameter by name.
    ///
    /// # Errors
    ///
    /// This method will fail if the given amplitude-parameter pair does not exist. See
    /// [`Model::set_initial`] for more information.
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

    /// Get a list of bounds for all free parameters in the [`Model`]. See
    /// [`Model::get_bounds`] for more information.
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        self.data_manager.get_bounds();
        self.mc_manager.get_bounds()
    }

    /// Get a list of initial values for all free parameters in the [`Model`]. See
    /// [`Model::get_initial`] for more information.
    pub fn get_initial(&self) -> Vec<f64> {
        self.data_manager.get_initial();
        self.mc_manager.get_initial()
    }

    /// Get the number of free parameters in the [`Model`] See [`Model::get_n_free`] for
    /// more information.
    pub fn get_n_free(&self) -> usize {
        self.data_manager.get_n_free();
        self.mc_manager.get_n_free()
    }

    /// Activate an [`Amplitude`] by name. See [`Model::activate`] for more information.
    pub fn activate(&mut self, amplitude: &str) {
        self.data_manager.activate(amplitude);
        self.mc_manager.activate(amplitude)
    }
    /// Activates all [`Amplitude`]s by name. See [`Model::activate_all`] for more information.
    pub fn activate_all(&mut self) {
        self.data_manager.activate_all();
        self.mc_manager.activate_all()
    }
    /// Deactivate an [`Amplitude`] by name. See [`Model::deactivate`] for more information.
    pub fn deactivate(&mut self, amplitude: &str) {
        self.data_manager.deactivate(amplitude);
        self.mc_manager.deactivate(amplitude)
    }
    /// Deactivates all [`Amplitude`]s by name. See [`Model::deactivate_all`] for more information.
    pub fn deactivate_all(&mut self) {
        self.data_manager.deactivate_all();
        self.mc_manager.deactivate_all()
    }
}
