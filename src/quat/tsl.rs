use crate::{vec::Vec3, Quat};
use num_traits::Float;

/// Compound struct for rotation and translation.
pub struct QuatT<F>
where
    F: Float,
{
    /// Quaternion rotation.
    pub q: Quat<F>,
    /// Translation vector.
    pub t: Vec3<F>,
}

impl<F> QuatT<F>
where
    F: Float,
{
    /// Creates new [`QuatT`] from rotation quaternion and translation vector.
    #[inline]
    pub fn new(rotation: Quat<F>, translation: Vec3<F>) -> Self {
        Self {
            q: rotation,
            t: translation,
        }
    }

    /// Creates new [`QuatT`] with identity rotation and 0 translation.
    #[inline]
    pub fn identity() -> Self {
        Self {
            q: Quat::identity(),
            t: Vec3::zero(),
        }
    }

    /// Applies rotation and then translation to the vector.
    #[inline]
    pub fn apply(&self, vector: Vec3<F>) -> Vec3<F> {
        self.q.rotate(vector) + self.t
    }

    /// Applies translation and then rotation to the vector.
    #[inline]
    pub fn apply_reverse(&self, vector: Vec3<F>) -> Vec3<F> {
        self.q.rotate(vector + self.t)
    }
}
