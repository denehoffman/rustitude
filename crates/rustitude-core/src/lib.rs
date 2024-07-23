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
    pub use crate::{model, Field};
    // pub use crate::{constants::*, ComplexField, Field};
    pub use nalgebra::{ComplexField, RealField, Vector3};
    pub use num::Complex;
}

/// A trait which describes a field of "Real" numbers which can be used in calculating amplitudes.
pub trait Field:
    nalgebra::RealField
    + std::iter::Sum
    + std::iter::Product
    + Copy
    + Clone
    + Default
    + ganesh::core::Field
    + num::Float
    + num::traits::FloatConst
{
    /// See [`f64::MIN_POSITIVE`]
    const MIN_POSITIVE: Self;
    /// See [`f64::MAX`]
    const MAX: Self;
    /// See [`f64::MIN`]
    const MIN: Self;
    /// See [`f64::INFINITY`]
    const INFINITY: Self;
    /// See [`f64::NEG_INFINITY`]
    const NEG_INFINITY: Self;
    /// Alias for 0.0
    const ZERO: Self;
    /// Alias for 1.0
    const ONE: Self;
    /// Alias for 2.0
    const TWO: Self;
    /// Alias for 3.0
    const THREE: Self;
    /// Alias for 4.0
    const FOUR: Self;
    /// Alias for 5.0
    const FIVE: Self;
    /// Alias for 6.0
    const SIX: Self;
    /// Alias for 7.0
    const SEVEN: Self;
    /// Alias for 8.0
    const EIGHT: Self;
    /// Alias for 9.0
    const NINE: Self;
    /// Alias for 10.0
    const TEN: Self;
    /// Shorthand to convert an `f64` into a [`Field`].
    /// See also: [`Field::convert_f64`].
    fn f(x: f64) -> Self {
        Self::convert_f64(x)
    }
    /// Shorthand to convert a [`Vec<f64>`] into a [`Vec<Field>`].
    /// See also: [`Field::convert_vec_f64`].
    fn fv(x: Vec<f64>) -> Vec<Self> {
        Self::convert_vec_f64(x)
    }
    /// Shorthand to convert a `[f64; N]` into a `[Field; N]`.
    /// See also: [`Field::convert_array_f64`].
    fn fa<const N: usize>(x: [f64; N]) -> [Self; N] {
        Self::convert_array_f64(x)
    }
    /// Converts a `[f64; N]` into a `[Field; N]`.
    fn convert_array_f64<const N: usize>(x: [f64; N]) -> [Self; N] {
        std::array::from_fn(|i| Self::convert_f64(x[i]))
    }
    /// Converts a [`Vec<f64>`] into a [`Vec<Field>`].
    fn convert_vec_f64(x: Vec<f64>) -> Vec<Self> {
        x.into_iter().map(Self::f).collect()
    }
    /// Converts an `f64` into a [`Field`].
    fn convert_f64(x: f64) -> Self;
    /// Converts an `f32` into a [`Field`].
    fn convert_f32(x: f32) -> Self;
    /// Converts a `usize` into a [`Field`].
    fn convert_usize(x: usize) -> Self;
    /// Converts an `isize` into a [`Field`].
    fn convert_isize(x: isize) -> Self;
    /// Converts a `u32` into a [`Field`].
    fn convert_u32(x: u32) -> Self;
    /// Shorthand for [`num::Float::abs`].
    fn fabs(self) -> Self {
        num::Float::abs(self)
    }
    /// Shorthand for [`num::Float::sqrt`].
    fn fsqrt(self) -> Self {
        num::Float::sqrt(self)
    }
    /// Shorthand for [`num::Float::cbrt`].
    fn fcbrt(self) -> Self {
        num::Float::cbrt(self)
    }
    /// Shorthand for [`num::Float::powi`].
    fn fpowi(self, n: i32) -> Self {
        num::Float::powi(self, n)
    }
    /// Shorthand for [`num::Float::powf`].
    fn fpowf(self, n: Self) -> Self {
        num::Float::powf(self, n)
    }
    /// Shorthand for [`num::Float::sin`].
    fn fsin(self) -> Self {
        num::Float::sin(self)
    }
    /// Shorthand for [`num::Float::cos`].
    fn fcos(self) -> Self {
        num::Float::cos(self)
    }
    /// Shorthand for [`num::Float::tan`].
    fn ftan(self) -> Self {
        num::Float::tan(self)
    }
    /// Shorthand for [`num::Float::asin`].
    fn fasin(self) -> Self {
        num::Float::asin(self)
    }
    /// Shorthand for [`num::Float::acos`].
    fn facos(self) -> Self {
        num::Float::acos(self)
    }
    /// Shorthand for [`num::Float::atan`].
    fn fatan(self) -> Self {
        num::Float::atan(self)
    }
    /// Shorthand for [`nalgebra::RealField::atan2`].
    fn fatan2(self, other: Self) -> Self {
        nalgebra::RealField::atan2(self, other)
    }
    /// Shorthand for [`num::Float::sinh`].
    fn fsinh(self) -> Self {
        num::Float::sinh(self)
    }
    /// Shorthand for [`num::Float::cosh`].
    fn fcosh(self) -> Self {
        num::Float::cosh(self)
    }
    /// Shorthand for [`num::Float::tanh`].
    fn ftanh(self) -> Self {
        num::Float::tanh(self)
    }
    /// Shorthand for [`num::Float::asinh`].
    fn fasinh(self) -> Self {
        num::Float::asinh(self)
    }
    /// Shorthand for [`num::Float::acosh`].
    fn facosh(self) -> Self {
        num::Float::acosh(self)
    }
    /// Shorthand for [`num::Float::atanh`].
    fn fatanh(self) -> Self {
        num::Float::atanh(self)
    }
    /// Shorthand for [`num::Float::log`].
    fn flog(self, base: Self) -> Self {
        num::Float::log(self, base)
    }
    /// Shorthand for [`num::Float::log2`].
    fn flog2(self) -> Self {
        num::Float::log2(self)
    }
    /// Shorthand for [`num::Float::log10`].
    fn flog10(self) -> Self {
        num::Float::log10(self)
    }
    /// Shorthand for [`num::Float::ln`].
    fn fln(self) -> Self {
        num::Float::ln(self)
    }
    /// Shorthand for [`num::Float::ln_1p`].
    fn fln_1p(self) -> Self {
        num::Float::ln_1p(self)
    }
    /// Shorthand for [`num::Float::exp`].
    fn fexp(self) -> Self {
        num::Float::exp(self)
    }
    /// Shorthand for [`num::Float::exp2`].
    fn fexp2(self) -> Self {
        num::Float::exp2(self)
    }
    /// Shorthand for [`num::Float::exp_m1`].
    fn fexp_m1(self) -> Self {
        num::Float::exp_m1(self)
    }
    /// Shorthand for [`num::Float::hypot`].
    fn fhypot(self, other: Self) -> Self {
        num::Float::hypot(self, other)
    }
    /// Shorthand for [`num::Float::recip`].
    fn frecip(self) -> Self {
        num::Float::recip(self)
    }
    /// Shorthand for [`num::Float::mul_add`].
    fn fmul_add(self, a: Self, b: Self) -> Self {
        num::Float::mul_add(self, a, b)
    }
    /// Shorthand for [`num::Float::floor`].
    fn ffloor(self) -> Self {
        num::Float::floor(self)
    }
    /// Shorthand for [`num::Float::ceil`].
    fn fceil(self) -> Self {
        num::Float::ceil(self)
    }
    /// Shorthand for [`num::Float::round`].
    fn fround(self) -> Self {
        num::Float::round(self)
    }
    /// Shorthand for [`num::Float::trunc`].
    fn ftrunc(self) -> Self {
        num::Float::trunc(self)
    }
    /// Shorthand for [`num::Float::fract`].
    fn ffract(self) -> Self {
        num::Float::fract(self)
    }
    /// Shorthand for [`num::Float::min`].
    fn fmin(self, other: Self) -> Self {
        num::Float::min(self, other)
    }
    /// Shorthand for [`num::Float::max`].
    fn fmax(self, other: Self) -> Self {
        num::Float::max(self, other)
    }
}

impl Field for f64 {
    const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const INFINITY: Self = Self::INFINITY;
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const FOUR: Self = 4.0;
    const FIVE: Self = 5.0;
    const SIX: Self = 6.0;
    const SEVEN: Self = 7.0;
    const EIGHT: Self = 8.0;
    const NINE: Self = 9.0;
    const TEN: Self = 10.0;

    fn convert_f64(x: f64) -> Self {
        x
    }

    fn convert_f32(x: f32) -> Self {
        x as Self
    }

    fn convert_usize(x: usize) -> Self {
        x as Self
    }

    fn convert_isize(x: isize) -> Self {
        x as Self
    }
    fn convert_u32(x: u32) -> Self {
        x as Self
    }
}
impl Field for f32 {
    const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const INFINITY: Self = Self::INFINITY;
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const FOUR: Self = 4.0;
    const FIVE: Self = 5.0;
    const SIX: Self = 6.0;
    const SEVEN: Self = 7.0;
    const EIGHT: Self = 8.0;
    const NINE: Self = 9.0;
    const TEN: Self = 10.0;

    fn convert_f64(x: f64) -> Self {
        x as Self
    }

    fn convert_f32(x: f32) -> Self {
        x
    }

    fn convert_usize(x: usize) -> Self {
        x as Self
    }

    fn convert_isize(x: isize) -> Self {
        x as Self
    }

    fn convert_u32(x: u32) -> Self {
        x as Self
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

    /// Checks if two floating point numbers are essentially equal.
    /// See [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/).
    pub fn is_close<F: Field>(a: F, b: F, epsilon: F) -> bool {
        let abs_a = F::fabs(a);
        let abs_b = F::fabs(b);
        let diff = F::fabs(a - b);
        if a == b {
            true
        } else if a == F::ZERO || b == F::ZERO || (abs_a + abs_b < F::MIN_POSITIVE) {
            diff < (epsilon * F::MIN_POSITIVE)
        } else {
            diff / F::fmin(abs_a + abs_b, F::MAX) < epsilon
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
