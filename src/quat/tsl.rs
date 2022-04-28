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

    #[inline]
    pub fn idenitity() -> Self {
        Self {
            q: Quat::idenitity(),
            t: Vec3::zero(),
        }
    }

    /// Applies rotation and then translation to the vector.
    #[inline]
    pub fn apply(&self, vector: Vec3<F>) -> Vec3<F> {
        self.q.rotate(vector) + self.t
    }
}
