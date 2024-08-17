//! # Rustitude
//! ## Demystifying Amplitude Analysis with Rust and Python
//!
//! The `rustitude-core` crate aims to implement common amplitude analysis techniques in Rust with
//! bindings to Python. This crate does not include the Python bindings, see the [GitHub
//! repo](https://github.com/denehoffman/rustitude) for more information on the Python API.
//!
//! The three core principles of `rustitude-core` are:
//! 1. Parallelization over events is automatically handeled by a [`Manager`](`crate::manager::Manager`).
//! 2. Amplitudes are written to do as much work as possible ahead of time, and evaluations use
//!    caching as much as possible automatically.
//! 3. Developers just need to implement the [`Node`](`crate::amplitude::Node`) trait to write a new
//!    amplitude, everything else is handled by the crate.
//!
//! ## Table of Contents
//!
//! * [Dataset Structure](#dataset-structure)
//! * [Creating a New Amplitude](#creating-a-new-amplitude)
//! * [Combining Amplitudes into Models](#combining-amplitudes-into-models)
//! * [Managing Parameters](#managing-parameters)
//! * [Evaluating Likelihoods](#evaluating-likelihoods)
//!
//! # Dataset Structure
//!
//! A [`Dataset`](`crate::dataset::Dataset`) is essentially just a wrapper for a [`Vec`] of
//! [`Event`](`crate::dataset::Event`)s. The current [`Event`](`crate::dataset::Event`) structure is as follows:
//!
//! ```ignore
//! pub struct Event {
//!     pub index: usize,                    // Position of event within dataset
//!     pub weight: f32,                     // Event weight
//!     pub beam_p4: FourMomentum,           // Beam four-momentum
//!     pub recoil_p4: FourMomentum,         // Recoil four-momentum
//!     pub daughter_p4s: Vec<FourMomentum>, // Four-momenta of final state particles sans recoil
//!     pub eps: Vector3<f32>,               // Beam polarization vector
//! }
//! ```
//!
//! In the Rust API, we can create [`Dataset`](`crate::dataset::Dataset`)s from `ROOT` files as well as
//! `Parquet` files. `ROOT` file reading is done through [`oxyroot`] - This still has some issues,
//! and large files or files with user metadata might fail to load. The alternative `Parquet`
//! format can be obtained from a `ROOT` file by using a conversion script like the one provided
//! [here](https://github.com/denehoffman/rustitude/blob/main/bin/convert). By default, we expect
//! all of the [`Event`](`crate::dataset::Event`) fields to be mirrored as the following branches:
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
//! A `Parquet` file with these columns can be loaded with the following:
//! ```ignore
//! use rustitude_core::prelude::*;
//! fn main() -> Result<(), RustitudeError> {
//!     let dataset = Dataset::from_parquet("path/to/file.parquet")?;
//!     println!("{}", dataset.events()[0]); // print first event
//! }
//! ```
//!
//! Because the beam is often directed along the $`z`$-axis, there is an alternative way to store
//! the `EPS` vector without a new branch (for linear polarization. The $`x`$ and $`y`$ components
//! of `EPS` can be stored as `Px_Beam` and `Py_Beam` respectively, and the format can be loaded
//! using [`Dataset::from_parquet_eps_in_beam`](`crate::dataset::Dataset::from_parquet_eps_in_beam`).
//!
//! # Creating a New Amplitude
//!
//! To make a new amplitude, we will first create a new struct and then implement
//! [`Node`](`crate::amplitude::Node`). Let's start with a trivial example, an amplitude which returns a
//! complex scalar. This particular amplitude is already implemented as a convenience struct called
//! [`ComplexScalar`](`crate::amplitude::ComplexScalar`).
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//!
//! #[derive(Clone)]
//! pub struct ComplexScalar;
//! impl<F: Field> Node<F> for ComplexScalar {
//!     fn calculate(&self, parameters: &[F], _event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
//!         Ok(Complex::new(parameters[0], parameters[1]))
//!     }
//!
//!     fn parameters(&self) -> Vec<String> {
//!         vec!["real".to_string(), "imag".to_string()]
//!     }
//! }
//!
//! ```
//!
//! For a second example, we can look at the precalculation feature. Here's an Dalitz-like
//! amplitude for the $`\omega`$ particle:
//! ```ignore
//! use rayon::prelude::*;
//! use rustitude_core::prelude::*;
//!
//! #[derive(Default, Clone)]
//! pub struct OmegaDalitz<F: Field> {
//!     dalitz_z: Vec<F>,
//!     dalitz_sin3theta: Vec<F>,
//!     lambda: Vec<F>,
//! }
//!
//! impl<F: Field> Node<F> for OmegaDalitz<F> {
//!     fn precalculate(&mut self, dataset: &Dataset<F>) -> Result<(), RustitudeError> {
//!         (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
//!             .events
//!             .par_iter()
//!             .map(|event| {
//!                 let pi0 = event.daughter_p4s[0];
//!                 let pip = event.daughter_p4s[1];
//!                 let pim = event.daughter_p4s[2];
//!                 let omega = pi0 + pip + pim;
//!
//!                 let dalitz_s = (pip + pim).m2();
//!                 let dalitz_t = (pip + pi0).m2();
//!                 let dalitz_u = (pim + pi0).m2();
//!
//!                 let m3pi = (F::TWO * pip.m()) + pi0.m();
//!                 let dalitz_d = F::TWO * omega.m() * (omega.m() - m3pi);
//!                 let dalitz_sc = (F::ONE / F::THREE) * (omega.m2() + pip.m2() + pim.m2() + pi0.m2());
//!                 let dalitz_x = F::fsqrt(F::THREE) * (dalitz_t - dalitz_u) / dalitz_d;
//!                 let dalitz_y = F::THREE * (dalitz_sc - dalitz_s) / dalitz_d;
//!
//!                 let dalitz_z = dalitz_x * dalitz_x + dalitz_y * dalitz_y;
//!                 let dalitz_sin3theta = F::fsin(F::THREE * F::fasin(dalitz_y / F::fsqrt(dalitz_z)));
//!
//!                 let pip_omega = pip.boost_along(&omega);
//!                 let pim_omega = pim.boost_along(&omega);
//!                 let pi_cross = pip_omega.momentum().cross(&pim_omega.momentum());
//!
//!                 let lambda = (F::FOUR / F::THREE) * F::fabs(pi_cross.dot(&pi_cross))
//!                     / ((F::ONE / F::NINE)
//!                         * (omega.m2() - (F::TWO * pip.m() + pi0.m()).fpowi(2)).fpowi(2));
//!
//!                 (dalitz_z, (dalitz_sin3theta, lambda))
//!             })
//!             .unzip();
//!         Ok(())
//!     }
//!
//!     fn calculate(&self, parameters: &[F], event: &Event<F>) -> Result<Complex<F>, RustitudeError> {
//!         let dalitz_z = self.dalitz_z[event.index];
//!         let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
//!         let lambda = self.lambda[event.index];
//!         let alpha = parameters[0];
//!         let beta = parameters[1];
//!         let gamma = parameters[2];
//!         let delta = parameters[3];
//!         Ok(F::fsqrt(F::fabs(
//!             lambda
//!                 * (F::ONE
//!                     + F::TWO * alpha * dalitz_z
//!                     + F::TWO * beta * dalitz_z.fpowf(F::THREE / F::TWO) * dalitz_sin3theta
//!                     + F::TWO * gamma * dalitz_z.fpowi(2)
//!                     + F::TWO * delta * dalitz_z.fpowf(F::FIVE / F::TWO) * dalitz_sin3theta),
//!         ))
//!         .into())
//!     }
//!
//!     fn parameters(&self) -> Vec<String> {
//!         vec![
//!             "alpha".to_string(),
//!             "beta".to_string(),
//!             "gamma".to_string(),
//!             "delta".to_string(),
//!         ]
//!     }
//! }
//! ```
//! Note several of the generic features which allow this amplitude to be used with different
//! numeric data types. Because it isn't specifically written for 64-bit floats (`f64`s), we can
//! conduct analyses that use the same code with 32-bit floats (`f32`s), which saves on memory and
//! time while sacrificing a bit of precision. In fact, we can go a step further and conduct the
//! majority of an analysis in 32-bit mode, switching over to 64-bit mode when we actually get near
//! a solution and want the increased accuracy!
//!
//! The [`Field`] trait contains a few helper constants and functions to make this easier for those
//! who aren't as familiar with rust. Constants are provided for whole numbers between zero and ten
//! (inclusively), and the [`Field`] trait also contains a few mathematical constants like
//! [`Field::PI()`][`num::traits::FloatConst::PI()`] and
//! [`Field::SQRT_2()`][`num::traits::FloatConst::SQRT_2()`]. Most mathematical functions are
//! aliased with a leading "f" to simplify duplicated function definitions in the [`num::Float`]
//! and [`nalgebra::RealField`] traits. For instance, [`Field::fabs()`] calls
//! [`num::Float::abs()`], since the alternative would be to use the fully qualified name to
//! distinguish it from [`nalgebra::ComplexField::abs()`].
//!
//! # Combining Amplitudes into Models
//! We can use several operations to modify and combine amplitudes. Since amplitudes yield complex
//! values, the following convenience methods are provided:
//! [`real`](`amplitude::AmpLike::real`), and [`imag`](`amplitude::AmpLike::imag`) give the real and
//! imaginary part of the amplitude, respectively. Additionally, amplitudes can be added and multiplied
//! together using operator overloading. [`Model`](`amplitude::Model`)s implicitly take the
//! absolute square of each provided term in their constructor and add those results incoherently.
//!
//! To incoherently sum two [`Amplitude`](`amplitude::Amplitude`)s, say `amp1` and `amp2`, we would
//! first assume that we actually want the absolute square of the given term (or write our
//! amplitude as the square root of what we really want), and then include them both in our model:
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//! // Define amp1/amp2: Amplitude here...
//! let model = model!(amp1, amp2)
//! ```
//!
//! To reiterate, this would yield something like $`\left|\text{amp}_1\right|^2 + \left|\text{amp}_2\right|^2`$.
//!
//! The [`Scalar`](`crate::amplitude::Scalar`),
//! [`ComplexScalar`](`crate::amplitude::ComplexScalar`), and
//! [`PolarComplexScalar`](`crate::amplitude::PolarComplexScalar`) amplitudes all have convenience
//! methods, [`scalar`](`crate::amplitude::scalar`), [`cscalar`](`crate::amplitude::cscalar`), and
//! [`pcscalar`](`crate::amplitude::pcscalar`) respectively. We then wrap the final expression in a
//! [`Model`](crate::amplitude::Model) which can manage all of the
//! [`Parameter`](`crate::amplitude::Parameter`)s.
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//!
//! #[derive(Default)]
//! pub struct OmegaDalitz { ... }
//! impl Node for OmegaDalitz { ... }
//!
//! let complex_term = cscalar("my complex scalar");
//! let omega_dalitz = Amplitude::new("omega dalitz", OmegaDalitz::default());
//! let term = complex_term * omega_dalitz;
//! term.print_tree();
//! // [ norm sqr ]
//! //   ┗━[ * ]
//! //       ┣━ !my complex scalar(real, imag)
//! //       ┗━ !omega dalitz(alpha, beta, gamma, delta)
//! let model = model!(term);
//! ```
//!
//! # Managing Parameters
//!
//! Now that we have a model, we might want to constrain or fix parameters. Parameters are
//! identified solely by their name and the name of the amplitude they are associated with. This
//! means that two amplitudes with the same name will share parameters which also have the same
//! name. If we want to intentionally set one parameter in a particular amplitude equal to another,
//! we can use the [`Model::constrain`](`crate::amplitude::Model::constrain`). This will reduce the
//! number of free parameters in the fit, and will yield a
//! [`RustitudeError`](`crate::errors::RustitudeError`) if either of the parameters is not found.
//! Parameters can also be fixed and freed using [`Model::fix`](`crate::amplitude::Model::fix`) and
//! [`Model::free`](`crate::amplitude::Model::free`) respectively, and these methods are mirrored in
//! [`Manager`](`crate::manager::Manager`) and
//! [`ExtendedLogLikelihood`](`crate::manager::ExtendedLogLikelihood`) for convenience.
//!
//! # Evaluating Likelihoods
//!
//! If we wanted to obtain the negative log-likelihood for this particular amplitude, we need to
//! link our [`Model`](`crate::amplitude::Model`) to a [`Dataset`](`crate::dataset::Dataset`). This is done using a
//! [`Manager`](`crate::manager::Manager``). Finally, two [`Manager`](`crate::manager::Manager``)s may be combined into an
//! [`ExtendedLogLikelihood`](`crate::manager::ExtendedLogLikelihood`). Both of these manager-like structs have an
//! `evaluate` method that takes some parameters as a `&[f32]` (along with a [`usize`] for the
//! number of threads to use for the [`ExtendedLogLikelihood`](`crate::manager::ExtendedLogLikelihood`)).
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//!
//! #[derive(Default)]
//! pub struct OmegaDalitz { ... }
//! impl Node for OmegaDalitz { ... }
//!
//! fn main() -> Result<(), RustitudeError> {
//!     let complex_term = cscalar("my complex scalar");
//!     let omega_dalitz = Amplitude::new("omega dalitz", OmegaDalitz::default());
//!     let term = complex_term * omega_dalitz;
//!     let model = model!(term);
//!     let dataset = Dataset::from_parquet("path/to/file.parquet")?;
//!     let dataset_mc = Dataset::from_parquet("path/to/monte_carlo_file.parquet")?;
//!     let nll = ExtendedLogLikelihood::new(
//!         Manager::new(&model, &dataset),
//!         Manager::new(&model, &dataset_mc)
//!     );
//!     println!("NLL: {}", nll.evaluate(&nll.get_initial())?);
//!     Ok(())
//! }
//! ```
//!
//! # Fitting Amplitudes to Data
//!
//! Of course, the goal of all of this is to be able to construct a
//! [`Model`](`crate::amplitude::Model`), load up a [`Dataset`](`crate::dataset::Dataset`), create
//! an [`ExtendedLogLikelihood`](`crate::manager::ExtendedLogLikelihood`), and fit the model to
//! data. Here's an example to show how that might be accomplished:
//!
//! ```ignore
//! use ganesh::algorithms::NelderMead;
//! use ganesh::prelude::*;
//! use rustitude::gluex::harmonics::Zlm;
//! use rustitude::gluex::{
//!     resonances::BreitWigner,
//!     utils::{Frame, Reflectivity, Wave},
//! };
//! use rustitude::prelude::*;
//! fn main() -> Result<(), RustitudeError> {
//!     let a2_1320 = BreitWigner::new(&[0], &[1], 2).named("a2_1320");
//!     let a2_1700 = BreitWigner::new(&[0], &[1], 2).named("a2_1700");
//!     let pw_s_wave = piecewise_m("pw_s_wave", 40, (1.04, 1.72));
//!     let zlm_s0p = Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity).named("zlm_s0p");
//!     let zlm_s0n = Zlm::new(Wave::S0, Reflectivity::Negative, Frame::Helicity).named("zlm_s0n");
//!     let zlm_dn2p = Zlm::new(Wave::Dn2, Reflectivity::Positive, Frame::Helicity).named("zlm_dn2p");
//!     let zlm_dn1p = Zlm::new(Wave::Dn1, Reflectivity::Positive, Frame::Helicity).named("zlm_dn1p");
//!     let zlm_d0p = Zlm::new(Wave::D0, Reflectivity::Positive, Frame::Helicity).named("zlm_d0p");
//!     let zlm_d1p = Zlm::new(Wave::D1, Reflectivity::Positive, Frame::Helicity).named("zlm_d1p");
//!     let zlm_d2p = Zlm::new(Wave::D2, Reflectivity::Positive, Frame::Helicity).named("zlm_d2p");
//!     let zlm_dn2n = Zlm::new(Wave::Dn2, Reflectivity::Negative, Frame::Helicity).named("zlm_dn2n");
//!     let zlm_dn1n = Zlm::new(Wave::Dn1, Reflectivity::Negative, Frame::Helicity).named("zlm_dn1n");
//!     let zlm_d0n = Zlm::new(Wave::D0, Reflectivity::Negative, Frame::Helicity).named("zlm_d0n");
//!     let zlm_d1n = Zlm::new(Wave::D1, Reflectivity::Negative, Frame::Helicity).named("zlm_d1n");
//!     let zlm_d2n = Zlm::new(Wave::D2, Reflectivity::Negative, Frame::Helicity).named("zlm_d2n");
//!     let pos_d_wave = zlm_dn2p + zlm_dn1p + zlm_d0p + zlm_d1p + zlm_d2p;
//!     let neg_d_wave = zlm_dn2n + zlm_dn1n + zlm_d0n + zlm_d1n + zlm_d2n;
//!     let pos_real =
//!         zlm_s0p.real() * &pw_s_wave + &a2_1320 * &pos_d_wave.real() + &a2_1700 * &pos_d_wave.real();
//!     let pos_imag =
//!         zlm_s0p.imag() * &pw_s_wave + &a2_1320 * &pos_d_wave.imag() + &a2_1700 * &pos_d_wave.imag();
//!     let neg_real =
//!         zlm_s0n.real() * &pw_s_wave + &a2_1320 * &neg_d_wave.real() + &a2_1700 * &neg_d_wave.real();
//!     let neg_imag =
//!         zlm_s0n.imag() * &pw_s_wave + &a2_1320 * &neg_d_wave.imag() + &a2_1700 * &neg_d_wave.imag();
//!     let model = model!(pos_real, pos_imag, neg_real, neg_imag);
//!     let ds_data = Dataset::from_parquet("path/to/data.root", ReadMethod::EPSInBeam)?;
//!     let ds_accmc = Dataset::from_parquet("path/to/accmc.root", ReadMethod::EPSInBeam)?;
//!     let mut ell = ExtendedLogLikelihood::new(
//!         Manager::new(&model, &ds_data)?,
//!         Manager::new(&model, &ds_accmc)?,
//!     );
//!     ell.set_initial("a2_1320", "mass", 1.3182)?;
//!     ell.set_initial("a2_1320", "width", 0.1111)?;
//!     ell.fix("a2_1700", "mass", 1.698)?;
//!     ell.fix("a2_1700", "width", 0.265)?;
//!     ell.fix("pw_s_wave", "bin 10 im", 0.0)?;
//!
//!     let mut nm = NelderMead::new(ell.clone(), &ell.get_initial(), None);
//!     minimize!(nm, 1000)?; // Run 1000 steps
//!     let (best_pars, best_fx) = nm.best();
//!     for (par_name, par_value) in ell.free_parameters().iter().zip(best_pars) {
//!         println!("{} -> {} (NLL = {})", par_name, par_value, best_fx);
//!     }
//!     Ok(())
//! }
//! ```
#![warn(
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::doc_markdown,
    clippy::doc_link_with_quotes,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::perf,
    clippy::style,
    missing_docs
)]
#![allow(deprecated)]

use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};

use nalgebra::Vector3;
use num::{
    traits::{FloatConst, NumAssignOps},
    Float, FromPrimitive,
};
pub mod amplitude;
pub mod dataset;
pub mod four_momentum;
pub mod manager;
/// Recommended namespace for use and development.
pub mod prelude {
    pub use crate::amplitude::{
        cscalar, pcscalar, piecewise_m, scalar, AmpLike, Amplitude, AsTree, Imag, Model, Node,
        Parameter, Piecewise, Product, Real, Sum,
    };
    pub use crate::dataset::{Dataset, Event, ReadMethod};
    pub use crate::errors::RustitudeError;
    pub use crate::four_momentum::FourMomentum;
    pub use crate::manager::{ExtendedLogLikelihood, Manager};
    pub use crate::{convert, convert_array, convert_vec, model, Field, UnitVector};
    pub use nalgebra::Vector3;
    pub use num::Complex;
}

/// A trait representing a numeric field which can be used in calculating amplitudes.
pub trait Field:
    Float
    + Sum
    + Product
    + FloatConst
    + NumAssignOps
    + Debug
    + Display
    + Default
    + Send
    + Sync
    + FromPrimitive
{
}
impl Field for f64 {}
impl Field for f32 {}

#[macro_export]
/// Convenience macro for converting raw numeric values to a generic.
macro_rules! convert {
    ($value:expr, $type:ty) => {{
        #[allow(clippy::unwrap_used)]
        <$type as num::NumCast>::from($value).unwrap()
    }};
}

#[macro_export]
/// Convenience macro for converting a raw numeric [`Vec`] to a generic [`Vec`].
macro_rules! convert_vec {
    ($vec:expr, $type:ty) => {{
        $vec.into_iter()
            .map(|value| $crate::convert!(value, $type))
            .collect::<Vec<$type>>()
    }};
}

#[macro_export]
/// Convenience macro for converting a raw numeric array to a generic array.
macro_rules! convert_array {
    ($arr:expr, $type:ty) => {{
        let temp_vec: Vec<_> = $arr
            .iter()
            .map(|&value| $crate::convert!(value, $type))
            .collect();
        #[allow(clippy::unwrap_used)]
        temp_vec.try_into().unwrap()
    }};
}

/// A trait to normalize structs (mostly to use on nalgebra vectors without needing [`nalgebra::RealField`])
pub trait UnitVector {
    /// Returns a normalized form of the input.
    fn unit(&self) -> Self;
}

impl<F: Field + 'static> UnitVector for Vector3<F> {
    fn unit(&self) -> Self {
        let mag = F::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self / mag
    }
}

#[macro_export]
/// Convenience macro for boxing up coherent sum terms into a [`Model`](`crate::amplitude::Model`).
macro_rules! model {
    ($($term:expr),+ $(,)?) => {
        Model::new(&[$(Box::new($term),)+])
    };
}

pub mod errors {
    //! This module contains an all-encompassing error enum that almost every crate method will
    //! produce if it returns a Result.
    use pyo3::{exceptions::PyException, PyErr};
    use thiserror::Error;

    /// The main [`Error`] structure for `rustitude_core`. All errors internal to the crate should
    /// eventually pass through here, since it provides a single-location interface for `PyO3`
    /// errors.
    #[derive(Debug, Error)]
    pub enum RustitudeError {
        #[allow(missing_docs)]
        #[error(transparent)]
        IOError(#[from] std::io::Error),

        #[allow(missing_docs)]
        #[error(transparent)]
        ParquetError(#[from] parquet::errors::ParquetError),

        #[allow(missing_docs)]
        #[error("Oxyroot: {0}")]
        OxyrootError(String),

        #[allow(missing_docs)]
        #[error(transparent)]
        ThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),

        #[allow(missing_docs)]
        #[error("Could not cast value from {0} (type in file) to {1} (required type)")]
        DatasetReadError(String, String),

        #[allow(missing_docs)]
        #[error("Parameter not found: {0}")]
        ParameterNotFoundError(String),

        #[allow(missing_docs)]
        #[error("Amplitude not found: {0}")]
        AmplitudeNotFoundError(String),

        #[allow(missing_docs)]
        #[error("Invalid parameter value: {0}")]
        InvalidParameterValue(String),

        #[allow(missing_docs)]
        #[error("Evaluation error: {0}")]
        EvaluationError(String),

        #[allow(missing_docs)]
        #[error("Python error: {0}")]
        PythonError(String),

        #[allow(missing_docs)]
        #[error("Parsing error: {0}")]
        ParseError(String),
    }
    impl From<RustitudeError> for PyErr {
        fn from(err: RustitudeError) -> Self {
            PyException::new_err(err.to_string())
        }
    }
    impl From<PyErr> for RustitudeError {
        fn from(err: PyErr) -> Self {
            Self::PythonError(err.to_string())
        }
    }
}

pub mod utils {
    //! This module holds some convenience methods for writing nice test functions for Amplitudes.
    use crate::prelude::*;

    /// Generate a test event for the reaction $`\gamma p \to K_S K_S p`$ with 64-bit precision.
    pub fn generate_test_event_f64() -> Event<f64> {
        Event {
            index: 0,
            weight: -0.48,
            beam_p4: FourMomentum::new(8.747_921, 0.0, 0.0, 8.747_921),
            recoil_p4: FourMomentum::new(1.040_902_7, 0.119_110_32, 0.373_947_23, 0.221_585_83),
            daughter_p4s: vec![
                FourMomentum::new(3.136_247_2, -0.111_774_68, 0.293_426_28, 3.080_557_3),
                FourMomentum::new(5.509_043, -0.007_335_639, -0.667_373_54, 5.445_778),
            ],
            eps: Vector3::from([0.385_109_57, 0.022_205_278, 0.0]),
        }
    }

    /// Generate a test dataset for the reaction $`\gamma p \to K_S K_S p`$ with 64-bit precision.
    pub fn generate_test_dataset_f64() -> Dataset<f64> {
        Dataset::new(vec![
            Event {
                index: 0,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.383_563, 0.0, 0.0, 8.383_563),
                recoil_p4: FourMomentum::new(1.311_736, 0.664_397, 0.327_881, 0.539_785),
                daughter_p4s: vec![
                    FourMomentum::new(3.140_736, -0.074_363, 0.335_501, 3.081_966),
                    FourMomentum::new(4.869_362, -0.590_033, -0.663_383, 4.761_812),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 1,
                weight: 0.967_937,
                beam_p4: FourMomentum::new(8.373_471, 0.0, 0.0, 8.373_471),
                recoil_p4: FourMomentum::new(1.099_134, -0.318_113, -0.241_351, 0.410_238),
                daughter_p4s: vec![
                    FourMomentum::new(6.803_817, 0.662_458, -0.146_496, 6.751_592),
                    FourMomentum::new(1.408_791, -0.344_344, 0.387_849, 1.211_640),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 2,
                weight: 0.016_893,
                beam_p4: FourMomentum::new(8.686_482, 0.0, 0.0, 8.686_482),
                recoil_p4: FourMomentum::new(1.041_158, 0.141_536, 0.374_024, 0.209_115),
                daughter_p4s: vec![
                    FourMomentum::new(3.348_294, -0.007_810, 0.232_603, 3.302_921),
                    FourMomentum::new(5.235_301, -0.133_726, -0.606_628, 5.174_445),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 3,
                weight: -0.022_154,
                beam_p4: FourMomentum::new(8.799_066, 0.0, 0.0, 8.799_066),
                recoil_p4: FourMomentum::new(1.078_011, -0.411_542, 0.243_270, 0.230_664),
                daughter_p4s: vec![
                    FourMomentum::new(5.382_554, 0.240_169, 0.105_882, 5.353_071),
                    FourMomentum::new(3.276_772, 0.171_372, -0.349_153, 3.215_329),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 4,
                weight: 0.012_900,
                beam_p4: FourMomentum::new(8.561_700, 0.0, 0.0, 8.561_700),
                recoil_p4: FourMomentum::new(1.078_375, -0.409_737, 0.245_940, 0.232_739),
                daughter_p4s: vec![
                    FourMomentum::new(5.221_115, 0.242_604, 0.099_132, 5.190_736),
                    FourMomentum::new(3.200_482, 0.167_133, -0.345_072, 3.138_225),
                ],
                eps: Vector3::from([-0.016_448, 0.324_690, 0.0]),
            },
            Event {
                index: 5,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.714_853, 0.0, 0.0, 8.714_853),
                recoil_p4: FourMomentum::new(1.458_814, -0.309_093, -0.853_077, 0.651_541),
                daughter_p4s: vec![
                    FourMomentum::new(3.879_303, -0.067_345, 0.225_269, 3.840_064),
                    FourMomentum::new(4.315_006, 0.376_439, 0.627_807, 4.223_246),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 6,
                weight: 1.111_018,
                beam_p4: FourMomentum::new(8.271_341, 0.0, 0.0, 8.271_341),
                recoil_p4: FourMomentum::new(1.296_389, -0.275_474, 0.706_565, 0.474_499),
                daughter_p4s: vec![
                    FourMomentum::new(5.433_060, 0.203_167, -0.343_429, 5.395_489),
                    FourMomentum::new(2.480_163, 0.072_306, -0.363_136, 2.401_352),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 7,
                weight: 1.111_339,
                beam_p4: FourMomentum::new(8.743_071, 0.0, 0.0, 8.743_071),
                recoil_p4: FourMomentum::new(1.126_252, -0.317_043, 0.461_564, 0.273_006),
                daughter_p4s: vec![
                    FourMomentum::new(5.651_356, 0.200_123, -0.228_232, 5.621_215),
                    FourMomentum::new(2.903_734, 0.116_919, -0.233_331, 2.848_849),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 8,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.657_957, 0.0, 0.0, 8.657_957),
                recoil_p4: FourMomentum::new(1.125_095, -0.315_415, 0.460_129, 0.272_539),
                daughter_p4s: vec![
                    FourMomentum::new(5.604_545, 0.200_701, -0.230_638, 5.574_032),
                    FourMomentum::new(2.866_588, 0.114_713, -0.229_491, 2.811_384),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 9,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.403_684, 0.0, 0.0, 8.403_684),
                recoil_p4: FourMomentum::new(1.109_429, 0.481_598, -0.076_590, 0.335_673),
                daughter_p4s: vec![
                    FourMomentum::new(1.882_555, -0.201_094, -0.392_549, 1.761_210),
                    FourMomentum::new(6.349_971, -0.280_504, 0.469_139, 6.306_800),
                ],
                eps: Vector3::from([-0.016_448, 0.324_690, 0.0]),
            },
        ])
    }

    /// Generate a test event for the reaction $`\gamma p \to K_S K_S p`$ with 32-bit precision.
    pub fn generate_test_event_f32() -> Event<f32> {
        Event {
            index: 0,
            weight: -0.48,
            beam_p4: FourMomentum::new(8.747_921, 0.0, 0.0, 8.747_921),
            recoil_p4: FourMomentum::new(1.040_902_7, 0.119_110_32, 0.373_947_23, 0.221_585_83),
            daughter_p4s: vec![
                FourMomentum::new(3.136_247_2, -0.111_774_68, 0.293_426_28, 3.080_557_3),
                FourMomentum::new(5.509_043, -0.007_335_639, -0.667_373_54, 5.445_778),
            ],
            eps: Vector3::from([0.385_109_57, 0.022_205_278, 0.0]),
        }
    }

    /// Generate a test dataset for the reaction $`\gamma p \to K_S K_S p`$ with 32-bit precision.
    pub fn generate_test_dataset_f32() -> Dataset<f32> {
        Dataset::new(vec![
            Event {
                index: 0,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.383_563, 0.0, 0.0, 8.383_563),
                recoil_p4: FourMomentum::new(1.311_736, 0.664_397, 0.327_881, 0.539_785),
                daughter_p4s: vec![
                    FourMomentum::new(3.140_736, -0.074_363, 0.335_501, 3.081_966),
                    FourMomentum::new(4.869_362, -0.590_033, -0.663_383, 4.761_812),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 1,
                weight: 0.967_937,
                beam_p4: FourMomentum::new(8.373_471, 0.0, 0.0, 8.373_471),
                recoil_p4: FourMomentum::new(1.099_134, -0.318_113, -0.241_351, 0.410_238),
                daughter_p4s: vec![
                    FourMomentum::new(6.803_817, 0.662_458, -0.146_496, 6.751_592),
                    FourMomentum::new(1.408_791, -0.344_344, 0.387_849, 1.211_64),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 2,
                weight: 0.016_893,
                beam_p4: FourMomentum::new(8.686_482, 0.0, 0.0, 8.686_482),
                recoil_p4: FourMomentum::new(1.041_158, 0.141_536, 0.374_024, 0.209_115),
                daughter_p4s: vec![
                    FourMomentum::new(3.348_294, -0.007_810, 0.232_603, 3.302_921),
                    FourMomentum::new(5.235_301, -0.133_726, -0.606_628, 5.174_445),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 3,
                weight: -0.022_154,
                beam_p4: FourMomentum::new(8.799_066, 0.0, 0.0, 8.799_066),
                recoil_p4: FourMomentum::new(1.078_011, -0.411_542, 0.243_270, 0.230_664),
                daughter_p4s: vec![
                    FourMomentum::new(5.382_554, 0.240_169, 0.105_882, 5.353_071),
                    FourMomentum::new(3.276_772, 0.171_372, -0.349_153, 3.215_329),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 4,
                weight: 0.012_900,
                beam_p4: FourMomentum::new(8.561_70, 0.0, 0.0, 8.561_7),
                recoil_p4: FourMomentum::new(1.078_375, -0.409_737, 0.245_940, 0.232_739),
                daughter_p4s: vec![
                    FourMomentum::new(5.221_115, 0.242_604, 0.099_132, 5.190_736),
                    FourMomentum::new(3.200_482, 0.167_133, -0.345_072, 3.138_225),
                ],
                eps: Vector3::from([-0.016_448, 0.324_690, 0.0]),
            },
            Event {
                index: 5,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.714_853, 0.0, 0.0, 8.714_853),
                recoil_p4: FourMomentum::new(1.458_814, -0.309_093, -0.853_077, 0.651_541),
                daughter_p4s: vec![
                    FourMomentum::new(3.879_303, -0.067_345, 0.225_269, 3.840_064),
                    FourMomentum::new(4.315_006, 0.376_439, 0.627_807, 4.223_246),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 6,
                weight: 1.111_018,
                beam_p4: FourMomentum::new(8.271_341, 0.0, 0.0, 8.271_341),
                recoil_p4: FourMomentum::new(1.296_389, -0.275_474, 0.706_565, 0.474_499),
                daughter_p4s: vec![
                    FourMomentum::new(5.433_06, 0.203_167, -0.343_429, 5.395_489),
                    FourMomentum::new(2.480_163, 0.072_306, -0.363_136, 2.401_352),
                ],
                eps: Vector3::from([-0.016_172, 0.319_243, 0.0]),
            },
            Event {
                index: 7,
                weight: 1.111_339,
                beam_p4: FourMomentum::new(8.743_071, 0.0, 0.0, 8.743_071),
                recoil_p4: FourMomentum::new(1.126_252, -0.317_043, 0.461_564, 0.273_006),
                daughter_p4s: vec![
                    FourMomentum::new(5.651_356, 0.200_123, -0.228_232, 5.621_215),
                    FourMomentum::new(2.903_734, 0.116_919, -0.233_331, 2.848_849),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 8,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.657_957, 0.0, 0.0, 8.657_957),
                recoil_p4: FourMomentum::new(1.125_095, -0.315_415, 0.460_129, 0.272_539),
                daughter_p4s: vec![
                    FourMomentum::new(5.604_545, 0.200_701, -0.230_638, 5.574_032),
                    FourMomentum::new(2.866_588, 0.114_713, -0.229_491, 2.811_384),
                ],
                eps: Vector3::from([-0.018_940, 0.373_890, 0.0]),
            },
            Event {
                index: 9,
                weight: -0.138_917,
                beam_p4: FourMomentum::new(8.403_684, 0.0, 0.0, 8.403_684),
                recoil_p4: FourMomentum::new(1.109_429, 0.481_598, -0.076_590, 0.335_673),
                daughter_p4s: vec![
                    FourMomentum::new(1.882_555, -0.201_094, -0.392_549, 1.761_21),
                    FourMomentum::new(6.349_971, -0.280_504, 0.469_139, 6.306_80),
                ],
                eps: Vector3::from([-0.016_448, 0.324_690, 0.0]),
            },
        ])
    }

    /// Checks if two floating point numbers are essentially equal.
    /// See [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/).
    pub fn is_close<F: Field>(a: F, b: F, epsilon: F) -> bool {
        let abs_a = F::abs(a);
        let abs_b = F::abs(b);
        let diff = F::abs(a - b);
        if a == b {
            true
        } else if a == F::zero() || b == F::zero() || (abs_a + abs_b < F::min_positive_value()) {
            diff < (epsilon * F::min_positive_value())
        } else {
            diff / F::min(abs_a + abs_b, F::max_value()) < epsilon
        }
    }

    /// A macro to assert if two floating point numbers are essentially equal. Similar to [`approx`] crate.
    #[macro_export]
    macro_rules! assert_is_close {
        ($given:expr, $expected:expr, f64) => {
            let abs_a = f64::abs($given);
            let abs_b = f64::abs($expected);
            let diff = f64::abs($given - $expected);
            let abs_diff = diff / f64::min(abs_a + abs_b, f64::MAX);
            match (&($given), &($expected)) {
                (given, expected) => assert!(
                    $crate::utils::is_close(f64::from(*given), *expected, 1e-5),
                    "assert_is_close!({}, {})

    a = {:?}
    b = {:?}
    |a - b| / (|a| + |b|) = {:?} > 1e-5

",
                    stringify!($given),
                    stringify!($expected),
                    given,
                    expected,
                    abs_diff
                ),
            }
        };
        ($given:expr, $expected:expr, f32) => {
            let abs_a = f32::abs($given);
            let abs_b = f32::abs($expected);
            let diff = f32::abs($given - $expected);
            let abs_diff = diff / f32::min(abs_a + abs_b, f32::MAX);
            match (&($given), &($expected)) {
                (given, expected) => assert!(
                    $crate::utils::is_close(f32::from(*given), *expected, 1e-5),
                    "assert_is_close!({}, {})

    a = {:?}
    b = {:?}
    |a - b| / (|a| + |b|) = {:?} > 1e-5

",
                    stringify!($given),
                    stringify!($expected),
                    given,
                    expected,
                    abs_diff
                ),
            }
        };
        ($given:expr, $expected:expr, $eps:expr, f64) => {
            let abs_a = f64::abs($given);
            let abs_b = f64::abs($expected);
            let diff = f64::abs($given - $expected);
            let abs_diff = diff / f64::min(abs_a + abs_b, f64::MAX);
            match (&($given), &($expected), &($eps)) {
                (given, expected, eps) => assert!(
                    $crate::utils::is_close(*given, *expected, *eps),
                    "assert_is_close!({}, {}, {})

    a = {:?}
    b = {:?}
    |a - b| / (|a| + |b|) = {:?} > {:?}

",
                    stringify!($given),
                    stringify!($expected),
                    stringify!($eps),
                    given,
                    expected,
                    abs_diff,
                    eps
                ),
            }
        };
        ($given:expr, $expected:expr, $eps:expr, f32) => {
            let abs_a = f32::abs($given);
            let abs_b = f32::abs($expected);
            let diff = f32::abs($given - $expected);
            let abs_diff = diff / f32::min(abs_a + abs_b, f32::MAX);
            match (&($given), &($expected), &($eps)) {
                (given, expected, eps) => assert!(
                    $crate::utils::is_close(*given, *expected, *eps),
                    "assert_is_close!({}, {}, {})

    a = {:?}
    b = {:?}
    |a - b| / (|a| + |b|) = {:?} > {:?}

",
                    stringify!($given),
                    stringify!($expected),
                    stringify!($eps),
                    given,
                    expected,
                    abs_diff,
                    eps
                ),
            }
        };
    }
}
