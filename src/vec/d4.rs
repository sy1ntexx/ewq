use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super::{Vec2, Vec3};
use num_traits::Float;

pub type Vec4f = Vec4<f32>;
pub type Vec4d = Vec4<f64>;

/// 4D Euclidian vector with X, Y, Z and W components.
#[derive(Debug, PartialEq, Default, Clone, Copy, PartialOrd)]
#[repr(C)]
pub struct Vec4<F>
where
    F: Float,
{
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl<F> Vec4<F>
where
    F: Float,
{
    /// Creates new vector.
    #[inline(always)]
    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

    /// Creates new vector by expanding 3D vector.
    #[inline]
    pub fn from_vec3(v: Vec3<F>, w: F) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }

    /// Creates new vector from two 2D vectors.
    #[inline]
    pub fn from_vec2(xy: Vec2<F>, zw: Vec2<F>) -> Self {
        Self {
            x: xy.x,
            y: xy.y,
            z: zw.x,
            w: zw.y,
        }
    }

    /// Splits vector into X and Y components.
    #[inline]
    pub fn split(&self) -> (F, F, F, F) {
        (self.x, self.y, self.z, self.w)
    }

    /// Combines tuple into vector.
    #[inline]
    pub fn combine(xyzw: (F, F, F, F)) -> Self {
        Self {
            x: xyzw.0,
            y: xyzw.1,
            z: xyzw.2,
            w: xyzw.3,
        }
    }

    /// Split vector into it's basic vectors `i`, `j`, `k`, `l`.
    #[inline]
    pub fn into_basis(self) -> (Self, Self, Self, Self) {
        (
            Self {
                x: self.x,
                y: F::zero(),
                z: F::zero(),
                w: F::zero(),
            },
            Self {
                x: F::zero(),
                y: self.y,
                z: F::zero(),
                w: F::zero(),
            },
            Self {
                x: F::zero(),
                y: F::zero(),
                z: self.z,
                w: F::zero(),
            },
            Self {
                x: F::zero(),
                y: F::zero(),
                z: F::zero(),
                w: self.w,
            },
        )
    }

    /// Computes the distance between two vectors.
    #[inline]
    pub fn distance_to(&self, other: Self) -> F {
        (other - *self).magnitude()
    }

    /// Computes dot product between two vectors.
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Computes dot product between two normalized copies of the vectors.
    #[inline]
    pub fn dot_normalized(&self, other: Self) -> F {
        self.normalized().dot(other.normalized())
    }

    /// Computes the angle between two vectors.
    /// ```
    /// # use ewq::vec::Vec4f;
    /// let a = Vec4f::new(1., 0., 0., 0.);
    /// let b = Vec4f::new(1., 1., 0., 0.);
    /// assert_eq!(a.angle_to(b), std::f32::consts::FRAC_PI_4);
    /// ```
    #[inline]
    pub fn angle_to(&self, other: Self) -> F {
        F::acos(self.dot_normalized(other))
    }

    /// Linearly interpolates between two vectors.
    #[inline]
    pub fn lerp(&self, other: Self, t: F) -> Self {
        *self + (other - *self) * (F::one() - t)
    }

    /// Scales all of the components by `factor`.
    #[inline]
    pub fn scale(&mut self, factor: F) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
        self.w = self.w * factor;
    }

    /// Computes the magnitude of the vector.
    #[inline]
    pub fn magnitude(&self) -> F {
        F::sqrt(self.sqrt_magnitude())
    }

    /// Computes the squared magnitude of the vector.
    #[inline]
    pub fn sqrt_magnitude(&self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the normalized version of the vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let l = self.magnitude();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
            w: self.w / l,
        }
    }

    /// Normalizes the vector, preserving direction but reducing its magnitude to `1`.
    #[inline]
    pub fn normalize(&mut self) {
        let l = self.magnitude();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
        self.w = self.w / l;
    }
}

impl<F> Add for Vec4<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<F> AddAssign for Vec4<F>
where
    F: Float,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl<F> Sub for Vec4<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<F> SubAssign for Vec4<F>
where
    F: Float,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

impl<F> Mul<F> for Vec4<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<F> MulAssign<F> for Vec4<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

impl<F> Div<F> for Vec4<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<F> DivAssign<F> for Vec4<F>
where
    F: Float,
{
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w = self.w / rhs;
    }
}

impl<F> Neg for Vec4<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
