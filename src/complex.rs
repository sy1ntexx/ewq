use crate::vec::Vec2;
use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Complex number with real and imaginary parts.
#[derive(Debug, PartialEq, Default, Clone, Copy)]
#[repr(C)]
pub struct Complex<F>
where
    F: Float,
{
    pub real: F,
    pub imag: F,
}

impl<F> Complex<F>
where
    F: Float,
{
    /// Creates new complex number.
    #[inline]
    pub fn new(real: F, imag: F) -> Self {
        Self { real, imag }
    }

    /// Computes the reciprocal of the complex number.
    #[inline]
    pub fn reciprocal(&self) -> Self {
        Self {
            real: self.real / (self.real * self.real + self.imag * self.imag),
            imag: -self.imag / (self.real * self.real + self.imag * self.imag),
        }
    }

    /// Computes the conjugate of the complex number
    #[inline]
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Raises complex number to the second power.
    pub fn pow2(&self) -> Self {
        Self {
            real: self.real * self.real - self.imag * self.imag,
            imag: F::from(2).unwrap() * self.real * self.imag,
        }
    }
}

impl<F> Add for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<F> AddAssign for Complex<F>
where
    F: Float,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.real = self.real + rhs.real;
        self.imag = self.imag + rhs.imag;
    }
}

impl<F> Sub for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl<F> SubAssign for Complex<F>
where
    F: Float,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.real = self.real - rhs.real;
        self.imag = self.imag - rhs.imag;
    }
}

impl<F> Mul<F> for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl<F> Mul for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl<F> MulAssign for Complex<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.real = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
    }
}

impl<F> MulAssign<F> for Complex<F>
where
    F: Float,
{
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.real = self.real * rhs;
        self.imag = self.imag * rhs;
    }
}

impl<F> Div<F> for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl<F> DivAssign<F> for Complex<F>
where
    F: Float,
{
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.real = self.real / rhs;
        self.imag = self.imag / rhs;
    }
}

impl<F> Neg for Complex<F>
where
    F: Float,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<F> From<Vec2<F>> for Complex<F>
where
    F: Float,
{
    fn from(v: Vec2<F>) -> Self {
        Self {
            real: v.x,
            imag: v.y,
        }
    }
}
