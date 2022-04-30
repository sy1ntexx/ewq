//! # Ewq - a small math library.

/// Angles.
pub mod ang;
/// Euclidian vectors.
pub mod vec;
/// Axis aligned bounding boxes.
pub mod aabb;

mod complex;
pub use complex::*;

mod quat;
pub use quat::*;


mod mat;
pub use mat::*;

mod macros;
