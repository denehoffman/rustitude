//! The amplitude module contains structs and methods for defining and manipulating [`Amplitude`]s
//! and [`Model`]s
//!
//! To create a new [`Amplitude`] in Rust, we simply need to implement the [`Node`] trait on a
//! struct. You can provide a convenience method for creating a new implementation of your
//! [`Amplitude`], or you can use the [`crate::amplitude!`] macro as a shortcut.
//!
//! Amplitudes are typically defined first, and then [`Model`]s are built by multiplying and
//! operating on [`Amplitude`]s as [`AmpOp`]s. These [`AmpOp`]s are summed into [`CohSum`]s
//! (coherent sums), and these are then summed by a [`Model`].
//!
//! We can then use [`Manager`](crate::manager::Manager)-like structs to handle computataion
//! over [`Dataset`]s.
use itertools::{iproduct, Itertools};
use nalgebra::ComplexField;
use num::complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    ops::{Add, Mul},
    sync::Arc,
};

use crate::{
    dataset::{Dataset, Event},
    errors::RustitudeError,
};

/// A single parameter within an [`Amplitude`].
#[derive(Clone)]
pub struct Parameter {
    /// Name of the parent [`Amplitude`] containing this parameter.
    pub amplitude: String,
    /// Name of the parameter.
    pub name: String,
    /// Index of the parameter with respect to the [`Model`]. This will be [`Option::None`] if
    /// the parameter is fixed.
    pub index: Option<usize>,
    /// A separate index for fixed parameters to ensure they stay constrained properly if freed.
    /// This will be [`Option::None`] if the parameter is free in the [`Model`].
    pub fixed_index: Option<usize>,
    /// The initial value the parameter takes, or alternatively the value of the parameter if it is
    /// fixed in the fit.
    pub initial: f64,
    /// Bounds for the given parameter (defaults to +/- infinity). This is mostly optional and
    /// isn't used in any Rust code asside from being able to get and set it.
    pub bounds: (f64, f64),
}
impl Parameter {
    /// Creates a new [`Parameter`] within an [`Amplitude`] using the name of the [`Amplitude`],
    /// the name of the [`Parameter`], and the index of the parameter within the [`Model`].
    ///
    /// By default, new [`Parameter`]s are free, have an initial value of `0.0`, and their bounds
    /// are set to `(f64::NEG_INFINITY, f64::INFINITY)`.
    pub fn new(amplitude: &str, name: &str, index: usize) -> Self {
        Self {
            amplitude: amplitude.to_string(),
            name: name.to_string(),
            index: Some(index),
            fixed_index: None,
            initial: 0.0,
            bounds: (f64::NEG_INFINITY, f64::INFINITY),
        }
    }
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.index.is_none() {
            write!(
                f,
                "< {} >[ {} (*{}*) ]({:?})({:?})",
                self.amplitude, self.name, self.initial, self.index, self.fixed_index,
            )
        } else {
            write!(
                f,
                "< {} >[ {} ({}) ]({:?})({:?})",
                self.amplitude, self.name, self.initial, self.index, self.fixed_index,
            )
        }
    }
}
impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.index.is_none() {
            write!(
                f,
                "<{}>[ {} (*{}*) ]",
                self.amplitude, self.name, self.initial
            )
        } else {
            write!(
                f,
                "<{}>[ {} ({}) ]",
                self.amplitude, self.name, self.initial
            )
        }
    }
}

/// Creates a wrapped [`AmpOp`] which can be registered by a [`crate::amplitude::Model`].
///
/// This macro is a convenience method which takes a name and a [`Node`] and generates a new [`AmpOp`].
#[macro_export]
macro_rules! amplitude {
    ($name:expr, $node:expr) => {{
        Amplitude::new($name, $node).into()
    }};
}

/// A trait which contains all the required methods for a functioning [`Amplitude`].
///
/// The [`Node`] trait represents any mathematical structure which takes in some parameters and some
/// [`Event`] data and computes a [`Complex64`] for each [`Event`]. This is the fundamental
/// building block of all analyses built with Rustitude. Nodes are intended to be optimized at the
/// user level, so they should be implemented on structs which can store some precalculated data.
///
/// # Examples:
///
/// A [`Node`] for calculating spherical harmonics:
///
/// ```
/// use rustitude_core::prelude::*;
///
/// use nalgebra::{SMatrix, SVector};
/// use num_complex::Complex64;
/// use rayon::prelude::*;
/// use sphrs::SHEval;
/// use sphrs::{ComplexSH, Coordinates};
///
/// #[derive(Clone, Copy, Default)]
/// #[rustfmt::skip]
/// enum Wave {
///     #[default]
///     S,
///     S0,
///     Pn1, P0, P1, P,
///     Dn2, Dn1, D0, D1, D2, D,
///     Fn3, Fn2, Fn1, F0, F1, F2, F3, F,
/// }
///
/// #[rustfmt::skip]
/// impl Wave {
///     fn l(&self) -> i64 {
///         match self {
///             Self::S0 | Self::S => 0,
///             Self::Pn1 | Self::P0 | Self::P1 | Self::P => 1,
///             Self::Dn2 | Self::Dn1 | Self::D0 | Self::D1 | Self::D2 | Self::D => 2,
///             Self::Fn3 | Self::Fn2 | Self::Fn1 | Self::F0 | Self::F1 | Self::F2 | Self::F3 | Self::F => 3,
///         }
///     }
///     fn m(&self) -> i64 {
///         match self {
///             Self::S | Self::P | Self::D | Self::F => 0,
///             Self::S0 | Self::P0 | Self::D0 | Self::F0 => 0,
///             Self::Pn1 | Self::Dn1 | Self::Fn1 => -1,
///             Self::P1 | Self::D1 | Self::F1 => 1,
///             Self::Dn2 | Self::Fn2 => -2,
///             Self::D2 | Self::F2 => 2,
///             Self::Fn3 => -3,
///             Self::F3 => 3,
///         }
///     }
/// }
///
/// struct Ylm(Wave, Vec<Complex64>);
/// impl Ylm {
///     fn new(wave: Wave) -> Self {
///         Self(wave, Vec::default())
///     }
/// }
/// impl Node for Ylm {
///     fn parameters(&self) -> Vec<String> { vec![] }
///     fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
///         self.1 = dataset.events.read()
///             .par_iter()
///             .map(|event| {
///                 let resonance = event.daughter_p4s[0] + event.daughter_p4s[1];
///                 let p1 = event.daughter_p4s[0];
///                 let recoil_res = event.recoil_p4.boost_along(&resonance); // Boost to helicity frame
///                 let p1_res = p1.boost_along(&resonance);
///                 let z = -1.0 * recoil_res.momentum().normalize();
///                 let y = event
///                     .beam_p4
///                     .momentum()
///                     .cross(&(-1.0 * event.recoil_p4.momentum()));
///                 let x = y.cross(&z);
///                 let p1_vec = p1_res.momentum();
///                 let p = Coordinates::cartesian(p1_vec.dot(&x), p1_vec.dot(&y), p1_vec.dot(&z));
///                 ComplexSH::Spherical.eval(self.0.l(), self.0.m(), &p)
///             })
///             .collect();
///         Ok(())
///     }
///
///     fn calculate(&self, _parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
///         Ok(self.1[event.index])
///     }
/// }
/// ```
///
/// A [`Node`] which computes a single complex scalar entirely determined by input parameters:
///
/// ```
/// use rustitude_core::prelude::*;
/// struct ComplexScalar;
/// impl Node for ComplexScalar {
///     fn calculate(&self, parameters: &[f64], _event: &Event) -> Result<Complex64, RustitudeError> {
///         Ok(Complex64::new(parameters[0], parameters[1]))
///     }
///
///     fn parameters(&self) -> Vec<String> {
///         vec!["real".to_string(), "imag".to_string()]
///     }
/// }
/// ```
pub trait Node: Sync + Send {
    /// A method that is run once and stores some precalculated values given a [`Dataset`] input.
    ///
    /// This method is intended to run expensive calculations which don't actually depend on the
    /// parameters. For instance, to calculate a spherical harmonic, we don't actually need any
    /// other information than what is contained in the [`Event`], so we can calculate a spherical
    /// harmonic for every event once and then retrieve the data in the [`Node::calculate`] method.
    ///
    /// # Errors
    ///
    /// This function should be written to return a [`RustitudeError`] if any part of the
    /// calculation fails.
    fn precalculate(&mut self, _dataset: &Dataset) -> Result<(), RustitudeError> {
        Ok(())
    }

    /// A method which runs every time the amplitude is evaluated and produces a [`Complex64`].
    ///
    /// Because this method is run on every evaluation, it should be as lean as possible.
    /// Additionally, you should avoid [`rayon`]'s parallel loops inside this method since we
    /// already parallelize over the [`Dataset`]. This method expects a single [`Event`] as well as
    /// a slice of [`f64`]s. This slice is guaranteed to have the same length and order as
    /// specified in the [`Node::parameters`] method, or it will be empty if that method returns
    /// [`None`].
    ///
    /// # Errors
    ///
    /// This function should be written to return a [`RustitudeError`] if any part of the
    /// calculation fails.
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError>;

    /// A method which specifies the number and order of parameters used by the [`Node`].
    ///
    /// This method tells the [`crate::manager::Manager`] how to assign its input [`Vec`] of parameter values to
    /// each [`Node`]. If this method returns [`None`], it is implied that the [`Node`] takes no
    /// parameters as input. Otherwise, the parameter names should be listed in the same order they
    /// are expected to be given as input to the [`Node::calculate`] method.
    fn parameters(&self) -> Vec<String> {
        vec![]
    }
}

/// An enum for storing individual [`Amplitude`]s as well as products of [`AmpOp`]s or the real or
/// imaginary part of a single [`AmpOp`]. Sums of [`AmpOp`]s are handeled by the [`CohSum`] struct.
///
/// These structs follow some rules for addition and multiplication:
///
/// > `AmpOp + AmpOp = CohSum`
/// >
/// > `AmpOp + CohSum = CohSum` (appending)
/// >
/// > `CohSum + CohSum = CohSum` (concatenating)
///
/// > `AmpOp * AmpOp = AmpOp`
/// >
/// > `AmpOp * CohSum = CohSum` (`AmpOp` is distributed over terms of `CohSum`)
/// >
/// > `CohSum * CohSum = UNDEFINED`
///
/// This format should be able to handle any generalized intensity equation based on amplitude
/// nodes. For instance, if an incoherent sum is needed, two separate [`CohSum`]s can be created
/// and input as terns in the same [`Model`], since the results are summed normally there.
#[derive(Clone)]
pub enum AmpOp {
    /// An [`Amplitude`] defined by the user.
    Amplitude(Amplitude),
    /// The product of a set of [`AmpOp`]s.
    Product(Vec<AmpOp>),
    /// The real part of an [`AmpOp`].
    Real(Box<AmpOp>),
    /// The imag part of an [`AmpOp`].
    Imag(Box<AmpOp>),
}

impl Debug for AmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amplitude(amp) => writeln!(f, "{:?}", amp),
            Self::Product(ops) => {
                write!(f, "Prod [ ")?;
                for op in ops {
                    write!(f, "{:?} ", op)?;
                }
                write!(f, "]")
            }
            Self::Real(op) => write!(f, "Re[{:?}]", op),
            Self::Imag(op) => write!(f, "Im[{:?}]", op),
        }
    }
}
impl Display for AmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amplitude(amp) => writeln!(f, "{}", amp),
            Self::Product(ops) => {
                write!(f, "Prod [ ")?;
                for op in ops {
                    write!(f, "{} ", op)?;
                }
                write!(f, "]")
            }
            Self::Real(op) => write!(f, "Re[{:?}]", op),
            Self::Imag(op) => write!(f, "Im[{:?}]", op),
        }
    }
}
impl AmpOp {
    /// Pretty prints a tree diagram to show the node structure of the [`AmpOp`].
    pub fn print_tree(&self) {
        self._print_tree(vec![]);
    }
    fn _print_indent(bits: &[bool]) {
        bits.iter()
            .for_each(|b| if *b { print!("  ┃ ") } else { print!("    ") });
    }
    fn _print_intermediate() {
        print!("  ┣━");
    }
    fn _print_end() {
        print!("  ┗━");
    }
    fn _print_tree(&self, mut bits: Vec<bool>) {
        match self {
            Self::Amplitude(amp) => {
                if amp.parameters().len() > 7 {
                    println!(
                        " {}{}({},...)",
                        if amp.active { "!" } else { "" },
                        amp.name,
                        amp.parameters()[0..7].join(", ")
                    );
                } else {
                    println!(
                        " {}{}({})",
                        if amp.active { "!" } else { "" },
                        amp.name,
                        amp.parameters().join(", ")
                    );
                }
            }
            Self::Product(ops) => {
                println!("[ * ]");
                for (i, op) in ops.iter().enumerate() {
                    Self::_print_indent(&bits);
                    if i == ops.len() - 1 {
                        Self::_print_end();
                        bits.push(false);
                    } else {
                        Self::_print_intermediate();
                        bits.push(true);
                    }
                    op._print_tree(bits.clone());
                    bits.pop();
                }
            }
            Self::Real(op) => {
                println!("[ real ]");
                Self::_print_indent(&bits);
                Self::_print_end();
                bits.push(false);
                op._print_tree(bits.clone());
                bits.pop();
            }
            Self::Imag(op) => {
                println!("[ imag ]");
                Self::_print_indent(&bits);
                Self::_print_end();
                bits.push(false);
                op._print_tree(bits.clone());
                bits.pop();
            }
        }
    }
    /// Walks through an [`AmpOp`] and collects all the contained [`Amplitude`]s recursively.
    pub fn walk(&self) -> Vec<Amplitude> {
        match self {
            Self::Amplitude(amp) => vec![amp.clone()],
            Self::Product(ops) => ops.iter().flat_map(|op| op.walk()).collect(),
            Self::Real(op) => op.walk(),
            Self::Imag(op) => op.walk(),
        }
    }
    /// Walks through an [`AmpOp`] and collects all the contained [`Amplitude`]s recursively. This
    /// method gives mutable access to said [`Amplitude`]s.
    pub fn walk_mut(&mut self) -> Vec<&mut Amplitude> {
        match self {
            Self::Amplitude(amp) => vec![amp],
            Self::Product(ops) => ops.iter_mut().flat_map(|op| op.walk_mut()).collect(),
            Self::Real(op) => op.walk_mut(),
            Self::Imag(op) => op.walk_mut(),
        }
    }
    /// Shortcut for computation using a cache of precomputed values. This method will return
    /// [`None`] if the cache value at the corresponding [`Amplitude`]'s
    /// [`Amplitude::cache_position`] is also [`None`], otherwise it just returns the corresponding
    /// cached value. Other branches of the enum will perform various operations, such as getting
    /// the product, real part, or imaginary part, and these will also have [`None`] values passed
    /// through.
    pub fn compute(&self, cache: &[Option<Complex64>]) -> Option<Complex64> {
        match self {
            Self::Amplitude(amp) => cache[amp.cache_position],
            Self::Product(ops) => Some(ops.iter().filter_map(|op| op.compute(cache)).product()),
            Self::Real(op) => op.compute(cache).map(|r| r.re.into()),
            Self::Imag(op) => op.compute(cache).map(|r| r.im.into()),
        }
    }
    /// Converts an [`AmpOp`] into a [`AmpOp::Real`] containing that [`AmpOp`].
    pub fn real(&self) -> Self {
        Self::Real(Box::new(self.clone()))
    }
    /// Converts an [`AmpOp`] into a [`AmpOp::Imag`] containing that [`AmpOp`].
    pub fn imag(&self) -> Self {
        Self::Imag(Box::new(self.clone()))
    }
}
impl Add for AmpOp {
    type Output = CohSum;

    fn add(self, rhs: Self) -> Self::Output {
        CohSum(vec![self, rhs])
    }
}
impl Add<AmpOp> for &AmpOp {
    type Output = <AmpOp as Add>::Output;

    fn add(self, rhs: AmpOp) -> Self::Output {
        AmpOp::add(self.clone(), rhs)
    }
}
impl Add<&Self> for AmpOp {
    type Output = <Self as Add>::Output;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(self, rhs.clone())
    }
}
impl Add for &AmpOp {
    type Output = <AmpOp as Add>::Output;

    fn add(self, rhs: Self) -> Self::Output {
        AmpOp::add(self.clone(), rhs.clone())
    }
}

impl Add<CohSum> for AmpOp {
    type Output = CohSum;

    fn add(self, rhs: CohSum) -> Self::Output {
        CohSum([vec![self], rhs.0].concat())
    }
}
impl Add<AmpOp> for CohSum {
    type Output = Self;

    fn add(self, rhs: AmpOp) -> Self::Output {
        Self([self.0, vec![rhs]].concat())
    }
}
impl Add<Self> for CohSum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0, rhs.0].concat())
    }
}

impl Mul for AmpOp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Self::Product(ops_l), Self::Product(ops_r)) => Self::Product([ops_l, ops_r].concat()),
            (Self::Product(ops), _) => {
                let mut sum_ops = ops;
                sum_ops.push(rhs);
                Self::Product(sum_ops)
            }
            (_, Self::Product(ops)) => {
                let mut sum_ops = ops;
                sum_ops.push(self);
                Self::Product(sum_ops)
            }
            (_, _) => Self::Product(vec![self, rhs]),
        }
    }
}
impl Mul<AmpOp> for &AmpOp {
    type Output = <AmpOp as Mul>::Output;

    fn mul(self, rhs: AmpOp) -> Self::Output {
        AmpOp::mul(self.clone(), rhs)
    }
}
impl Mul<&Self> for AmpOp {
    type Output = <Self as Mul>::Output;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self::mul(self, rhs.clone())
    }
}
impl Mul for &AmpOp {
    type Output = <AmpOp as Mul>::Output;

    fn mul(self, rhs: Self) -> Self::Output {
        AmpOp::mul(self.clone(), rhs.clone())
    }
}

impl Mul<CohSum> for AmpOp {
    type Output = CohSum;

    fn mul(self, rhs: CohSum) -> Self::Output {
        CohSum(rhs.0.iter().map(|term| self.clone() * term).collect())
    }
}
impl Mul<AmpOp> for CohSum {
    type Output = Self;

    fn mul(self, rhs: AmpOp) -> Self::Output {
        Self(self.0.iter().map(|term| term * rhs.clone()).collect())
    }
}

impl From<AmpOp> for CohSum {
    fn from(op: AmpOp) -> Self {
        Self(vec![op])
    }
}

/// A struct which stores a named [`Node`].
///
/// The [`Amplitude`] struct turns a [`Node`] trait into a concrete type and also stores a name
/// associated with the [`Node`]. This allows us to distinguish multiple uses of the same [`Node`]
/// in an analysis, and makes each [`Node`]'s parameters unique.
///
/// This is mostly used interally as an intermediate step to an [`AmpOp`].
#[derive(Clone)]
pub struct Amplitude {
    /// A name which uniquely identifies an [`Amplitude`] within a sum and group.
    pub name: String,
    /// A [`Node`] which contains all of the operations needed to compute a [`Complex64`] from an
    /// [`Event`] in a [`Dataset`], a [`Vec<f64>`] of parameter values, and possibly some
    /// precomputed values.
    pub node: Arc<RwLock<Box<dyn Node>>>,
    /// Indicates whether the amplitude should be included in calculations or skipped.
    pub active: bool,
    /// Indicates the reserved position in the cache for shortcutting computation with a
    /// precomputed cache.
    pub cache_position: usize,
    /// Indicates the position in the final parameter vector that coincides with the starting index
    /// for parameters in this [`Amplitude`]
    pub parameter_index_start: usize,
}

impl Debug for Amplitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Amplitude")
            .field("name", &self.name)
            .field("active", &self.active)
            .field("cache_position", &self.cache_position)
            .field("parameter_index_start", &self.parameter_index_start)
            .finish()
    }
}
impl Display for Amplitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.active {
            write!(f, "{}", self.name)
        } else {
            write!(f, "# {} #", self.name)
        }
    }
}
impl From<Amplitude> for AmpOp {
    fn from(amp: Amplitude) -> Self {
        Self::Amplitude(amp)
    }
}
impl Amplitude {
    /// Creates a new [`Amplitude`] from a name and a [`Node`]-implementing struct.
    pub fn new(name: &str, node: impl Node + 'static) -> Self {
        Self {
            name: name.to_string(),
            node: Arc::new(RwLock::new(Box::new(node))),
            active: true,
            cache_position: 0,
            parameter_index_start: 0,
        }
    }
    /// Set the [`Amplitude::cache_position`] and [`Amplitude::parameter_index_start`] and runs
    /// [`Amplitude::precalculate`] over the given [`Dataset`].
    ///
    /// # Errors
    /// This function will raise a [`RustitudeError`] if the precalculation step fails.
    pub fn register(
        &mut self,
        cache_position: usize,
        parameter_index_start: usize,
        dataset: &Dataset,
    ) -> Result<(), RustitudeError> {
        self.cache_position = cache_position;
        self.parameter_index_start = parameter_index_start;
        self.precalculate(dataset)
    }
}
impl Node for Amplitude {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.node.write().precalculate(dataset)
    }
    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        self.node.read().calculate(
            &parameters
                [self.parameter_index_start..self.parameter_index_start + self.parameters().len()],
            event,
        )
    }
    fn parameters(&self) -> Vec<String> {
        self.node.read().parameters()
    }
}

/// Struct to hold a coherent sum of [`AmpOp`]s
#[derive(Clone)]
pub struct CohSum(Vec<AmpOp>);

impl Debug for CohSum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sum [ ")?;
        for op in &self.0 {
            write!(f, "{:?} ", op)?;
        }
        write!(f, "]")
    }
}
impl CohSum {
    /// Create a new [`CohSum`] from a [`Vec`] of terms ([`AmpOp`]s).
    pub fn new(terms: Vec<AmpOp>) -> Self {
        Self(terms)
    }

    /// Pretty prints a tree diagram to show the node structure of the [`CohSum`].
    pub fn print_tree(&self) {
        let bits = vec![true];
        println!("[ CohSum ]");
        for term in self.0.iter() {
            term._print_tree(bits.clone())
        }
    }
    /// Function which returns a sum of all cross terms inside a coherent sum.
    ///
    /// Take the following coherent sum, where $`\vec{p}`$ are input parameters $`e`$ is an
    /// event, and $`f_i`$ is the $`i`$th term in the sum:
    ///
    /// ```math
    /// \left| \sum_{i\in\text{terms}} f_i(\vec{p}, e) \right|^2
    /// ```
    ///
    /// This function will then return
    ///
    /// ```math
    /// \sum_{i\in\text{terms}} \sum_{j\in\text{terms}} f_i(\vec{p}, e) f_j^*(\vec{p}, e)
    /// ```
    ///
    /// This should be used to compute normalization integrals. Note that if on of the terms is
    /// [`None`], this function will not add any products which contain that term. This can be used
    /// to turn terms on and off.
    pub fn norm_int(&self, cache: &[Option<Complex64>]) -> Option<f64> {
        let results = self.0.iter().map(|op| op.compute(cache));
        iproduct!(results.clone(), results)
            .map(|terms| match terms {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a * b.conjugate()),
            })
            .sum::<Option<Complex64>>()
            .map(|val| val.re)
    }

    /// Shortcut for computation using a cache of precomputed values. This method will return
    /// [`None`] if the cache value at the corresponding [`Amplitude`]'s
    /// [`Amplitude::cache_position`] is also [`None`], otherwise it just returns the corresponding
    /// cached value. The computation is run across the [`CohSum`]'s terms, and the absolute square
    /// of the result is returned (coherent sum).
    pub fn compute(&self, cache: &[Option<Complex64>]) -> Option<f64> {
        self.0
            .iter()
            .map(|ampop| ampop.compute(cache))
            .sum::<Option<Complex64>>()
            .map(|val| val.norm_sqr())
    }

    /// Walks through a [`CohSum`] and collects all the contained [`Amplitude`]s recursively.
    pub fn walk(&self) -> Vec<Amplitude> {
        self.0.iter().flat_map(|op| op.walk()).collect()
    }

    /// Walks through an [`CohSum`] and collects all the contained [`Amplitude`]s recursively. This
    /// method gives mutable access to said [`Amplitude`]s.
    pub fn walk_mut(&mut self) -> Vec<&mut Amplitude> {
        self.0.iter_mut().flat_map(|op| op.walk_mut()).collect()
    }
}

/// A model contains an API to interact with a group of [`CohSum`]s by managing their amplitudes
/// and parameters. Models are typically passed to [`Manager`](crate::manager::Manager)-like
/// struct.
#[derive(Debug, Clone)]
pub struct Model {
    /// The set of coherent sums included in the [`Model`].
    pub cohsums: Vec<CohSum>,
    /// The unique amplitudes located within all [`CohSum`]s.
    pub amplitudes: Vec<Amplitude>,
    /// The unique parameters located within all [`CohSum`]s.
    pub parameters: Vec<Parameter>,
}

impl Model {
    /// Pretty-prints a tree diagram to show the node structure of the [`Model`].
    pub fn print_tree(&self) {
        for cohsum in &self.cohsums {
            cohsum.print_tree()
        }
    }
    /// Retrieves a copy of an [`Amplitude`] in the [`Model`] by name.
    ///
    /// # Errors
    /// This will throw a [`RustitudeError`] if the amplitude name is not located within the model.
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.amplitudes
            .iter()
            .find(|a: &&Amplitude| a.name == amplitude_name)
            .ok_or_else(|| RustitudeError::AmplitudeNotFoundError(amplitude_name.to_string()))
            .cloned()
    }
    /// Retrieves a copy of a [`Parameter`] in the [`Model`] by name.
    ///
    /// # Errors
    /// This will throw a [`RustitudeError`] if the parameter name is not located within the model
    /// or if the amplitude name is not located within the model (this is checked first).
    pub fn get_parameter(
        &self,
        amplitude_name: &str,
        parameter_name: &str,
    ) -> Result<Parameter, RustitudeError> {
        self.get_amplitude(amplitude_name)?;
        self.parameters
            .iter()
            .find(|p: &&Parameter| p.amplitude == amplitude_name && p.name == parameter_name)
            .ok_or_else(|| RustitudeError::ParameterNotFoundError(parameter_name.to_string()))
            .cloned()
    }
    /// Pretty-prints all parameters in the model
    pub fn print_parameters(&self) {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        if self.any_fixed() {
            println!(
                "Fixed: {}",
                self.group_by_index()[0]
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .join(", ")
            );
        }
        for (i, group) in self.group_by_index().iter().skip(any_fixed).enumerate() {
            println!(
                "{}: {}",
                i,
                group.iter().map(|p| format!("{:?}", p)).join(", ")
            );
        }
    }
    /// Constrains two [`Parameter`]s in the [`Model`] to be equal to each other when evaluated.
    ///
    /// # Errors
    ///
    /// This method will yield a [`RustitudeError`] if either of the parameters is not found by
    /// name.
    pub fn constrain(
        &mut self,
        amplitude_1: &str,
        parameter_1: &str,
        amplitude_2: &str,
        parameter_2: &str,
    ) -> Result<(), RustitudeError> {
        let p1 = self.get_parameter(amplitude_1, parameter_1)?;
        let p2 = self.get_parameter(amplitude_2, parameter_2)?;
        for par in self.parameters.iter_mut() {
            // None < Some(0)
            match p1.index.cmp(&p2.index) {
                // p1 < p2
                std::cmp::Ordering::Less => {
                    if par.index == p2.index {
                        par.index = p1.index;
                        par.initial = p1.initial;
                        par.fixed_index = p1.fixed_index;
                    }
                }
                std::cmp::Ordering::Equal => unimplemented!(),
                // p2 < p1
                std::cmp::Ordering::Greater => {
                    if par.index == p1.index {
                        par.index = p2.index;
                        par.initial = p2.initial;
                        par.fixed_index = p2.fixed_index;
                    }
                }
            }
        }
        self.reindex_parameters();
        Ok(())
    }

    /// Fixes a [`Parameter`] in the [`Model`] to a given value.
    ///
    /// This method technically sets the [`Parameter`] to be fixed and gives it an initial value of
    /// the given value. This method also handles groups of constrained parameters.
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if the parameter is not found by name.
    pub fn fix(
        &mut self,
        amplitude: &str,
        parameter: &str,
        value: f64,
    ) -> Result<(), RustitudeError> {
        let search_par = self.get_parameter(amplitude, parameter)?;
        let fixed_index = self.get_min_fixed_index();
        for par in self.parameters.iter_mut() {
            if par.index == search_par.index {
                par.index = None;
                par.initial = value;
                par.fixed_index = fixed_index;
            }
        }
        self.reindex_parameters();
        Ok(())
    }
    /// Frees a [`Parameter`] in the [`Model`].
    ///
    /// This method does not modify the initial value of the parameter. This method
    /// also handles groups of constrained parameters.
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if the parameter is not found by name.
    pub fn free(&mut self, amplitude: &str, parameter: &str) -> Result<(), RustitudeError> {
        let search_par = self.get_parameter(amplitude, parameter)?;
        let index = self.get_min_free_index();
        for par in self.parameters.iter_mut() {
            if par.fixed_index == search_par.fixed_index {
                par.index = index;
                par.fixed_index = None;
            }
        }
        self.reindex_parameters();
        Ok(())
    }
    /// Sets the bounds on a [`Parameter`] in the [`Model`].
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if the parameter is not found by name.
    pub fn set_bounds(
        &mut self,
        amplitude: &str,
        parameter: &str,
        bounds: (f64, f64),
    ) -> Result<(), RustitudeError> {
        let search_par = self.get_parameter(amplitude, parameter)?;
        if search_par.index.is_some() {
            for par in self.parameters.iter_mut() {
                if par.index == search_par.index {
                    par.bounds = bounds;
                }
            }
        } else {
            for par in self.parameters.iter_mut() {
                if par.fixed_index == search_par.fixed_index {
                    par.bounds = bounds;
                }
            }
        }
        Ok(())
    }
    /// Sets the initial value of a [`Parameter`] in the [`Model`].
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if the parameter is not found by name.
    pub fn set_initial(
        &mut self,
        amplitude: &str,
        parameter: &str,
        initial: f64,
    ) -> Result<(), RustitudeError> {
        let search_par = self.get_parameter(amplitude, parameter)?;
        if search_par.index.is_some() {
            for par in self.parameters.iter_mut() {
                if par.index == search_par.index {
                    par.initial = initial;
                }
            }
        } else {
            for par in self.parameters.iter_mut() {
                if par.fixed_index == search_par.fixed_index {
                    par.initial = initial;
                }
            }
        }
        Ok(())
    }
    /// Returns a list of bounds of free [`Parameter`]s in the [`Model`].
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        self.group_by_index()
            .iter()
            .skip(any_fixed)
            .filter_map(|group| group.first().map(|par| par.bounds))
            .collect()
    }
    /// Returns a list of initial values of free [`Parameter`]s in the [`Model`].
    pub fn get_initial(&self) -> Vec<f64> {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        self.group_by_index()
            .iter()
            .skip(any_fixed)
            .filter_map(|group| group.first().map(|par| par.initial))
            .collect()
    }
    /// Returns the number of free [`Parameter`]s in the [`Model`].
    pub fn get_n_free(&self) -> usize {
        self.get_min_free_index().unwrap_or(0)
    }
    /// Activates an [`Amplitude`] in the [`Model`] by name.
    pub fn activate(&mut self, amplitude: &str) {
        self.amplitudes.iter_mut().for_each(|amp| {
            if amp.name == amplitude {
                amp.active = true
            }
        })
    }
    /// Deactivates an [`Amplitude`] in the [`Model`] by name.
    pub fn deactivate(&mut self, amplitude: &str) {
        self.amplitudes.iter_mut().for_each(|amp| {
            if amp.name == amplitude {
                amp.active = false
            }
        })
    }
    /// Creates a new [`Model`] from a list of [`CohSum`]s.
    pub fn new(cohsums: Vec<CohSum>) -> Self {
        let mut amp_names = HashSet::new();
        let amplitudes: Vec<Amplitude> = cohsums
            .iter()
            .flat_map(|cohsum| cohsum.walk())
            .filter_map(|amp| {
                if amp_names.insert(amp.name.clone()) {
                    Some(amp)
                } else {
                    None
                }
            })
            .collect();
        let parameter_tags: Vec<(String, String)> = amplitudes
            .iter()
            .flat_map(|amp| {
                amp.parameters()
                    .iter()
                    .map(|p| (amp.name.clone(), p.clone()))
                    .collect::<Vec<_>>()
            })
            .collect();
        let parameters = parameter_tags
            .iter()
            .enumerate()
            .map(|(i, (amp_name, par_name))| Parameter::new(amp_name, par_name, i))
            .collect();
        Self {
            cohsums: cohsums.into_iter().map(CohSum::from).collect(),
            amplitudes,
            parameters,
        }
    }
    /// Computes the result of evaluating the terms in the model with the given [`Parameter`]s for
    /// the given [`Event`] by summing the result of [`CohSum::compute`] for each [`CohSum`]
    /// contained in the [`Model`].
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if any of the [`Amplitude::calculate`] steps fail.
    pub fn compute(&self, parameters: &[f64], event: &Event) -> Result<f64, RustitudeError> {
        let pars: Vec<f64> = self
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        // First, we calculate the values for the active amplitudes
        let cache: Vec<Option<Complex64>> = self
            .amplitudes
            .iter()
            .map(|amp| {
                if amp.active {
                    amp.calculate(&pars, event).map(Some)
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<Option<Complex64>>, RustitudeError>>()?;
        Ok(self
            .cohsums
            .iter()
            .map(|cohsum| cohsum.compute(&cache))
            .sum::<Option<f64>>()
            .unwrap_or_default())
    }
    /// Computes the result of evaluating the terms in the model with the given [`Parameter`]s for
    /// the given [`Event`] by summing the result of [`CohSum::norm_int`] for each [`CohSum`]
    /// contained in the [`Model`].
    ///
    /// # Errors
    ///
    /// This method yields a [`RustitudeError`] if any of the [`Amplitude::calculate`] steps fail.
    pub fn norm_int(&self, parameters: &[f64], event: &Event) -> Result<f64, RustitudeError> {
        let pars: Vec<f64> = self
            .parameters
            .iter()
            .map(|p| p.index.map_or_else(|| p.initial, |i| parameters[i]))
            .collect();
        // First, we calculate the values for the active amplitudes
        let cache: Vec<Option<Complex64>> = self
            .amplitudes
            .iter()
            .map(|amp| {
                if amp.active {
                    amp.calculate(&pars, event).map(Some)
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<Option<Complex64>>, RustitudeError>>()?;
        Ok(self
            .cohsums
            .iter()
            .map(|cohsum| cohsum.norm_int(&cache))
            .sum::<Option<f64>>()
            .unwrap_or_default())
    }
    /// Registers the [`Model`] with the [`Dataset`] by [`Amplitude::register`]ing each
    /// [`Amplitude`] and setting the proper cache position and parameter starting index.
    ///
    /// # Errors
    ///
    /// This method will yield a [`RustitudeError`] if any [`Amplitude::precalculate`] steps fail.
    pub fn load(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        let mut next_cache_pos = 0;
        let mut parameter_index = 0;
        self.amplitudes.iter_mut().try_for_each(|amp| {
            amp.register(next_cache_pos, parameter_index, dataset)?;
            self.cohsums.iter_mut().for_each(|cohsum| {
                cohsum.walk_mut().iter_mut().for_each(|r_amp| {
                    if r_amp.name == amp.name {
                        r_amp.cache_position = next_cache_pos;
                        r_amp.parameter_index_start = parameter_index;
                    }
                })
            });
            next_cache_pos += 1;
            parameter_index += amp.parameters().len();
            Ok(())
        })
    }
    fn group_by_index(&self) -> Vec<Vec<&Parameter>> {
        self.parameters
            .iter()
            .sorted_by_key(|par| par.index)
            .chunk_by(|par| par.index)
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<_>>())
            .collect()
    }
    fn group_by_index_mut(&mut self) -> Vec<Vec<&mut Parameter>> {
        self.parameters
            .iter_mut()
            .sorted_by_key(|par| par.index)
            .chunk_by(|par| par.index)
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect()
    }
    fn any_fixed(&self) -> bool {
        self.parameters.iter().any(|p| p.index.is_none())
    }
    fn reindex_parameters(&mut self) {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        self.group_by_index_mut()
            .iter_mut()
            .skip(any_fixed) // first element could be index = None
            .enumerate()
            .for_each(|(ind, par_group)| par_group.iter_mut().for_each(|par| par.index = Some(ind)))
    }
    fn get_min_free_index(&self) -> Option<usize> {
        self.parameters
            .iter()
            .filter_map(|p| p.index)
            .max()
            .map_or(Some(0), |max| Some(max + 1))
    }
    fn get_min_fixed_index(&self) -> Option<usize> {
        self.parameters
            .iter()
            .filter_map(|p| p.fixed_index)
            .max()
            .map_or(Some(0), |max| Some(max + 1))
    }
}

/// A [`Node`] for computing a single scalar value from an input parameter.
///
/// This struct implements [`Node`] to generate a single new parameter called `value`.
///
/// # Parameters:
///
/// - `value`: The value of the scalar.
pub struct Scalar;
impl Node for Scalar {
    fn parameters(&self) -> Vec<String> {
        vec!["value".to_string()]
    }
    fn calculate(&self, parameters: &[f64], _event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(Complex64::new(parameters[0], 0.0))
    }
}

pub fn scalar(name: &str) -> AmpOp {
    //! Creates a named [`Scalar`].
    //!
    //! This is a convenience method to generate an [`AmpOp`] which is just a single free
    //! parameter called `value`.
    //!
    //! # Examples
    //!
    //! Basic usage:
    //!
    //! ```
    //! use rustitude_core::prelude::*;
    //! let my_scalar = scalar("MyScalar");
    //! if let AmpOp::Amplitude(amp) = my_scalar {
    //!     assert_eq!(amp.node.read().parameters(), vec!["value".to_string()]);
    //! }
    //! ```
    Amplitude::new(name, Scalar).into()
}
/// A [`Node`] for computing a single complex value from two input parameters.
///
/// This struct implements [`Node`] to generate a complex value from two input parameters called
/// `real` and `imag`.
///
/// # Parameters:
///
/// - `real`: The real part of the complex scalar.
/// - `imag`: The imaginary part of the complex scalar.
pub struct ComplexScalar;
impl Node for ComplexScalar {
    fn calculate(&self, parameters: &[f64], _event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(Complex64::new(parameters[0], parameters[1]))
    }

    fn parameters(&self) -> Vec<String> {
        vec!["real".to_string(), "imag".to_string()]
    }
}

pub fn cscalar(name: &str) -> AmpOp {
    //! Creates a named [`ComplexScalar`].
    //!
    //! This is a convenience method to generate an [`AmpOp`] which represents a complex
    //! value determined by two parameters, `real` and `imag`.
    //!
    //! # Examples
    //!
    //! Basic usage:
    //!
    //! ```
    //! use rustitude_core::prelude::*;
    //! let my_cscalar = cscalar("MyComplexScalar");
    //! if let AmpOp::Amplitude(amp) = my_cscalar {
    //!     assert_eq!(amp.node.read().parameters(), vec!["real".to_string(), "imag".to_string()]);
    //! }
    //! ```
    Amplitude::new(name, ComplexScalar).into()
}

/// A [`Node`] for computing a single complex value from two input parameters in polar form.
///
/// This struct implements [`Node`] to generate a complex value from two input parameters called
/// `mag` and `phi`.
///
/// # Parameters:
///
/// - `mag`: The magnitude of the complex scalar.
/// - `phi`: The phase of the complex scalar.
pub struct PolarComplexScalar;
impl Node for PolarComplexScalar {
    fn calculate(&self, parameters: &[f64], _event: &Event) -> Result<Complex64, RustitudeError> {
        Ok(parameters[0] * Complex64::cis(parameters[1]))
    }

    fn parameters(&self) -> Vec<String> {
        vec!["mag".to_string(), "phi".to_string()]
    }
}

pub fn pcscalar(name: &str) -> AmpOp {
    //! Creates a named [`PolarComplexScalar`].
    //!
    //! This is a convenience method to generate an [`AmpOp`] which represents a complex
    //! value determined by two parameters, `real` and `imag`.
    //!
    //! # Examples
    //!
    //! Basic usage:
    //!
    //! ```
    //! use rustitude_core::prelude::*;
    //! let my_pcscalar = pcscalar("MyPolarComplexScalar");
    //! if let AmpOp::Amplitude(amp) = my_pcscalar {
    //!     assert_eq!(amp.node.read().parameters(), vec!["mag".to_string(), "phi".to_string()]);
    //! }
    //! ```
    Amplitude::new(name, PolarComplexScalar).into()
}

/// A generic struct which can be used to create any kind of piecewise function.
pub struct Piecewise<F>
where
    F: Fn(&Event) -> f64 + Send + Sync + Copy,
{
    edges: Vec<(f64, f64)>,
    variable: F,
    calculated_variable: Vec<f64>,
}

impl<F> Piecewise<F>
where
    F: Fn(&Event) -> f64 + Send + Sync + Copy,
{
    /// Create a new [`Piecewise`] struct from a number of bins, a range of values, and a callable
    /// which defines a variable over the [`Event`]s in a [`Dataset`].
    pub fn new(bins: usize, range: (f64, f64), variable: F) -> Self {
        let diff = (range.1 - range.0) / (bins as f64);
        let edges = (0..bins)
            .map(|i| {
                (
                    (i as f64).mul_add(diff, range.0),
                    ((i + 1) as f64).mul_add(diff, range.0),
                )
            })
            .collect();
        Self {
            edges,
            variable,
            calculated_variable: Vec::default(),
        }
    }
}

impl<F> Node for Piecewise<F>
where
    F: Fn(&Event) -> f64 + Send + Sync + Copy,
{
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        self.calculated_variable = dataset
            .events
            .read()
            .par_iter()
            .map(self.variable)
            .collect();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let val = self.calculated_variable[event.index];
        let opt_i_bin = self.edges.iter().position(|&(l, r)| val >= l && val <= r);
        opt_i_bin.map_or_else(
            || Ok(Complex64::default()),
            |i_bin| {
                Ok(Complex64::new(
                    parameters[i_bin * 2],
                    parameters[(i_bin * 2) + 1],
                ))
            },
        )
    }

    fn parameters(&self) -> Vec<String> {
        (0..self.edges.len())
            .flat_map(|i| vec![format!("bin {} re", i), format!("bin {} im", i)])
            .collect()
    }
}

pub fn piecewise_m(name: &str, bins: usize, range: (f64, f64)) -> AmpOp {
    //! Creates a named [`Piecewise`] amplitude with the resonance mass as the binning variable.
    Amplitude::new(
        name,
        Piecewise::new(bins, range, |e: &Event| {
            (e.daughter_p4s[0] + e.daughter_p4s[1]).m()
        }),
    )
    .into()
}
