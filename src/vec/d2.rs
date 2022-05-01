use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_traits::Float;
use crate::Complex;

pub type Vec2f = Vec2<f32>;
pub type Vec2d = Vec2<f64>;

/// 2D Euclidian vector with X and Y components.
#[derive(Debug, PartialEq, Default, Clone, Copy, PartialOrd)]
#[repr(C)]
pub struct Vec2<F>
where
    F: Float,
{
    pub x: F,
    pub y: F,
}

impl<F> Vec2<F>
where
    F: Float,
{
    /// Creates new vector.
    #[inline(always)]
    pub fn new(x: F, y: F) -> Self {
        Self { x, y }
    }

    /// Splits vector into X and Y components.
    #[inline]
    pub fn split(&self) -> (F, F) {
        (self.x, self.y)
    }

    /// Combines tuple into vector.
    #[inline]
    pub fn combine(xy: (F, F)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }

    /// Split vector into it's basic vectors i, j.
    #[inline]
    pub fn into_basis(self) -> (Self, Self) {
        (
            Self {
                x: self.x,
                y: F::zero(),
            },
            Self {
                x: F::zero(),
                y: self.y,
            },
        )
    }

    /// Computes dot product between two vectors.
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y
    }

    /// Computes dot product between two normalized copies of the vectors.
    #[inline]
    pub fn dot_normalized(&self, other: Self) -> F {
        self.normalized().dot(other.normalized())
    }

    /// Computes the angle between two vectors.
    /// ```
    /// # use ewq::vec::Vec2f;
    /// let a = Vec2f::new(1., 0.);
    /// let b = Vec2f::new(1., 1.);
    /// assert_eq!(a.angle_to(b), std::f32::consts::FRAC_PI_4);
    /// ```
    #[inline]
    pub fn angle_to(&self, other: Self) -> F {
        F::acos(self.dot_normalized(other))
    }

    /// Scales all of the components by `factor`.
    #[inline]
    pub fn scale(&mut self, factor: F) {
        self.x = self.x * factor;
        self.y = self.y * factor;
    }

    /// Computes the magnitude of the vector.
    #[inline]
    pub fn magnitude(&self) -> F {
        F::sqrt(self.sqrt_magnitude())
    }

    /// Computes the squared magnitude of the vector.
    #[inline]
    pub fn sqrt_magnitude(&self) -> F {
        self.x * self.x + self.y * self.y
    }

    /// Computes the distance between two vectors.
    #[inline]
    pub fn distance_to(&self, other: Self) -> F {
        (other - *self).magnitude()
    }

    /// Returns the normalized version of the vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let l = self.magnitude();
        Self {
            x: self.x / l,
            y: self.y / l,
        }
    }

    /// Linearly interpolates between two vectors.
    /// ```
    /// # use ewq::vec::Vec2;
    /// let a = Vec2::new(1., 1.);
    /// let b = Vec2::new(2., 2.);
    /// assert_eq!(a.lerp(b, 0.5), Vec2::new(1.5, 1.5));
    /// ```
    #[inline]
    pub fn lerp(&self, other: Self, t: F) -> Self {
        *self + (other - *self) * (F::one() - t)
    }

    /// Normalizes the vector, preserving direction but reducing its magnitude to `1`.
    #[inline]
    pub fn normalize(&mut self) {
        let l = self.magnitude();
        self.x = self.x / l;
        self.y = self.y / l;
    }
}

impl<F> Add for Vec2<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<F> AddAssign for Vec2<F>
where
    F: Float,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<F> Sub for Vec2<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<F> SubAssign for Vec2<F>
where
    F: Float,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<F> Mul<F> for Vec2<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<F> MulAssign<F> for Vec2<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<F> Div<F> for Vec2<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<F> DivAssign<F> for Vec2<F>
where
    F: Float,
{
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<F> Neg for Vec2<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<F> From<Complex<F>> for Vec2<F>
where
    F: Float,
{
    #[inline]
    fn from(z: Complex<F>) -> Self {
        Self {
            x: z.real,
            y: z.imag,
        }
    }
}
