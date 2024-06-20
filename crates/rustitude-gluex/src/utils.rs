use std::{fmt::Display, str::FromStr};

use factorial::Factorial;
use rustitude_core::prelude::*;
use sphrs::Coordinates;

pub fn breakup_momentum(m0: f64, m1: f64, m2: f64) -> f64 {
    f64::sqrt(f64::abs(
        m0.powi(4) + m1.powi(4) + m2.powi(4)
            - 2.0 * (m0.powi(2) * m1.powi(2) + m0.powi(2) * m2.powi(2) + m1.powi(2) * m2.powi(2)),
    )) / (2.0 * m0)
}

pub fn blatt_weisskopf(m0: f64, m1: f64, m2: f64, l: usize) -> f64 {
    let q = breakup_momentum(m0, m1, m2);
    let z = q.powi(2) / f64::powi(0.1973, 2);
    match l {
        0 => 1.0,
        1 => f64::sqrt((2.0 * z) / (z + 1.0)),
        2 => f64::sqrt((13.0 * z.powi(2)) / ((z - 3.0).powi(2) + 9.0 * z)),
        3 => f64::sqrt(
            (277.0 * z.powi(3)) / (z * (z - 15.0).powi(2) + 9.0 * (2.0 * z - 5.0).powi(2)),
        ),
        4 => f64::sqrt(
            (12746.0 * z.powi(4)) / (z.powi(2) - 45.0 * z + 105.0).powi(2)
                + 25.0 * z * (2.0 * z - 21.0).powi(2),
        ),
        l => panic!("L = {l} is not yet implemented"),
    }
}

pub fn small_wigner_d_matrix(beta: f64, j: usize, m: isize, n: isize) -> f64 {
    let jpm = (j as i32 + m as i32) as u32;
    let jmm = (j as i32 - m as i32) as u32;
    let jpn = (j as i32 + n as i32) as u32;
    let jmn = (j as i32 - n as i32) as u32;
    let prefactor =
        f64::sqrt((jpm.factorial() * jmm.factorial() * jpn.factorial() * jmn.factorial()) as f64);
    let s_min = isize::max(0, n - m) as usize;
    let s_max = isize::min(jpn as isize, jmm as isize) as usize;
    let sum: f64 = (s_min..=s_max)
        .map(|s| {
            ((-1.0f64).powi(m as i32 - n as i32 + s as i32)
                * (f64::cos(beta / 2.0)
                    .powi(2 * (j as i32) + n as i32 - m as i32 - 2 * (s as i32)))
                * (f64::sin(beta / 2.0).powi(m as i32 - n as i32 + 2 * s as i32)))
                / ((jpm - s as u32).factorial()
                    * (s as u32).factorial()
                    * ((m - n + s as isize) as u32).factorial()
                    * (jmm - s as u32).factorial()) as f64
        })
        .sum();
    prefactor * sum
}

pub fn wigner_d_matrix(
    alpha: f64,
    beta: f64,
    gamma: f64,
    j: usize,
    m: isize,
    n: isize,
) -> Complex64 {
    Complex64::cis(-(m as f64) * alpha)
        * small_wigner_d_matrix(beta, j, m, n)
        * Complex64::cis(-(n as f64) * gamma)
}

#[derive(Clone, Copy, Default)]
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

#[derive(Clone)]
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

impl Frame {
    pub fn coordinates(
        &self,
        beam_res_vec: &Vector3<f64>,
        recoil_res_vec: &Vector3<f64>,
        daughter_res_vec: &Vector3<f64>,
        event: &Event,
    ) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>, Coordinates<f64>) {
        match self {
            Frame::Helicity => {
                let z = -recoil_res_vec.normalize();
                let y = event
                    .beam_p4
                    .momentum()
                    .cross(&(-recoil_res_vec))
                    .normalize();
                let x = y.cross(&z);
                (
                    x,
                    y,
                    z,
                    Coordinates::cartesian(
                        daughter_res_vec.dot(&x),
                        daughter_res_vec.dot(&y),
                        daughter_res_vec.dot(&z),
                    ),
                )
            }
            Frame::GottfriedJackson => {
                let z = beam_res_vec.normalize();
                let y = event
                    .beam_p4
                    .momentum()
                    .cross(&(-recoil_res_vec))
                    .normalize();
                let x = y.cross(&z);
                (
                    x,
                    y,
                    z,
                    Coordinates::cartesian(
                        daughter_res_vec.dot(&x),
                        daughter_res_vec.dot(&y),
                        daughter_res_vec.dot(&z),
                    ),
                )
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Reflectivity {
    Positive = 1,
    Negative = -1,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseReflectivityError;

impl FromStr for Reflectivity {
    type Err = ParseReflectivityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "positive" => Ok(Reflectivity::Positive),
            "pos" => Ok(Reflectivity::Positive),
            "p" => Ok(Reflectivity::Positive),
            "+" => Ok(Reflectivity::Positive),
            "plus" => Ok(Reflectivity::Positive),
            "negative" => Ok(Reflectivity::Negative),
            "neg" => Ok(Reflectivity::Negative),
            "n" => Ok(Reflectivity::Negative),
            "-" => Ok(Reflectivity::Negative),
            "minus" => Ok(Reflectivity::Negative),
            "m" => Ok(Reflectivity::Negative),
            _ => Err(ParseReflectivityError),
        }
    }
}

impl Display for Reflectivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reflectivity::Positive => write!(f, "+"),
            Reflectivity::Negative => write!(f, "-"),
        }
    }
}
