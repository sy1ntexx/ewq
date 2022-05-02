use crate::vec::{Vec3, Vec4};
use num_traits::Float;
use std::ops::Mul;

pub type Mat3x4f = Mat3x4<f32>;
pub type Mat3x4d = Mat3x4<f64>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Mat3x4<F>
where
    F: Float,
{
    r0: Vec4<F>,
    r1: Vec4<F>,
    r2: Vec4<F>,
}

impl<F> Mat3x4<F>
where
    F: Float,
{
    /// Creates new matrix from rows.
    #[inline]
    pub fn new(r0: Vec4<F>, r1: Vec4<F>, r2: Vec4<F>) -> Self {
        Self { r0, r1, r2 }
    }

    /// Gets matrix's row by index.
    /// # Panics
    /// If index is `>2`.
    #[inline]
    pub fn row<const I: usize>(&self) -> Vec4<F> {
        match I {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            _ => panic!("Index out of range"),
        }
    }

    /// Sets matrix's row by index.
    /// # Panics
    /// If index is `>2`.
    #[inline]
    pub fn set_row<const I: usize>(&mut self, row: Vec4<F>) {
        match I {
            0 => self.r0 = row,
            1 => self.r1 = row,
            2 => self.r2 = row,
            _ => panic!("Index out of range")
        }
    }

    /// Gets matrix's column by index.
    /// # Panics
    /// If index is `>3`.
    #[inline]
    pub fn column<const I: usize>(&self) -> Vec3<F> {
        match I {
            0 => Vec3::new(self.r0.x, self.r1.x, self.r2.x),
            1 => Vec3::new(self.r0.y, self.r1.y, self.r2.y),
            2 => Vec3::new(self.r0.z, self.r1.z, self.r2.z),
            3 => Vec3::new(self.r0.w, self.r1.w, self.r2.w),
            _ => panic!("Index out of range"),
        }
    }

    /// Sets matrix's column by index.
    /// # Panics
    /// If index is `>3`.
    #[inline]
    pub fn set_column<const I: usize>(&mut self, column: Vec3<F>) {
        match I {
            0 => {
                self.r0.x = column.x;
                self.r1.x = column.y;
                self.r2.x = column.z;
            }
            1 => {
                self.r0.y = column.x;
                self.r1.y = column.y;
                self.r2.y = column.z;
            }
            2 => {
                self.r0.z = column.x;
                self.r1.z = column.y;
                self.r2.z = column.z;
            }
            3 => {
                self.r0.w = column.x;
                self.r1.w = column.y;
                self.r2.w = column.z;
            }
            _ => panic!("Index out of range"),
        }
    }

    /// Creates new identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Self {
            r0: Vec4::new(F::one(), F::zero(), F::zero(), F::zero()),
            r1: Vec4::new(F::zero(), F::one(), F::zero(), F::zero()),
            r2: Vec4::new(F::zero(), F::zero(), F::one(), F::zero()),
        }
    }
}

impl<F> Mul<Mat3x4<F>> for Vec4<F>
where
    F: Float,
{
    type Output = Vec3<F>;

    #[inline]
    fn mul(self, rhs: Mat3x4<F>) -> Self::Output {
        Vec3 {
            x: self.dot(rhs.r0),
            y: self.dot(rhs.r1),
            z: self.dot(rhs.r2),
        }
    }
}
