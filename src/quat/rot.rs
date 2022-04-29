use crate::vec::Vec3;
use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub type Quatf = Quat<f32>;
pub type Quatd = Quat<f64>;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
#[repr(C)]
pub struct Quat<F>
where
    F: Float,
{
    /// Vector part.
    v: Vec3<F>,
    /// Scalar part.
    w: F,
}

impl<F> Quat<F>
where
    F: Float,
{
    /// Creates new quaternion.
    #[inline]
    pub fn new(v: Vec3<F>, w: F) -> Self {
        Self { v, w }
    }

    /// Creates new quaternion with `v` as vector part and `0` for scalar.
    #[inline]
    pub fn new_vector(v: Vec3<F>) -> Self {
        Self { v, w: F::zero() }
    }

    /// Creates new quaternion that represents a rotation around `axis` by `angle` radians.
    /// Method can accept denormalized input.
    #[inline]
    pub fn new_axis_rotation(axis: Vec3<F>, angle: F) -> Self {
        Self {
            v: axis.normalized() * angle.div(F::from(2).unwrap()).sin(),
            w: angle.div(F::from(2).unwrap()).cos(),
        }
    }

    /// Creates new quaternion that represents a rotation that moves vector `from` to vector `to`.
    #[inline]
    pub fn new_dst_rotation(from: Vec3<F>, to: Vec3<F>) -> Self {
        let normal = from.cross(to);
        let angle = to.dot_normalized(from).acos();
        Self::new_axis_rotation(normal, angle)
    }

    /// Recovers angle axis representation of the quaternion.
    #[inline]
    pub fn into_axis_angle(self) -> (Vec3<F>, F) {
        (
            self.v.normalized(),
            F::atan2(self.v.magnitude(), self.w) * F::from(2).unwrap(),
        )
    }

    /// Rotates vector `v` by the rotation defined by this quaternion.
    #[inline]
    pub fn rotate(&self, v: Vec3<F>) -> Vec3<F> {
        self.product(Self::new_vector(v))
            .product(self.reciprocal())
            .v
    }

    /// Creates new zero quaternion.
    #[inline]
    pub fn zero() -> Self {
        Self {
            w: F::zero(),
            v: Vec3::zero(),
        }
    }

    /// Creates new identity quaternion.
    #[inline]
    pub fn identity() -> Self {
        Self {
            w: F::one(),
            v: Vec3::zero(),
        }
    }

    /// Creates new quaternion from individual parts.
    #[inline]
    pub fn from_parts(x: F, y: F, z: F, w: F) -> Self {
        Self {
            v: Vec3 { x, y, z },
            w,
        }
    }

    /// Computes the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(&self) -> Self {
        Self {
            v: -self.v,
            w: self.w,
        }
    }

    /// Computes the norm of the quaternion.
    #[inline]
    pub fn norm(&self) -> F {
        F::sqrt(self.sqrt_norm())
    }

    /// Computes the squared norm of the quaternion.
    #[inline]
    pub fn sqrt_norm(&self) -> F {
        self.v.sqrt_magnitude() + self.w * self.w
    }

    /// Normalizes quaternion preserving direction but reducing its norm to `1`.
    #[inline]
    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    /// Computes the reciprocal of the quaternion.
    #[inline]
    pub fn reciprocal(&self) -> Self {
        self.conjugate() / self.sqrt_norm()
    }

    /// Combines two rotations. Applying `self` first and then `other.
    #[inline]
    pub fn combine(self, other: Self) -> Self {
        other.product(self)
    }

    /// Computes the hamilton product between two vectors.
    #[inline]
    pub fn product(&self, other: Self) -> Self {
        Self {
            v: Vec3 {
                x: self.w * other.v.x + self.v.x * other.w + self.v.y * other.v.z
                    - self.v.z * other.v.y,
                y: self.w * other.v.y - self.v.x * other.v.z
                    + self.v.y * other.w
                    + self.v.z * other.v.x,
                z: self.w * other.v.z + self.v.x * other.v.y - self.v.y * other.v.x
                    + self.v.z * other.w,
            },
            w: self.w * other.w
                - self.v.x * other.v.x
                - self.v.y * other.v.y
                - self.v.z * other.v.z,
        }
    }
}

impl<F> Add for Quat<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v + rhs.v,
            w: self.w + rhs.w,
        }
    }
}

impl<F> AddAssign for Quat<F>
where
    F: Float,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.v = self.v + rhs.v;
        self.w = self.w + rhs.w;
    }
}

impl<F> Sub for Quat<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v - rhs.v,
            w: self.w - rhs.w,
        }
    }
}

impl<F> SubAssign for Quat<F>
where
    F: Float,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.v = self.v - rhs.v;
        self.w = self.w - rhs.w;
    }
}

impl<F> Mul<F> for Quat<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            v: self.v * rhs,
            w: self.w * rhs,
        }
    }
}

impl<F> MulAssign<F> for Quat<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.v = self.v * rhs;
        self.w = self.w * rhs;
    }
}

impl<F> Div<F> for Quat<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            v: self.v / rhs,
            w: self.w / rhs,
        }
    }
}

impl<F> DivAssign<F> for Quat<F>
where
    F: Float,
{
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.v = self.v / rhs;
        self.w = self.w / rhs;
    }
}
