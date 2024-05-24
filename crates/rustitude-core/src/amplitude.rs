use itertools::Itertools;
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

#[derive(Clone)]
pub struct Parameter {
    pub amplitude: String,
    pub name: String,
    pub index: Option<usize>,
    pub fixed_index: Option<usize>,
    pub initial: f64,
    pub bounds: (f64, f64),
}
impl Parameter {
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

#[derive(Clone)]
pub enum AmpOp {
    Amplitude(Amplitude),
    Sum(Vec<AmpOp>),
    Product(Vec<AmpOp>),
    Real(Box<AmpOp>),
    Imag(Box<AmpOp>),
    NormSqr(Box<AmpOp>),
}

impl Debug for AmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amplitude(amp) => writeln!(f, "{:?}", amp),
            Self::Sum(ops) => {
                write!(f, "Sum [ ")?;
                for op in ops {
                    write!(f, "{:?} ", op)?;
                }
                write!(f, "]")
            }
            Self::Product(ops) => {
                write!(f, "Prod [ ")?;
                for op in ops {
                    write!(f, "{:?} ", op)?;
                }
                write!(f, "]")
            }
            Self::Real(op) => write!(f, "Re[{:?}]", op),
            Self::Imag(op) => write!(f, "Im[{:?}]", op),
            Self::NormSqr(op) => write!(f, "|[{:?}]|^2", op),
        }
    }
}

impl Display for AmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amplitude(amp) => writeln!(f, "{}", amp),
            Self::Sum(ops) => {
                write!(f, "Sum [ ")?;
                for op in ops {
                    write!(f, "{} ", op)?;
                }
                write!(f, "]")
            }
            Self::Product(ops) => {
                write!(f, "Prod [ ")?;
                for op in ops {
                    write!(f, "{} ", op)?;
                }
                write!(f, "]")
            }
            Self::Real(op) => write!(f, "Re[{:?}]", op),
            Self::Imag(op) => write!(f, "Im[{:?}]", op),
            Self::NormSqr(op) => write!(f, "|[{:?}]|^2", op),
        }
    }
}
impl AmpOp {
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
            Self::Sum(ops) => {
                println!("[ + ]");
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
            Self::NormSqr(op) => {
                println!("[ norm sqr ]");
                Self::_print_indent(&bits);
                Self::_print_end();
                bits.push(false);
                op._print_tree(bits.clone());
                bits.pop();
            }
        }
    }
    pub fn walk(&self) -> Vec<Amplitude> {
        match self {
            Self::Amplitude(amp) => vec![amp.clone()],
            Self::Sum(ops) => ops.iter().flat_map(|op| op.walk()).collect(),
            Self::Product(ops) => ops.iter().flat_map(|op| op.walk()).collect(),
            Self::Real(op) => op.walk(),
            Self::Imag(op) => op.walk(),
            Self::NormSqr(op) => op.walk(),
        }
    }

    pub fn walk_mut(&mut self) -> Vec<&mut Amplitude> {
        match self {
            Self::Amplitude(amp) => vec![amp],
            Self::Sum(ops) => ops.iter_mut().flat_map(|op| op.walk_mut()).collect(),
            Self::Product(ops) => ops.iter_mut().flat_map(|op| op.walk_mut()).collect(),
            Self::Real(op) => op.walk_mut(),
            Self::Imag(op) => op.walk_mut(),
            Self::NormSqr(op) => op.walk_mut(),
        }
    }

    pub fn compute(&self, cache: &[Option<Complex64>]) -> Option<Complex64> {
        match self {
            Self::Amplitude(amp) => cache[amp.cache_position],
            Self::Sum(ops) => Some(ops.iter().filter_map(|op| op.compute(cache)).sum()),
            Self::Product(ops) => Some(ops.iter().filter_map(|op| op.compute(cache)).product()),
            Self::Real(op) => op.compute(cache).map(|r| r.re.into()),
            Self::Imag(op) => op.compute(cache).map(|r| r.im.into()),
            Self::NormSqr(op) => op.compute(cache).map(|r| r.norm_sqr().into()),
        }
    }

    pub fn real(&self) -> Self {
        Self::Real(Box::new(self.clone()))
    }
    pub fn imag(&self) -> Self {
        Self::Imag(Box::new(self.clone()))
    }
    pub fn norm_sqr(&self) -> Self {
        Self::NormSqr(Box::new(self.clone()))
    }
}
impl Add for AmpOp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Self::Sum(ops_l), Self::Sum(ops_r)) => Self::Sum([ops_l, ops_r].concat()),
            (Self::Sum(ops), _) => {
                let mut sum_ops = ops;
                sum_ops.push(rhs);
                Self::Sum(sum_ops)
            }
            (_, Self::Sum(ops)) => {
                let mut sum_ops = ops;
                sum_ops.push(self);
                Self::Sum(sum_ops)
            }
            (_, _) => Self::Sum(vec![self, rhs]),
        }
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
    pub active: bool,
    pub cache_position: usize,
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
    pub fn new(name: &str, node: impl Node + 'static) -> Self {
        Self {
            name: name.to_string(),
            node: Arc::new(RwLock::new(Box::new(node))),
            active: true,
            cache_position: 0,
            parameter_index_start: 0,
        }
    }
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

#[derive(Debug, Clone)]
pub struct Model {
    pub root: AmpOp,
    pub amplitudes: Vec<Amplitude>,
    pub parameters: Vec<Parameter>,
}

impl Model {
    pub fn get_amplitude(&self, amplitude_name: &str) -> Result<Amplitude, RustitudeError> {
        self.amplitudes
            .iter()
            .find(|a: &&Amplitude| a.name == amplitude_name)
            .ok_or_else(|| RustitudeError::AmplitudeNotFoundError(amplitude_name.to_string()))
            .cloned()
    }
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
    pub fn get_bounds(&self) -> Vec<(f64, f64)> {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        self.group_by_index()
            .iter()
            .skip(any_fixed)
            .filter_map(|group| group.first().map(|par| par.bounds))
            .collect()
    }
    pub fn get_initial(&self) -> Vec<f64> {
        let any_fixed = if self.any_fixed() { 1 } else { 0 };
        self.group_by_index()
            .iter()
            .skip(any_fixed)
            .filter_map(|group| group.first().map(|par| par.initial))
            .collect()
    }
    pub fn get_n_free(&self) -> usize {
        self.get_min_free_index().unwrap_or(0)
    }
    pub fn activate(&mut self, amplitude: &str) {
        self.amplitudes.iter_mut().for_each(|amp| {
            if amp.name == amplitude {
                amp.active = true
            }
        })
    }
    pub fn deactivate(&mut self, amplitude: &str) {
        self.amplitudes.iter_mut().for_each(|amp| {
            if amp.name == amplitude {
                amp.active = false
            }
        })
    }
    pub fn new(root: AmpOp) -> Self {
        let mut amp_names = HashSet::new();
        let amplitudes: Vec<Amplitude> = root
            .walk()
            .into_iter()
            .filter(|amp| amp_names.insert(amp.name.clone()))
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
            root,
            amplitudes,
            parameters,
        }
    }
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
        Ok(self.root.compute(&cache).unwrap_or_default().re)
    }
    pub fn load(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        let mut next_cache_pos = 0;
        let mut parameter_index = 0;
        self.amplitudes.iter_mut().try_for_each(|amp| {
            amp.register(next_cache_pos, parameter_index, dataset)?;
            self.root.walk_mut().iter_mut().for_each(|r_amp| {
                if r_amp.name == amp.name {
                    r_amp.cache_position = next_cache_pos;
                    r_amp.parameter_index_start = parameter_index;
                }
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
