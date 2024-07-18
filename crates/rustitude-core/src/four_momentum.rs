//! # Four-Momentum
//!
//! This module contains a helper struct, [`FourMomentum`], which contains useful methods and
//! manipulations for physics four-vectors representing momentum coordinates. In particular,
//! this struct has the same layout as a `[Field; 4]` with components identified as
//! $`(E, p_x, p_y, p_z)`$.
use crate::Field;
use nalgebra::{Matrix4, Vector3, Vector4};
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

/// Struct which holds energy and three-momentum as a four-vector.
///
/// A four-momentum structure with helpful methods for boosts.
///
/// This is the basic structure of a Lorentz four-vector
/// of the form $`(E, \vec{p})`$ where $`E`$ is the energy and $`\vec{p}`$ is the
/// momentum.
///
/// # Examples
/// ```
/// use rustitude_core::prelude::*;
///
/// let vec_a = FourMomentum::new(1.3, 0.2, 0.3, 0.1);
/// let vec_b = FourMomentum::new(4.2, 0.5, 0.4, 0.5);
/// ```
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct FourMomentum<F: Field>([F; 4]);

impl<F: Field> Eq for FourMomentum<F> {}

impl<F: Field> Display for FourMomentum<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, ({}, {}, {})]",
            self.e(),
            self.px(),
            self.py(),
            self.pz(),
        )
    }
}

impl<F: Field> FourMomentum<F> {
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(e: F, px: F, py: F, pz: F) -> Self {
        //! Create a new [`FourMomentum`] from energy and momentum components.
        //!
        //! Components are listed in the order $` (E, p_x, p_y, p_z) `$
        Self([e, px, py, pz])
    }

    /// Returns the energy of the given [`FourMomentum`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn e(&self) -> F {
        self.0[0]
    }
    /// Returns the momentum along the $`x`$-axis of the given [`FourMomentum`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn px(&self) -> F {
        self.0[1]
    }
    /// Returns the momentum along the $`y`$-axis of the given [`FourMomentum`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn py(&self) -> F {
        self.0[2]
    }
    /// Returns the momentum along the $`z`$-axis of the given [`FourMomentum`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn pz(&self) -> F {
        self.0[3]
    }

    /// Sets the energy of the given [`FourMomentum`].
    pub fn set_e(&mut self, value: F) {
        self.0[0] = value;
    }
    /// Sets the momentum along the $`x`$-axis of the given [`FourMomentum`].
    pub fn set_px(&mut self, value: F) {
        self.0[1] = value;
    }
    /// Sets the momentum along the $`x`$-axis of the given [`FourMomentum`].
    pub fn set_py(&mut self, value: F) {
        self.0[2] = value;
    }
    /// Sets the momentum along the $`x`$-axis of the given [`FourMomentum`].
    pub fn set_pz(&mut self, value: F) {
        self.0[3] = value;
    }

    /// Calculate the invariant $`m^2`$ for this [`FourMomentum`] instance.
    ///
    /// Calculates $` m^2 = E^2 - \vec{p}^2 `$
    ///
    /// # Examples
    /// ```
    /// use rustitude_core::prelude::*;
    ///
    /// let vec_a = FourMomentum::new(20.0, 1.0, 0.2, -0.1);
    /// //assert_eq!(vec_a.m2(), 20.0 * 20.0 - (1.0 * 1.0 + 0.0 * 0.2 + (-0.1) * (-0.1)));
    ///
    /// ```
    #[allow(clippy::suboptimal_flops)]
    pub fn m2(&self) -> F {
        F::fpowi(self.e(), 2)
            - F::fpowi(self.px(), 2)
            - F::fpowi(self.py(), 2)
            - F::fpowi(self.pz(), 2)
    }

    /// Calculate the invariant $`m`$ for this [`FourMomentum`] instance.
    ///
    /// Calculates $` m = \sqrt{E^2 - \vec{p}^2} `$
    ///
    /// # See Also:
    ///
    /// [`FourMomentum::m2`]
    pub fn m(&self) -> F {
        F::fsqrt(self.m2())
    }

    /// Boosts an instance of [`FourMomentum`] along the $`\vec{\beta}`$
    /// vector of another [`FourMomentum`].
    ///
    /// Calculates $`\mathbf{\Lambda} \cdot \mathbf{x}`$
    ///
    /// # Examples
    /// ```
    /// use rustitude_core::prelude::*;
    ///
    /// let vec_a = FourMomentum::new(20.0, 1.0, -3.2, 4.0);
    /// let vec_a_COM = vec_a.boost_along(&vec_a);
    /// assert!(f64::abs(vec_a_COM.px()) < 1e-7);
    /// assert!(f64::abs(vec_a_COM.py()) < 1e-7);
    /// assert!(f64::abs(vec_a_COM.pz()) < 1e-7);
    /// ```
    pub fn boost_along(&self, other: &Self) -> Self {
        let m_boost = other.boost_matrix();
        (m_boost * Vector4::<F>::from(self)).into()
    }
    /// Extract the 3-momentum as a [`nalgebra::Vector3<Field>`]
    ///
    /// # Examples
    /// ```
    /// use rustitude_core::prelude::*;
    /// use nalgebra::Vector3;
    ///
    /// let vec_a = FourMomentum::new(20.0, 1.0, 0.2, -0.1);
    /// assert_eq!(vec_a.momentum(), Vector3::new(1.0, 0.2, -0.1));
    /// ```
    pub fn momentum(&self) -> Vector3<F> {
        Vector3::new(self.px(), self.py(), self.pz())
    }

    /// Construct the 3-vector $`\vec{\beta}`$ where
    ///
    /// $` \vec{\beta} = \frac{\vec{p}}{E} `$
    pub fn beta3(&self) -> Vector3<F> {
        self.momentum() / self.e()
    }

    /// Construct the Lorentz boost matrix $`\mathbf{\Lambda}`$ where
    ///
    /// ```math
    /// \mathbf{\Lambda} = \begin{pmatrix}
    /// \gamma & -\gamma \beta_x & -\gamma \beta_y & -\gamma \beta_z \\
    /// -\gamma \beta_x & 1 + (\gamma - 1) \frac{\beta_x^2}{\vec{\beta}^2} & (\gamma - 1) \frac{\beta_x \beta_y}{\vec{\beta}^2} & (\gamma - 1) \frac{\beta_x \beta_z}{\vec{\beta}^2} \\
    /// -\gamma \beta_y & (\gamma - 1) \frac{\beta_y \beta_x}{\vec{\beta}^2} & 1 + (\gamma - 1) \frac{\beta_y^2}{\vec{\beta}^2} & (\gamma - 1) \frac{\beta_y \beta_z}{\vec{\beta}^2} \\
    /// -\gamma \beta_z & (\gamma - 1) \frac{\beta_z \beta_x}{\vec{\beta}^2} & (\gamma - 1) \frac{\beta_z \beta_y}{\vec{\beta}^2} & 1 + (\gamma - 1) \frac{\beta_z^2}{\vec{\beta}^2}
    /// \end{pmatrix}
    /// ```
    /// where
    /// $`\vec{\beta} = \frac{\vec{p}}{E}`$ and $`\gamma = \frac{1}{\sqrt{1 - \vec{\beta}^2}}`$.
    pub fn boost_matrix(&self) -> Matrix4<F> {
        let b = self.beta3();
        let b2 = b.dot(&b);
        let g = F::ONE / F::fsqrt(F::ONE - b2);
        Matrix4::new(
            g,
            -g * b[0],
            -g * b[1],
            -g * b[2],
            -g * b[0],
            F::ONE + (g - F::ONE) * b[0] * b[0] / b2,
            (g - F::ONE) * b[0] * b[1] / b2,
            (g - F::ONE) * b[0] * b[2] / b2,
            -g * b[1],
            (g - F::ONE) * b[1] * b[0] / b2,
            F::ONE + (g - F::ONE) * b[1] * b[1] / b2,
            (g - F::ONE) * b[1] * b[2] / b2,
            -g * b[2],
            (g - F::ONE) * b[2] * b[0] / b2,
            (g - F::ONE) * b[2] * b[1] / b2,
            F::ONE + (g - F::ONE) * b[2] * b[2] / b2,
        )
    }
}

impl<F: Field> From<FourMomentum<F>> for Vector4<F> {
    fn from(val: FourMomentum<F>) -> Self {
        Self::new(val.e(), val.px(), val.py(), val.pz())
    }
}

impl<F: Field> From<&FourMomentum<F>> for Vector4<F> {
    fn from(val: &FourMomentum<F>) -> Self {
        Self::new(val.e(), val.px(), val.py(), val.pz())
    }
}

impl<F: Field> From<Vector4<F>> for FourMomentum<F> {
    fn from(value: Vector4<F>) -> Self {
        Self([value[0], value[1], value[2], value[3]])
    }
}

impl<F: Field> From<&Vector4<F>> for FourMomentum<F> {
    fn from(value: &Vector4<F>) -> Self {
        Self([value[0], value[1], value[2], value[3]])
    }
}

impl<F: Field> From<Vec<F>> for FourMomentum<F> {
    fn from(value: Vec<F>) -> Self {
        Self([value[0], value[1], value[2], value[3]])
    }
}

impl<F: Field> From<&Vec<F>> for FourMomentum<F> {
    fn from(value: &Vec<F>) -> Self {
        Self([value[0], value[1], value[2], value[3]])
    }
}

impl<F: Field> Add for FourMomentum<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl<F: Field> Add for &FourMomentum<F> {
    type Output = <FourMomentum<F> as Add>::Output;
    fn add(self, rhs: &FourMomentum<F>) -> Self::Output {
        FourMomentum::add(*self, *rhs)
    }
}

impl<F: Field> Sub for FourMomentum<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl<F: Field> Sub for &FourMomentum<F> {
    type Output = <FourMomentum<F> as Sub>::Output;
    fn sub(self, rhs: &FourMomentum<F>) -> Self::Output {
        FourMomentum::sub(*self, *rhs)
    }
}

impl<F: Field> std::iter::Sum<Self> for FourMomentum<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_is_close;
    #[test]
    fn test_set_components() {
        let mut p = FourMomentum::<f64>::default();
        p.set_e(1.0);
        p.set_px(2.0);
        p.set_py(3.0);
        p.set_pz(4.0);
        assert_is_close!(p.e(), 1.0, f64);
        assert_is_close!(p.px(), 2.0, f64);
        assert_is_close!(p.py(), 3.0, f64);
        assert_is_close!(p.pz(), 4.0, f64);
    }

    #[test]
    fn test_sum() {
        let a = FourMomentum::new(0.1, 0.2, 0.3, 0.4);
        let b = FourMomentum::new(1.0, 2.0, 3.0, 4.0);
        let c = FourMomentum::new(10.0, 20.0, 30.0, 40.0);
        let d: FourMomentum<f64> = [a, b, c].into_iter().sum();
        assert_is_close!(d.e(), 11.1, f64);
        assert_is_close!(d.px(), 22.2, f64);
        assert_is_close!(d.py(), 33.3, f64);
        assert_is_close!(d.pz(), 44.4, f64);
    }

    #[test]
    fn test_ops() {
        let a = FourMomentum::new(0.1, 0.2, 0.3, 0.4);
        let b = FourMomentum::new(1.0, 2.0, 3.0, 4.0);
        let c = a + b;
        let d = b - a;
        assert_is_close!(c.e(), 1.1, f64);
        assert_is_close!(c.px(), 2.2, f64);
        assert_is_close!(c.py(), 3.3, f64);
        assert_is_close!(c.pz(), 4.4, f64);
        assert_is_close!(d.e(), 0.9, f64);
        assert_is_close!(d.px(), 1.8, f64);
        assert_is_close!(d.py(), 2.7, f64);
        assert_is_close!(d.pz(), 3.6, f64);
    }
}
