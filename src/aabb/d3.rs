use num_traits::Float;
use crate::vec::Vec3;

pub type Aabb3f = Aabb3<f32>;
pub type Aabb3d = Aabb3<f64>;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Aabb3<F>
where
    F: Float
{
    pub min: Vec3<F>,
    pub max: Vec3<F>
}

impl<F> Aabb3<F>
where
    F: Float
{
    /// Creates new [`Aabb3`] with `min` and `max` vectors.
    #[inline]
    pub fn new(min: Vec3<F>, max: Vec3<F>) -> Self {
        Self { min, max }
    }

    /// Checks if `max` is greater than `min`.
    #[inline]
    pub fn is_right(&self) -> bool {
        self.max > self.min
    }

    /// Inverts [`Aabb3`] by swapping `min` and `max` in place.
    #[inline]
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.min, &mut self.max);
    }

    /// Returns an inverted copy of the [`Aabb3`] with swapped `min` and `max`.
    #[inline]
    pub fn inverted(&self) -> Self {
        Self {
            min: self.max,
            max: self.min
        }
    }

    /// Checks if the vector is inside of the [`Aabb3`].
    #[inline]
    pub fn is_inside(&self, vec: Vec3<F>) -> bool {
        self.min > vec && vec > self.max
    }

    /// Checks if the vector is outside of the [`Aabb3`].
    #[inline]
    pub fn is_outside(&self, vec: Vec3<F>) -> bool {
        !self.is_inside(vec)
    }

    /// Computes the volume of the bouding box.
    /// ```
    /// # use ewq::{vecf, aabb::Aabb3};
    /// let aabb = Aabb3::new(vecf!(1., 1., 1.), vecf!(2., 2., 2.));
    /// assert_eq!(aabb.volume(), 1.);
    /// ```
    #[inline]
    pub fn volume(&self) -> F {
        (self.max - self.min).cuboid_volume()
    }
}