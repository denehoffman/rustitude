//! # Rustitude
//! ## Demystifying Amplitude Analysis with Rust and Python
//!
//! The `rustitude` crate is an interface for both using amplitudes written in Rust and for writing
//! new amplitudes. There is quite a bit of additional documentation in the
//! [`rustitude-core`](`rustitude_core`) crate, and this crate just re-exports the prelude as well
//! as the amplitudes located in the [`rustitude-gluex`](`rustitude_gluex`) crate. See the
//! respective crates for more in-depth information.
pub mod prelude {
    pub use rustitude_core::prelude::*;
}

#[cfg(feature = "gluex")]
pub mod gluex {
    pub use rustitude_gluex::*;
}
