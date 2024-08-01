use std::{fmt::Display, str::FromStr};

use factorial::Factorial;
use pyo3::prelude::*;
use rustitude_core::prelude::*;
use sphrs::Coordinates;

pub fn breakup_momentum<F: Field>(m0: F, m1: F, m2: F) -> F {
    F::fsqrt(F::fabs(
        m0.fpowi(4) + m1.fpowi(4) + m2.fpowi(4)
            - F::TWO
                * (m0.fpowi(2) * m1.fpowi(2)
                    + m0.fpowi(2) * m2.fpowi(2)
                    + m1.fpowi(2) * m2.fpowi(2)),
    )) / (F::TWO * m0)
}

/// Computes the ([`Complex<F>`]) breakup momentum of a particle with mass `m0` decaying into two particles
/// with masses `m1` and `m2`.
pub fn breakup_momentum_c<F: Field>(m0: F, m1: F, m2: F) -> Complex<F> {
    rho(m0.fpowi(2), m1, m2) * m0 / F::TWO
}

pub fn chi_plus<F: Field>(s: F, m1: F, m2: F) -> Complex<F> {
    (F::ONE - ((m1 + m2) * (m1 + m2)) / s).c()
}

pub fn chi_minus<F: Field>(s: F, m1: F, m2: F) -> Complex<F> {
    (F::ONE - ((m1 - m2) * (m1 - m2)) / s).c()
}

pub fn rho<F: Field>(s: F, m1: F, m2: F) -> Complex<F> {
    Complex::sqrt(chi_plus(s, m1, m2) * chi_minus(s, m1, m2))
}

pub fn blatt_weisskopf<F: Field>(m0: F, m1: F, m2: F, l: usize) -> F {
    let q = breakup_momentum(m0, m1, m2);
    let z = q.fpowi(2) / F::f(0.1973).fpowi(2);
    match l {
        0 => F::ONE,
        1 => F::fsqrt((F::TWO * z) / (z + F::ONE)),
        2 => F::fsqrt((F::f(13.0) * z.fpowi(2)) / ((z - F::THREE).fpowi(2) + F::NINE * z)),
        3 => F::fsqrt(
            (F::f(277.0) * z.fpowi(3))
                / (z * (z - F::f(15.0)).fpowi(2) + F::NINE * (F::TWO * z - F::FIVE).fpowi(2)),
        ),
        4 => F::fsqrt(
            (F::f(12746.0) * z.fpowi(4)) / (z.fpowi(2) - F::f(45.0) * z + F::f(105.0)).fpowi(2)
                + F::f(25.0) * z * (F::TWO * z - F::f(21.0)).fpowi(2),
        ),
        l => panic!("L = {l} is not yet implemented"),
    }
}

/// Computes the ([`Complex<F>`]) Blatt-Weisskopf barrier factor representing the energy required for a particle
/// with mass `m0` to decay into two particles with masses `m1` and `m2` and angular momentum `l`.
///
/// In applications where `m0` is expected to be above the mass threshold to produce `m1` and
/// `m2`, the absolute value of this function can be safely assumed to be equal to its value.
pub fn blatt_weisskopf_c<F: Field>(m0: F, m1: F, m2: F, l: usize) -> Complex<F> {
    let q = breakup_momentum_c(m0, m1, m2);
    let z = q.powi(2) / F::f(0.1973).fpowi(2);
    match l {
        0 => F::ONE.c(),
        1 => Complex::sqrt((F::TWO.c() * z) / (z + F::ONE)),
        2 => Complex::sqrt((z.powi(2) * F::f(13.0)) / ((z - F::THREE).powi(2) + z * F::NINE)),
        3 => Complex::sqrt(
            (z.powi(3) * F::f(277.0))
                / (z * (z - F::f(15.0)).powi(2) + (z * F::TWO - F::FIVE).powi(2))
                * F::NINE,
        ),
        4 => Complex::sqrt(
            (z.powi(4) * F::f(12746.0)) / (z.powi(2) - z * F::f(45.0) + F::f(105.0)).powi(2)
                + z * F::f(25.0) * (z * F::TWO - F::f(21.0)).powi(2),
        ),
        l => panic!("L = {l} is not yet implemented"),
    }
}

pub fn small_wigner_d_matrix<F: Field>(beta: F, j: usize, m: isize, n: isize) -> F {
    let jpm = (j as i32 + m as i32) as u32;
    let jmm = (j as i32 - m as i32) as u32;
    let jpn = (j as i32 + n as i32) as u32;
    let jmn = (j as i32 - n as i32) as u32;
    let prefactor = F::fsqrt(F::convert_u32(
        jpm.factorial() * jmm.factorial() * jpn.factorial() * jmn.factorial(),
    ));
    let s_min = isize::max(0, n - m) as usize;
    let s_max = isize::min(jpn as isize, jmm as isize) as usize;
    let sum: F = (s_min..=s_max)
        .map(|s| {
            (F::fpowi(-F::ONE, m as i32 - n as i32 + s as i32)
                * (F::fcos(beta / F::TWO)
                    .fpowi(2 * (j as i32) + n as i32 - m as i32 - 2 * (s as i32)))
                * (F::fsin(beta / F::TWO).fpowi(m as i32 - n as i32 + 2 * s as i32)))
                / F::convert_u32(
                    (jpm - s as u32).factorial()
                        * (s as u32).factorial()
                        * ((m - n + s as isize) as u32).factorial()
                        * (jmm - s as u32).factorial(),
                )
        })
        .sum();
    prefactor * sum
}

pub fn wigner_d_matrix<F: Field>(
    alpha: F,
    beta: F,
    gamma: F,
    j: usize,
    m: isize,
    n: isize,
) -> Complex<F> {
    Complex::cis(-(F::convert_isize(m)) * alpha)
        * small_wigner_d_matrix(beta, j, m, n)
        * Complex::cis(-(F::convert_isize(n)) * gamma)
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, Default, PartialEq)]
#[rustfmt::skip]
pub enum Wave {
    #[default]
    S,
    S0,
    Pn1, P0, P1, P,
    Dn2, Dn1, D0, D1, D2, D,
    Fn3, Fn2, Fn1, F0, F1, F2, F3, F,
}

#[rustfmt::skip]
impl Wave {
    pub fn new(l: usize, m: isize) -> Self {
        match l {
            0 => Self::S0,
            1 => match m {
                -1 => Self::Pn1,
                0 => Self::P0,
                1 => Self::P1,
                _ => panic!("|m = {m}| > (l = {l})")
            }
            2 => match m {
                -2 => Self::Dn2,
                -1 => Self::Dn1,
                0 => Self::D0,
                1 => Self::D1,
                2 => Self::D2,
                _ => panic!("|m = {m}| > (l = {l})")
            }
            3 => match m {
                -3 => Self::Fn3,
                -2 => Self::Fn2,
                -1 => Self::Fn1,
                0 => Self::F0,
                1 => Self::F1,
                2 => Self::F2,
                3 => Self::F3,
                _ => panic!("|m = {m}| > (l = {l})")
            }
            _ => panic!("(l = {l}) > 3 is not yet implemented!")
        }
    }
    pub fn l(&self) -> i64 {
        match self {
            Self::S0 | Self::S => 0,
            Self::Pn1 | Self::P0 | Self::P1 | Self::P => 1,
            Self::Dn2 | Self::Dn1 | Self::D0 | Self::D1 | Self::D2 | Self::D => 2,
            Self::Fn3 | Self::Fn2 | Self::Fn1 | Self::F0 | Self::F1 | Self::F2 | Self::F3 | Self::F => 3,
        }
    }
    pub fn m(&self) -> i64 {
        match self {
            Self::S | Self::P | Self::D | Self::F => 0,
            Self::S0 | Self::P0 | Self::D0 | Self::F0 => 0,
            Self::Pn1 | Self::Dn1 | Self::Fn1 => -1,
            Self::P1 | Self::D1 | Self::F1 => 1,
            Self::Dn2 | Self::Fn2 => -2,
            Self::D2 | Self::F2 => 2,
            Self::Fn3 => -3,
            Self::F3 => 3,
        }
    }
}

impl Display for Wave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l_string = match self.l() {
            0 => "S",
            1 => "P",
            2 => "D",
            3 => "F",
            _ => unimplemented!(),
        };
        write!(f, "{} {:+}", l_string, self.m())
    }
}

#[pyclass(eq, eq_int)]
#[derive(Copy, Clone, PartialEq)]
pub enum Frame {
    Helicity,
    GottfriedJackson,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFrameError;

impl FromStr for Frame {
    type Err = ParseFrameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "helicity" => Ok(Frame::Helicity),
            "hx" => Ok(Frame::Helicity),
            "gottfried-jackson" => Ok(Frame::GottfriedJackson),
            "gj" => Ok(Frame::GottfriedJackson),
            _ => Err(ParseFrameError),
        }
    }
}

pub fn coordinates<F: Field>(
    x: &Vector3<F>,
    y: &Vector3<F>,
    z: &Vector3<F>,
    p: &Vector3<F>,
) -> Coordinates<F> {
    Coordinates::cartesian(p.dot(x), p.dot(y), p.dot(z))
}

impl Frame {
    pub fn coordinates<F: Field>(
        &self,
        decay: Decay,
        other_p4: &FourMomentum<F>,
        event: &Event<F>,
    ) -> (Vector3<F>, Vector3<F>, Vector3<F>, Coordinates<F>) {
        let resonance_p4 = decay.resonance_p4(event);
        let beam_res_vec = event.beam_p4.boost_along(&resonance_p4).momentum();
        let recoil_res_vec = event.recoil_p4.boost_along(&resonance_p4).momentum();
        let other_res_vec = other_p4.boost_along(&resonance_p4).momentum();
        let (x, y, z) = match self {
            Frame::Helicity => {
                let z = -recoil_res_vec.normalize();
                let y = beam_res_vec.cross(&z).normalize();
                let x = y.cross(&z);
                (x, y, z)
            }
            Frame::GottfriedJackson => {
                let z = beam_res_vec.normalize();
                let y = event
                    .beam_p4
                    .momentum()
                    .cross(&(-recoil_res_vec))
                    .normalize();
                let x = y.cross(&z);
                (x, y, z)
            }
        };
        (x, y, z, coordinates(&x, &y, &z, &other_res_vec))
    }
    pub fn coordinates_from_boosted_vec<F: Field>(
        &self,
        decay: Decay,
        other_res_vec: &Vector3<F>,
        event: &Event<F>,
    ) -> (Vector3<F>, Vector3<F>, Vector3<F>, Coordinates<F>) {
        let resonance_p4 = decay.resonance_p4(event);
        let beam_res_vec = event.beam_p4.boost_along(&resonance_p4).momentum();
        let recoil_res_vec = event.recoil_p4.boost_along(&resonance_p4).momentum();
        let (x, y, z) = match self {
            Frame::Helicity => {
                let z = -recoil_res_vec.normalize();
                let y = beam_res_vec.cross(&z).normalize();
                let x = y.cross(&z);
                (x, y, z)
            }
            Frame::GottfriedJackson => {
                let z = beam_res_vec.normalize();
                let y = event
                    .beam_p4
                    .momentum()
                    .cross(&(-recoil_res_vec))
                    .normalize();
                let x = y.cross(&z);
                (x, y, z)
            }
        };
        (x, y, z, coordinates(&x, &y, &z, other_res_vec))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Copy, Clone, PartialEq)]
pub enum Sign {
    Positive = 1,
    Negative = -1,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSignError;

impl FromStr for Sign {
    type Err = ParseSignError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "positive" => Ok(Sign::Positive),
            "pos" => Ok(Sign::Positive),
            "p" => Ok(Sign::Positive),
            "+" => Ok(Sign::Positive),
            "plus" => Ok(Sign::Positive),
            "negative" => Ok(Sign::Negative),
            "neg" => Ok(Sign::Negative),
            "n" => Ok(Sign::Negative),
            "-" => Ok(Sign::Negative),
            "minus" => Ok(Sign::Negative),
            "m" => Ok(Sign::Negative),
            _ => Err(ParseSignError),
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Positive => write!(f, "+"),
            Sign::Negative => write!(f, "-"),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub enum Decay {
    TwoBodyDecay([usize; 2]),
    ThreeBodyDecay([usize; 3]),
}
impl Default for Decay {
    fn default() -> Self {
        Self::TwoBodyDecay([0, 1])
    }
}

impl Decay {
    pub fn resonance_p4<F: Field>(&self, event: &Event<F>) -> FourMomentum<F> {
        match self {
            Decay::TwoBodyDecay(inds) => inds.iter().map(|&i| event.daughter_p4s[i]).sum(),
            Decay::ThreeBodyDecay(inds) => inds.iter().map(|&i| event.daughter_p4s[i]).sum(),
        }
    }
    pub fn daughter_p4<'a, F: Field>(
        &self,
        index: usize,
        event: &'a Event<F>,
    ) -> &'a FourMomentum<F> {
        match self {
            Decay::TwoBodyDecay(inds) => &event.daughter_p4s[inds[index]],
            Decay::ThreeBodyDecay(inds) => &event.daughter_p4s[inds[index]],
        }
    }
    pub fn primary_p4<'a, F: Field>(&self, event: &'a Event<F>) -> &'a FourMomentum<F> {
        match self {
            Decay::TwoBodyDecay(inds) => &event.daughter_p4s[inds[0]],
            Decay::ThreeBodyDecay(inds) => &event.daughter_p4s[inds[0]],
        }
    }
    pub fn secondary_p4<'a, F: Field>(&self, event: &'a Event<F>) -> &'a FourMomentum<F> {
        match self {
            Decay::TwoBodyDecay(inds) => &event.daughter_p4s[inds[1]],
            Decay::ThreeBodyDecay(inds) => &event.daughter_p4s[inds[1]],
        }
    }
    pub fn tertiary_p4<'a, F: Field>(&self, event: &'a Event<F>) -> &'a FourMomentum<F> {
        match self {
            Decay::TwoBodyDecay(_) => panic!(),
            Decay::ThreeBodyDecay(inds) => &event.daughter_p4s[inds[2]],
        }
    }
    pub fn coordinates<F: Field>(
        &self,
        frame: Frame,
        index: usize,
        event: &Event<F>,
    ) -> (Vector3<F>, Vector3<F>, Vector3<F>, Coordinates<F>) {
        frame.coordinates(*self, self.daughter_p4(index, event), event)
    }
}
