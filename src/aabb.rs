use crate::vec::{Vec2d, Vec2f, Vec3d, Vec3f, Vector};
use num_traits::Float;
use std::marker::PhantomData;

/// Axis aligned bounding box.

pub type Aabb2f = Aabb<f32, Vec2f>;
pub type Aabb2d = Aabb<f64, Vec2d>;
pub type Aabb3f = Aabb<f32, Vec3f>;
pub type Aabb3d = Aabb<f64, Vec3d>;

#[derive(Debug, PartialEq, Clone)]
#[repr(C)]
pub struct Aabb<F, V>
where
    F: Float,
    V: Vector<F>,
{
    pub min: V,
    pub max: V,
    _ph: PhantomData<F>,
}

impl<F, V> Aabb<F, V>
where
    F: Float,
    V: Vector<F>,
{
    /// Creates new axis aligned bounding box.
    #[inline]
    pub fn new(min: V, max: V) -> Self {
        Self {
            _ph: PhantomData::default(),
            min,
            max,
        }
    }

    /// Inverts the bounding box swapping `min` and `max`.
    #[inline]
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.min, &mut self.max);
    }

    /// Returns the inverted copy of the bounding box.
    #[inline]
    pub fn inverted(&self) -> Self {
        Self {
            min: self.max.clone(),
            max: self.min.clone(),
            _ph: PhantomData::default(),
        }
    }

    /// Checks if `max` is bigger than `min`.
    #[inline]
    pub fn is_right(&self) -> bool {
        for i in 0..V::SIZE {
            if self.min.get(i) > self.max.get(i) {
                return false;
            }
        }
        return true;
    }

    /// Checks if the point is inside of the bounding box.
    #[inline]
    pub fn is_inside(&self, v: V) -> bool {
        for i in 0..V::SIZE {
            if !(self.min.get(i) <= v.get(i) && v.get(i) <= self.max.get(i)) {
                return false;
            }
        }
        true
    }
}
