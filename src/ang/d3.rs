use crate::{vec::Vec3, Quat};
use num_traits::Float;

pub type Ang3f = Ang3<f32>;
pub type Ang3d = Ang3<f32>;

/// Represents rotation angles in right hand coordinate system with:
/// * `+X` - Forward
/// * `-Y` - Right
/// * `+Z` - Top
#[repr(C)]
pub struct Ang3<F>
where
    F: Float,
{
    /// Rotation around +Z.
    pub yaw: F,
    /// Rotation around -Y.
    pub pitch: F,
    /// Rotation around +X.
    pub roll: F,
}

impl<F> Ang3<F>
where
    F: Float,
{
    /// Creates new rotation.
    #[inline]
    pub fn new(yaw: F, pitch: F, roll: F) -> Self {
        Self { yaw, pitch, roll }
    }

    /// Creates a quaternion representing this rotation in order Yaw, Pitch, Roll.
    #[inline]
    pub fn into_rotation(self) -> Quat<F> {
        Quat::new_axis_rotation(Vec3::yaw(), self.yaw)
            .combine(Quat::new_axis_rotation(Vec3::pitch(), self.pitch))
            .combine(Quat::new_axis_rotation(Vec3::roll(), self.roll))
    }
}

impl<F> From<Vec3<F>> for Ang3<F>
where
    F: Float,
{
    fn from(v: Vec3<F>) -> Self {
        Self {
            yaw: v.x,
            pitch: v.y,
            roll: v.z,
        }
    }
}
