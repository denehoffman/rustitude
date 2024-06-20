//! # Rustitude
//! ## Demystifying Amplitude Analysis with Rust and Python
//!
//! The `rustitude-core` crate aims to implement common amplitude analysis techniques in Rust with
//! bindings to Python. This crate does not include the Python bindings, see the [GitHub
//! repo](https://github.com/denehoffman/rustitude) for more information on the Python API.
//!
//! The three core principles of `rustitude-core` are:
//! 1. Parallelization over events is automatically handeled by a [`Manager`](crate::manager::Manager).
//! 2. Amplitudes are written to do as much work as possible ahead of time, and evaluations use
//!    caching as much as possible automatically.
//! 3. Developers just need to implement the [`Node`](crate::amplitude::Node) trait to write a new
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
//! A [`Dataset`](crate::dataset::Dataset) is essentially just a wrapper for a [`Vec`] of
//! [`Event`](crate::dataset::Event)s. The current [`Event`](crate::dataset::Event) structure is as follows:
//!
//! ```ignore
//! pub struct Event {
//!     pub index: usize,                    // Position of event within dataset
//!     pub weight: f64,                     // Event weight
//!     pub beam_p4: FourMomentum,           // Beam four-momentum
//!     pub recoil_p4: FourMomentum,         // Recoil four-momentum
//!     pub daughter_p4s: Vec<FourMomentum>, // Four-momenta of final state particles sans recoil
//!     pub eps: Vector3<f64>,               // Beam polarization vector
//! }
//! ```
//!
//! In the Rust API, we can create [`Dataset`](crate::dataset::Dataset)s from `ROOT` files as well as
//! `Parquet` files. `ROOT` file reading is done through [`oxyroot`] - This still has some issues,
//! and large files or files with user metadata might fail to load. The alternative `Parquet`
//! format can be obtained from a `ROOT` file by using a conversion script like the one provided
//! [here](https://github.com/denehoffman/rustitude/blob/main/bin/convert). By default, we expect
//! all of the [`Event`](crate::dataset::Event) fields to be mirrored as the following branches:
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
//! using [`Dataset::from_parquet_eps_in_beam`](crate::dataset::Dataset::from_parquet_eps_in_beam).
//!
//! # Creating a New Amplitude
//!
//! To make a new amplitude, we will first create a new struct and then implement
//! [`Node`](crate::amplitude::Node). Let's start with a trivial example, an amplitude which returns a
//! complex scalar. This particular amplitude is already implemented as a convenience struct called
//! [`ComplexScalar`](crate::amplitude::ComplexScalar).
//!
//! ```ignore
//! use rustitude_core::prelude::*
//! struct ComplexScalar;
//! impl Node for ComplexScalar {
//!     fn calculate(&self, parameters: &[f64], _event: &Event) -> Result<Complex64, RustitudeError> {
//!         Ok(Complex64::new(parameters[0], parameters[1]))
//!     }
//!
//!     fn parameters(&self) -> Vec<String> {
//!         vec!["real".to_string(), "imag".to_string()]
//!     }
//! }
//! ```
//!
//! For a second example, we can look at the precalculation feature. Here's an Dalitz-like
//! amplitude for the $`\omega`$ particle:
//! ```ignore
//! use rayon::prelude::*;
//! use rustitude_core::prelude::*;
//!
//! #[derive(Default)]
//! pub struct OmegaDalitz {
//!     dalitz_z: Vec<f64>,
//!     dalitz_sin3theta: Vec<f64>,
//!     lambda: Vec<f64>,
//! }
//!
//! impl Node for OmegaDalitz {
//!     fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
//!         (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
//!             .events
//!             .read()
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
//!                 let m3pi = (2.0 * pip.m()) + pi0.m();
//!                 let dalitz_d = 2.0 * omega.m() * (omega.m() - m3pi);
//!                 let dalitz_sc = (1.0 / 3.0) * (omega.m2() + pip.m2() + pim.m2() + pi0.m2());
//!                 let dalitz_x = f64::sqrt(3.0) * (dalitz_t - dalitz_u) / dalitz_d;
//!                 let dalitz_y = 3.0 * (dalitz_sc - dalitz_s) / dalitz_d;
//!
//!                 let dalitz_z = dalitz_x * dalitz_x + dalitz_y * dalitz_y;
//!                 let dalitz_sin3theta = f64::sin(3.0 * f64::asin(dalitz_y / f64::sqrt(dalitz_z)));
//!
//!                 let pip_omega = pip.boost_along(&omega);
//!                 let pim_omega = pim.boost_along(&omega);
//!                 let pi_cross = pip_omega.momentum().cross(&pim_omega.momentum());
//!
//!                 let lambda = (4.0 / 3.0) * f64::abs(pi_cross.dot(&pi_cross))
//!                     / ((1.0 / 9.0) * (omega.m2() - (2.0 * pip.m() + pi0.m()).powi(2)).powi(2));
//!
//!                 (dalitz_z, (dalitz_sin3theta, lambda))
//!             })
//!             .unzip();
//!         Ok(())
//!     }
//!
//!     fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
//!         let dalitz_z = self.dalitz_z[event.index];
//!         let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
//!         let lambda = self.lambda[event.index];
//!         let alpha = parameters[0];
//!         let beta = parameters[1];
//!         let gamma = parameters[2];
//!         let delta = parameters[3];
//!         Ok(f64::sqrt(f64::abs(
//!             lambda
//!                 * (1.0
//!                     + 2.0 * alpha * dalitz_z
//!                     + 2.0 * beta * dalitz_z.powf(3.0 / 2.0) * dalitz_sin3theta
//!                     + 2.0 * gamma * dalitz_z.powi(2)
//!                     + 2.0 * delta * dalitz_z.powf(5.0 / 2.0) * dalitz_sin3theta),
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
//! # Combining Amplitudes into Models
//! We can use several operations to modify and combine amplitudes. Since amplitudes yield complex
//! values, the following convenience methods are provided:
//! [`real`](`amplitude::AmpLike::real`), and [`imag`](`amplitude::AmpLike::imag`) give the real and
//! imaginary part of the amplitude, respectively. Additionally, amplitudes can be added and multiplied
//! together using operator overloading. All sums are interpreted as
//! [coherent sums](`crate::amplitude::CohSum`), and products with these coherent sums are
//! distributed. Incoherent sums are handled at the [`Model`](crate::amplitude::Model) level.
//!
//! To incoherently sum two [`Amplitude`](`amplitude::Amplitude`)s, say `amp1` and `amp2`, we would
//! first assume that we actually want the absolute square of the given term (or write our
//! amplitude as the square root of what we really want), and then include them both in our model:
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//! // Define amp1/amp2: Amplitude here...
//! let model = Model::new(vec![amp1.as_cohsum(), amp2.as_cohsum()])
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
//! let omega_dalitz = amplitude!("omega dalitz", OmegaDalitz::default());
//! let term = (complex_term * omega_dalitz).norm_sqr();
//! term.print_tree();
//! // [ norm sqr ]
//! //   ┗━[ * ]
//! //       ┣━ !my complex scalar(real, imag)
//! //       ┗━ !omega dalitz(alpha, beta, gamma, delta)
//! let model = Model::new(term);
//! ```
//!
//! # Managing Parameters
//!
//! Now that we have a model, we might want to constrain or fix parameters. Parameters are
//! identified solely by their name and the name of the amplitude they are associated with. This
//! means that two amplitudes with the same name will share parameters which also have the same
//! name. If we want to intentionally set one parameter in a particular amplitude equal to another,
//! we can use the [`Model::constrain`](crate::amplitude::Model::constrain). This will reduce the
//! number of free parameters in the fit, and will yield a
//! [`RustitudeError`](crate::errors::RustitudeError) if either of the parameters is not found.
//! Parameters can also be fixed and freed using [`Model::fix`](crate::amplitude::Model::fix) and
//! [`Model::free`](crate::amplitude::Model::free) respectively, and these methods are mirrored in
//! [`Manager`](crate::manager::Manager) and
//! [`ExtendedLogLikelihood`](crate::manager::ExtendedLogLikelihood) for convenience.
//!
//! # Evaluating Likelihoods
//!
//! If we wanted to obtain the negative log-likelihood for this particular amplitude, we need to
//! link our [`Model`](crate::amplitude::Model) to a [`Dataset`](crate::dataset::Dataset). This is done using a
//! [`Manager`](crate::manager::Manager). Finally, two [`Manager`](crate::manager::Manager)s may be combined into an
//! [`ExtendedLogLikelihood`](crate::manager::ExtendedLogLikelihood). Both of these manager-like structs have an
//! `evaluate` method that takes some parameters as a `&[f64]` (along with a [`usize`] for the
//! number of threads to use for the [`ExtendedLogLikelihood`](crate::manager::ExtendedLogLikelihood)).
//!
//! ```ignore
//! use rustitude_core::prelude::*;
//!
//! #[derive(Default)]
//! pub struct OmegaDalitz { ... }
//! impl Node for OmegaDalitz { ... }
//!
//! let complex_term = cscalar("my complex scalar");
//! let omega_dalitz = amplitude!("omega dalitz", OmegaDalitz::default());
//! let term = (complex_term * omega_dalitz).norm_sqr();
//! let model = Model::new(term);
//! let dataset = Dataset::from_parquet("path/to/file.parquet").unwrap();
//! let dataset_mc = Dataset::from_parquet("path/to/monte_carlo_file.parquet").unwrap();
//! let nll = ExtendedLogLikelihood::new(
//!         Manager::new(&model, &dataset),
//!         Manager::new(&model, &dataset_mc)
//!     );
//! println!("NLL on 4 threads: {}", nll.evaluate(&nll.get_initial(), 4));
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
#![cfg_attr(feature = "simd", feature(portable_simd))]
pub mod amplitude;
pub mod dataset;
pub mod four_momentum;
pub mod manager;
/// Recommended namespace for use and development.
pub mod prelude {
    pub use crate::amplitude::{
        cscalar, pcscalar, scalar, AmpLike, Amplitude, AsTree, CohSum, Imag, Model, Node,
        Parameter, Piecewise, Product, Real,
    };
    pub use crate::dataset::{Dataset, Event};
    pub use crate::errors::RustitudeError;
    pub use crate::four_momentum::FourMomentum;
    pub use crate::manager::{ExtendedLogLikelihood, Manager};
    pub use nalgebra::Vector3;
    pub use num_complex::Complex64;
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

    /// Generate a test event for the reaction $`\gamma p \to K_S K_S p`$.
    pub fn generate_test_event() -> Event {
        Event {
            index: 0,
            weight: -0.48,
            beam_p4: FourMomentum::new(8.747920989990234, 0.0, 0.0, 8.747920989990234),
            recoil_p4: FourMomentum::new(
                1.0409027338027954,
                0.11911032348871231,
                0.37394723296165466,
                0.22158582508563995,
            ),
            daughter_p4s: vec![
                FourMomentum::new(
                    3.136247158050537,
                    -0.11177468299865723,
                    0.2934262752532959,
                    3.080557346343994,
                ),
                FourMomentum::new(
                    5.509043216705322,
                    -0.007335639093071222,
                    -0.667373538017273,
                    5.445777893066406,
                ),
            ],
            eps: Vector3::from([0.3851095736026764, 0.022205278277397156, 0.0]),
        }
    }

    /// Checks if two floating point numbers are essentially equal.
    /// See [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/).
    pub fn is_close(a: f64, b: f64, epsilon: f64) -> bool {
        let abs_a = f64::abs(a);
        let abs_b = f64::abs(b);
        let diff = f64::abs(a - b);
        if a == b {
            true
        } else if a == 0.0 || b == 0.0 || (abs_a + abs_b < f64::MIN_POSITIVE) {
            diff < (epsilon * f64::MIN_POSITIVE)
        } else {
            diff / f64::min(abs_a + abs_b, f64::MAX) < epsilon
        }
    }

    /// A macro to assert if two floating point numbers are essentially equal. Similar to [`approx`] crate.
    #[macro_export]
    macro_rules! assert_is_close {
        ($given:expr, $expected:expr) => {
            match (&($given), &($expected)) {
                (given, expected) => assert!(
                    is_close(*given, *expected, 1e-7),
                    "assert_is_close!({}, {})

    a = {:?}
    b = {:?}

",
                    stringify!($given),
                    stringify!($expected),
                    given,
                    expected
                ),
            }
        };
        ($given:expr, $expected:expr, $eps:expr) => {
            match (&($given), &($expected), &($eps)) {
                (given, expected, eps) => assert!(
                    is_close(*given, *expected, *eps),
                    "assert_is_close!({}, {}, {})

    a = {:?}
    b = {:?}

",
                    stringify!($given),
                    stringify!($expected),
                    stringify!($eps),
                    given,
                    expected
                ),
            }
        };
    }
}

/// Creates a new thread pool.
///
/// This method uses [`rayon`] to create a thread pool with a given number of threads.
///
/// Arguments:
/// * `num_threads`: Number of threads to use in the pool
///
/// # Errors
///
/// Will yield a [`errors::RustitudeError`] which forwards a [`rayon::ThreadPoolBuildError`] if
/// there is any issue creating the thread pool.
pub fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, errors::RustitudeError> {
    Ok(rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()?)
}
