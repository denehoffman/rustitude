//! This module contains methods to link [`Model`]s with [`Dataset`]s via a [`Manager::evaluate`]
//! method. This module also holds a [`ExtendedLogLikelihood`] struct which holds two [`Manager`]s
//! and, as the name suggests, calculates an extended log-likelihood using a very basic method over
//! data and (accepted) Monte-Carlo.
use rayon::prelude::*;

use crate::{
    errors::RustitudeError,
    prelude::{Amplitude, Dataset, Event, Model, Parameter},
    Field,
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
    pub fn evaluate(&self, parameters: &[Field]) -> Result<Vec<Field>, RustitudeError> {
        let pars: Vec<Field> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        self.dataset
            .events
            .iter()
            .map(|event: &Event| self.model.compute(&pars, event))
            .collect()
    }

    /// Evaluate the [`Model`] over the [`Dataset`] with the given free parameters.
    ///
    /// This method allows the user to supply a list of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    pub fn evaluate_indexed(
        &self,
        parameters: &[Field],
        indices: &[usize],
    ) -> Result<Vec<Field>, RustitudeError> {
        if self.model.contains_python_amplitudes {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let pars: Vec<Field> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        indices
            .iter()
            .map(|index| self.model.compute(&pars, &self.dataset.events[*index]))
            .collect()
    }

    /// Evaluate the [`Model`] over the [`Dataset`] with the given free parameters.
    ///
    /// This version uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    pub fn par_evaluate(&self, parameters: &[Field]) -> Result<Vec<Field>, RustitudeError> {
        if self.model.contains_python_amplitudes {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let mut output = Vec::with_capacity(self.dataset.len());
        let pars: Vec<Field> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        self.dataset
            .events
            .par_iter()
            .map(|event| self.model.compute(&pars, event))
            .collect_into_vec(&mut output);
        output.into_iter().collect()
    }

    /// Evaluate the [`Model`] over the [`Dataset`] with the given free parameters.
    ///
    /// This method allows the user to supply a list of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// This version uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    pub fn par_evaluate_indexed(
        &self,
        parameters: &[Field],
        indices: &[usize],
    ) -> Result<Vec<Field>, RustitudeError> {
        if self.model.contains_python_amplitudes {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let mut output = Vec::with_capacity(indices.len());
        let pars: Vec<Field> = self
            .model
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        // indices
        //     .par_iter()
        //     .map(|index| self.model.compute(&pars, &self.dataset.events[*index]))
        //     .collect_into_vec(&mut output);
        let view: Vec<&Event> = indices
            .par_iter()
            .map(|&index| &self.dataset.events[index])
            .collect();
        view.par_iter()
            .map(|&event| self.model.compute(&pars, event))
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
        value: Field,
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
        bounds: (Field, Field),
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
        initial: Field,
    ) -> Result<(), RustitudeError> {
        self.model.set_initial(amplitude, parameter, initial)
    }

    /// Get a list of bounds for all free parameters in the [`Model`]. See
    /// [`Model::get_bounds`] for more information.
    pub fn get_bounds(&self) -> Vec<(Field, Field)> {
        self.model.get_bounds()
    }

    /// Get a list of initial values for all free parameters in the [`Model`]. See
    /// [`Model::get_initial`] for more information.
    pub fn get_initial(&self) -> Vec<Field> {
        self.model.get_initial()
    }

    /// Get the number of free parameters in the [`Model`] See [`Model::get_n_free`] for
    /// more information.
    pub fn get_n_free(&self) -> usize {
        self.model.get_n_free()
    }

    /// Activate an [`Amplitude`] by name. See [`Model::activate`] for more information.
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if the given
    /// amplitude is not present in the [`Model`].
    pub fn activate(&mut self, amplitude: &str) -> Result<(), RustitudeError> {
        self.model.activate(amplitude)
    }
    /// Activate all [`Amplitude`]s by name. See [`Model::activate_all`] for more information.
    pub fn activate_all(&mut self) {
        self.model.activate_all()
    }
    /// Activate only the specified [`Amplitude`]s while deactivating the rest. See
    /// [`Model::isolate`] for more information.
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if a given
    /// amplitude is not present in the [`Model`].
    pub fn isolate(&mut self, amplitudes: Vec<&str>) -> Result<(), RustitudeError> {
        self.model.isolate(amplitudes)
    }
    /// Deactivate an [`Amplitude`] by name. See [`Model::deactivate`] for more information.
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if the given
    /// amplitude is not present in the [`Model`].
    pub fn deactivate(&mut self, amplitude: &str) -> Result<(), RustitudeError> {
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
    pub fn evaluate(&self, parameters: &[Field]) -> Result<Field, RustitudeError> {
        let data_res = self.data_manager.evaluate(parameters)?;
        let data_weights = self.data_manager.dataset.weights();
        let n_data = data_weights.iter().sum::<Field>();
        let mc_norm_int = self.mc_manager.evaluate(parameters)?;
        let mc_weights = self.mc_manager.dataset.weights();
        let n_mc = mc_weights.iter().sum::<Field>();
        let ln_l = (data_res
            .iter()
            .zip(data_weights)
            .map(|(l, w)| w * l.ln())
            .sum::<Field>())
            - (n_data / n_mc)
                * (mc_norm_int
                    .iter()
                    .zip(mc_weights)
                    .map(|(l, w)| w * l)
                    .sum::<Field>());
        Ok(-2.0 * ln_l)
    }

    /// Evaluate the [`ExtendedLogLikelihood`] over the [`Dataset`] with the given free parameters.
    ///
    /// This method allows the user to supply two lists of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn evaluate_indexed(
        &self,
        parameters: &[Field],
        indices_data: &[usize],
        indices_mc: &[usize],
    ) -> Result<Field, RustitudeError> {
        let data_res = self
            .data_manager
            .evaluate_indexed(parameters, indices_data)?;
        let data_weights = self.data_manager.dataset.weights_indexed(indices_data);
        let n_data = data_weights.iter().sum::<Field>();
        let mc_norm_int = self.mc_manager.evaluate_indexed(parameters, indices_mc)?;
        let mc_weights = self.mc_manager.dataset.weights_indexed(indices_mc);
        let n_mc = mc_weights.iter().sum::<Field>();
        let ln_l = (data_res
            .iter()
            .zip(data_weights)
            .map(|(l, w)| w * l.ln())
            .sum::<Field>())
            - (n_data / n_mc)
                * (mc_norm_int
                    .iter()
                    .zip(mc_weights)
                    .map(|(l, w)| w * l)
                    .sum::<Field>());
        Ok(-2.0 * ln_l)
    }

    /// Evaluate the [`ExtendedLogLikelihood`] over the [`Dataset`] with the given free parameters.
    ///
    /// This method also allows the user to input a maximum number of threads to use in the
    /// calculation, as it uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_evaluate(&self, parameters: &[Field]) -> Result<Field, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let data_res = self.data_manager.par_evaluate(parameters)?;
        let data_weights = self.data_manager.dataset.weights();
        let n_data = data_weights.iter().sum::<Field>();
        let mc_norm_int = self.mc_manager.par_evaluate(parameters)?;
        let mc_weights = self.mc_manager.dataset.weights();
        let n_mc = mc_weights.iter().sum::<Field>();
        let ln_l = (data_res
            .par_iter()
            .zip(data_weights)
            .map(|(l, w)| w * l.ln())
            .sum::<Field>())
            - (n_data / n_mc)
                * (mc_norm_int
                    .par_iter()
                    .zip(mc_weights)
                    .map(|(l, w)| w * l)
                    .sum::<Field>());
        Ok(-2.0 * ln_l)
    }

    /// Evaluate the [`ExtendedLogLikelihood`] over the [`Dataset`] with the given free parameters.
    ///
    /// This method allows the user to supply two lists of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// This method also allows the user to input a maximum number of threads to use in the
    /// calculation, as it uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_evaluate_indexed(
        &self,
        parameters: &[Field],
        indices_data: &[usize],
        indices_mc: &[usize],
    ) -> Result<Field, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let data_res = self
            .data_manager
            .par_evaluate_indexed(parameters, indices_data)?;
        let data_weights = self.data_manager.dataset.weights_indexed(indices_data);
        let n_data = data_weights.iter().sum::<Field>();
        let mc_norm_int = self
            .mc_manager
            .par_evaluate_indexed(parameters, indices_mc)?;
        let mc_weights = self.mc_manager.dataset.weights_indexed(indices_mc);
        let n_mc = mc_weights.iter().sum::<Field>();
        let ln_l = (data_res
            .par_iter()
            .zip(data_weights)
            .map(|(l, w)| w * l.ln())
            .sum::<Field>())
            - (n_data / n_mc)
                * (mc_norm_int
                    .par_iter()
                    .zip(mc_weights)
                    .map(|(l, w)| w * l)
                    .sum::<Field>());
        Ok(-2.0 * ln_l)
    }

    /// Evaluate the normalized intensity function over the given Monte-Carlo [`Dataset`] with the
    /// given free parameters. This is intended to be used to plot a model over the dataset, usually
    /// with the generated or accepted Monte-Carlo as the input.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn intensity(
        &self,
        parameters: &[Field],
        dataset_mc: &Dataset,
    ) -> Result<Vec<Field>, RustitudeError> {
        let mc_manager = Manager::new(&self.data_manager.model, dataset_mc)?;
        let data_len_weighted: Field = self.data_manager.dataset.weights().iter().sum();
        let mc_len_weighted: Field = dataset_mc.weights().iter().sum();
        mc_manager.evaluate(parameters).map(|r_vec| {
            r_vec
                .iter()
                .zip(dataset_mc.events.iter())
                .map(|(r, e)| r * data_len_weighted / mc_len_weighted * e.weight)
                .collect()
        })
    }

    /// Evaluate the normalized intensity function over the given Monte-Carlo [`Dataset`] with the
    /// given free parameters. This is intended to be used to plot a model over the dataset, usually
    /// with the generated or accepted Monte-Carlo as the input.
    ///
    /// This method allows the user to supply a list of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn intensity_indexed(
        &self,
        parameters: &[Field],
        dataset_mc: &Dataset,
        indices_data: &[usize],
        indices_mc: &[usize],
    ) -> Result<Vec<Field>, RustitudeError> {
        let mc_manager = Manager::new(&self.data_manager.model, dataset_mc)?;
        let data_len_weighted = self
            .data_manager
            .dataset
            .weights_indexed(indices_data)
            .iter()
            .sum::<Field>();
        let mc_len_weighted = dataset_mc.weights_indexed(indices_mc).iter().sum::<Field>();
        let view: Vec<&Event> = indices_mc
            .par_iter()
            .map(|&index| &mc_manager.dataset.events[index])
            .collect();
        mc_manager
            .evaluate_indexed(parameters, indices_mc)
            .map(|r_vec| {
                r_vec
                    .iter()
                    .zip(view.iter())
                    .map(|(r, e)| r * data_len_weighted / mc_len_weighted * e.weight)
                    .collect()
            })
    }
    /// Evaluate the normalized intensity function over the given [`Dataset`] with the given
    /// free parameters. This is intended to be used to plot a model over the dataset, usually
    /// with the generated or accepted Monte-Carlo as the input.
    ///
    /// This method also allows the user to input a maximum number of threads to use in the
    /// calculation, as it uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_intensity(
        &self,
        parameters: &[Field],
        dataset_mc: &Dataset,
    ) -> Result<Vec<Field>, RustitudeError> {
        if self.data_manager.model.contains_python_amplitudes
            || self.mc_manager.model.contains_python_amplitudes
        {
            return Err(RustitudeError::PythonError(
                "Python amplitudes cannot be evaluated with Rust parallelism due to the GIL!"
                    .to_string(),
            ));
        }
        let mc_manager = Manager::new(&self.data_manager.model, dataset_mc)?;
        let data_len_weighted: Field = self.data_manager.dataset.weights().iter().sum();
        let mc_len_weighted: Field = dataset_mc.weights().iter().sum();
        mc_manager.par_evaluate(parameters).map(|r_vec| {
            r_vec
                .iter()
                .zip(dataset_mc.events.iter())
                .map(|(r, e)| r * data_len_weighted / mc_len_weighted * e.weight)
                .collect()
        })
    }

    /// Evaluate the normalized intensity function over the given Monte-Carlo [`Dataset`] with the
    /// given free parameters. This is intended to be used to plot a model over the dataset, usually
    /// with the generated or accepted Monte-Carlo as the input.
    ///
    /// This method allows the user to supply a list of indices and will only evaluate events at
    /// those indices. This can be used to evaluate only a subset of events or to resample events
    /// with replacement, such as in a bootstrap.
    ///
    /// This method also allows the user to input a maximum number of threads to use in the
    /// calculation, as it uses a parallel loop over events.
    ///
    /// # Errors
    ///
    /// This method will return a [`RustitudeError`] if the amplitude calculation fails. See
    /// [`Model::compute`] for more information.
    #[allow(clippy::suboptimal_flops)]
    pub fn par_intensity_indexed(
        &self,
        parameters: &[Field],
        dataset_mc: &Dataset,
        indices_data: &[usize],
        indices_mc: &[usize],
    ) -> Result<Vec<Field>, RustitudeError> {
        let mc_manager = Manager::new(&self.data_manager.model, dataset_mc)?;
        let data_len_weighted = self
            .data_manager
            .dataset
            .weights_indexed(indices_data)
            .iter()
            .sum::<Field>();
        let mc_len_weighted = dataset_mc.weights_indexed(indices_mc).iter().sum::<Field>();
        let view: Vec<&Event> = indices_mc
            .par_iter()
            .map(|&index| &mc_manager.dataset.events[index])
            .collect();
        mc_manager
            .par_evaluate_indexed(parameters, indices_mc)
            .map(|r_vec| {
                r_vec
                    .par_iter()
                    .zip(view.par_iter())
                    .map(|(r, e)| r * data_len_weighted / mc_len_weighted * e.weight)
                    .collect()
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
        value: Field,
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
        bounds: (Field, Field),
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
        initial: Field,
    ) -> Result<(), RustitudeError> {
        self.data_manager
            .set_initial(amplitude, parameter, initial)?;
        self.mc_manager.set_initial(amplitude, parameter, initial)
    }

    /// Get a list of bounds for all free parameters in the [`Model`]. See
    /// [`Model::get_bounds`] for more information.
    pub fn get_bounds(&self) -> Vec<(Field, Field)> {
        self.data_manager.get_bounds();
        self.mc_manager.get_bounds()
    }

    /// Get a list of initial values for all free parameters in the [`Model`]. See
    /// [`Model::get_initial`] for more information.
    pub fn get_initial(&self) -> Vec<Field> {
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
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if the given
    /// amplitude is not present in the [`Model`].
    pub fn activate(&mut self, amplitude: &str) -> Result<(), RustitudeError> {
        self.data_manager.activate(amplitude)?;
        self.mc_manager.activate(amplitude)
    }
    /// Activates all [`Amplitude`]s by name. See [`Model::activate_all`] for more information.
    pub fn activate_all(&mut self) {
        self.data_manager.activate_all();
        self.mc_manager.activate_all()
    }
    /// Activate only the specified [`Amplitude`]s while deactivating the rest. See
    /// [`Model::isolate`] for more information.
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if a given
    /// amplitude is not present in the [`Model`].
    pub fn isolate(&mut self, amplitudes: Vec<&str>) -> Result<(), RustitudeError> {
        self.data_manager.isolate(amplitudes.clone())?;
        self.mc_manager.isolate(amplitudes)
    }
    /// Deactivate an [`Amplitude`] by name. See [`Model::deactivate`] for more information.
    ///
    /// # Errors
    ///
    /// This function will return a [`RustitudeError::AmplitudeNotFoundError`] if the given
    /// amplitude is not present in the [`Model`].
    pub fn deactivate(&mut self, amplitude: &str) -> Result<(), RustitudeError> {
        self.data_manager.deactivate(amplitude)?;
        self.mc_manager.deactivate(amplitude)
    }
    /// Deactivates all [`Amplitude`]s by name. See [`Model::deactivate_all`] for more information.
    pub fn deactivate_all(&mut self) {
        self.data_manager.deactivate_all();
        self.mc_manager.deactivate_all()
    }
}
