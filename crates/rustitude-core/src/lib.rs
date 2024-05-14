#![warn(clippy::nursery)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
pub mod amplitude;
pub mod dataset;
pub mod four_momentum;
pub mod manager;
pub mod prelude {
    pub use crate::amplitude;
    pub use crate::amplitude::{
        cscalar, pcscalar, scalar, AmpOp, Amplitude, Model, Node, NodeError, Parameter, Piecewise,
        PyAmpOp,
    };
    pub use crate::dataset::{Dataset, Event};
    pub use crate::four_momentum::FourMomentum;
    pub use crate::manager::{ExtendedLogLikelihood, Manager};
    pub use num_complex::Complex64;
}

#[derive(Debug)]
pub struct RustitudeThreadError;

pub fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, RustitudeThreadError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .or(Err(RustitudeThreadError))
}
