use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_traits::Float;
use super::Vec2;

pub type Vec3f = Vec3<f32>;
pub type Vec3d = Vec3<f64>;

/// 3D Euclidian vector with X, Y and Z components.
#[derive(Debug, PartialEq, Default, Clone, Copy, PartialOrd)]
#[repr(C)]
pub struct Vec3<F>
where
    F: Float,
{
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F> Vec3<F>
where
    F: Float,
{
    #[inline]
    pub fn yaw() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::one(),
        }
    }

    #[inline]
    pub fn pitch() -> Self {
        Self {
            x: F::zero(),
            y: -F::one(),
            z: F::zero(),
        }
    }

    #[inline]
    pub fn roll() -> Self {
        Self {
            x: F::one(),
            y: F::zero(),
            z: F::zero(),
        }
    }
}

impl<F> Vec3<F>
where
    F: Float,
{
    #[inline]
    pub fn one() -> Self {
        Self {
            x: F::one(),
            y: F::one(),
            z: F::one(),
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    #[inline]
    pub fn top() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::one(),
        }
    }

    #[inline]
    pub fn bottom() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: -F::one(),
        }
    }

    #[inline]
    pub fn right() -> Self {
        Self {
            x: F::zero(),
            y: -F::one(),
            z: F::zero(),
        }
    }

    #[inline]
    pub fn left() -> Self {
        Self {
            x: F::zero(),
            y: F::one(),
            z: F::zero(),
        }
    }

    #[inline]
    pub fn forward() -> Self {
        Self {
            x: F::one(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    #[inline]
    pub fn back() -> Self {
        Self {
            x: -F::one(),
            y: F::zero(),
            z: F::zero(),
        }
    }
}

impl<F> Vec3<F>
where
    F: Float,
{
    /// Creates new vector.
    #[inline(always)]
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    /// Reduces Vec3 to Vec2.
    #[inline(always)]
    pub fn reduce(self) -> Vec2<F> {
        Vec2::new(self.x, self.y)
    }

    #[inline]
    pub fn from_vec2(v: Vec2<F>, z: F) -> Self {
        Self { x: v.x, y: v.y, z }
    }

    /// Splits vector into X and Y components.
    #[inline]
    pub fn split(&self) -> (F, F, F) {
        (self.x, self.y, self.z)
    }

    /// Combines tuple into vector.
    #[inline]
    pub fn combine(xyz: (F, F, F)) -> Self {
        Self {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
        }
    }

    /// Split vector into it's basic vectors `i`, `j`, `k`.
    #[inline]
    pub fn into_basis(self) -> (Self, Self, Self) {
        (
            Self {
                x: self.x,
                y: F::zero(),
                z: F::zero(),
            },
            Self {
                x: F::zero(),
                y: self.y,
                z: F::zero(),
            },
            Self {
                x: F::zero(),
                y: F::zero(),
                z: self.z,
            },
        )
    }

    /// Computes dot product between two vectors.
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes dot product between two normalized copies of the vectors.
    #[inline]
    pub fn dot_normalized(&self, other: Self) -> F {
        self.normalized().dot(other.normalized())
    }

    /// Computes the distance between two vectors.
    #[inline]
    pub fn distance_to(&self, other: Self) -> F {
        (other - *self).magnitude()
    }

    /// Scales all of the components by `factor`.
    #[inline]
    pub fn scale(&mut self, factor: F) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }

    /// Computes the magnitude of the vector.
    #[inline]
    pub fn magnitude(&self) -> F {
        F::sqrt(self.sqrt_magnitude())
    }

    /// Computes the squared magnitude of the vector.
    #[inline]
    pub fn sqrt_magnitude(&self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the normalized version of the vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let l = self.magnitude();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    /// Normalizes the vector, preserving direction but reducing its magnitude to `1`.
    #[inline]
    pub fn normalize(&mut self) {
        let l = self.magnitude();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
    }

    /// Computes the angle between two vectors.
    /// ```
    /// # use ewq::vec::Vec3f;
    /// let a = Vec3f::new(1., 0., 0.);
    /// let b = Vec3f::new(1., 1., 0.);
    /// assert_eq!(a.angle_to(b), std::f32::consts::FRAC_PI_4);
    /// ```
    #[inline]
    pub fn angle_to(&self, other: Self) -> F {
        F::acos(self.dot_normalized(other))
    }

    /// Linearly interpolates between two vectors.
    /// ```
    /// # use ewq::vec::Vec3;
    /// let a = Vec3::new(-1., -1., -1.);
    /// let b = Vec3::new(1., 1., 1.);
    /// assert_eq!(a.lerp(b, 0.5), Vec3::new(0., 0., 0.));
    /// ```
    #[inline]
    pub fn lerp(&self, other: Self, t: F) -> Self {
        *self + (other - *self) * (F::one() - t)
    }

    /// Computes the cross product between two vectors.
    /// ```
    /// # use ewq::vec::Vec3f;
    /// let a = Vec3f::new(1., 0., 0.);
    /// let b = Vec3f::new(0., 1., 0.);
    /// assert_eq!(a.cross(b), Vec3f::new(0., 0., 1.))
    /// ```
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Computes the triple product between three vectors in order.
    #[inline]
    pub fn triple(&self, former: Self, latter: Self) -> F {
        self.dot(former.cross(latter))
    }

    /// Computes the volume of the cuboid with a diagonal equals to the vector.
    #[inline]
    pub fn cuboid_volume(&self) -> F {
        self.x * self.y * self.z
    }
}

impl<F> Add for Vec3<F>
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
        }
    }
}

impl<F> AddAssign for Vec3<F>
where
    F: Float,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<F> Sub for Vec3<F>
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
        }
    }
}

impl<F> SubAssign for Vec3<F>
where
    F: Float,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<F> Mul<F> for Vec3<F>
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
        }
    }
}

impl<F> MulAssign<F> for Vec3<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<F> Div<F> for Vec3<F>
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
        }
    }
}

impl<F> DivAssign<F> for Vec3<F>
where
    F: Float,
{
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<F> Neg for Vec3<F>
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
        }
    }
}
