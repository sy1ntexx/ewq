//! # Ewq - a small math library.

/// Angles.
pub mod ang;
/// Euclidian vectors.
pub mod vec;

mod complex;
pub use complex::*;

mod quat;
pub use quat::*;

mod aabb;
pub use aabb::*;

mod mat;
pub use mat::*;
