//! # Ewq - a small math library.
#![warn(missing_debug_implementations)]

/// Angles.
pub mod ang;
/// Euclidian vectors.
pub mod vec;
/// Axis aligned bounding boxes.
pub mod aabb;
/// Matricies.
pub mod mat;

mod complex;
pub use complex::*;

mod quat;
pub use quat::*;

pub mod macros;
