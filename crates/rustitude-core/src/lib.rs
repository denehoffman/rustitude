#![warn(clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
pub mod amplitude;
pub mod dataset;
pub mod four_momentum;
pub mod manager;
pub mod prelude {
    pub use crate::amplitude;
    pub use crate::amplitude::{
        cscalar, pcscalar, scalar, AmpOp, Amplitude, Model, Node, Parameter, Piecewise,
    };
    pub use crate::dataset::{Dataset, Event};
    pub use crate::errors::RustitudeError;
    pub use crate::four_momentum::FourMomentum;
    pub use crate::manager::{ExtendedLogLikelihood, Manager};
    pub use num_complex::Complex64;
}

pub mod errors {
    use pyo3::{exceptions::PyException, PyErr};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum RustitudeError {
        #[error(transparent)]
        IOError(#[from] std::io::Error),

        #[error(transparent)]
        ParquetError(#[from] parquet::errors::ParquetError),

        #[error("Oxyroot: {0}")]
        OxyrootError(String),

        #[error(transparent)]
        ThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),

        #[error("Parameter not found: {0}")]
        ParameterNotFoundError(String),

        #[error("Amplitude not found: {0}")]
        AmplitudeNotFoundError(String),

        #[error("invalid parameter value")]
        InvalidParameterValue(String),

        #[error("evaluation error")]
        EvaluationError(String),
    }
    impl From<RustitudeError> for PyErr {
        fn from(err: RustitudeError) -> Self {
            PyException::new_err(err.to_string())
        }
    }
}

pub fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, errors::RustitudeError> {
    Ok(rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()?)
}
