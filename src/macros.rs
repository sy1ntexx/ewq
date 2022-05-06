#[macro_export]
macro_rules! vecf {
    ($x:expr, $y:expr) => {
        $crate::vec::Vec2f::new($x as f32, $y as f32)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec::Vec3f::new($x as f32, $y as f32, $z as f32)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::vec::Vec4f::new($x as f32, $y as f32, $z as f32, $w as f32)
    };
}

#[macro_export]
macro_rules! vecd {
    ($x:expr, $y:expr) => {
        $crate::vec::Vec2d::new($x as f64, $y as f64)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec::Vec3d::new($x as f64, $y as f64, $z as f64)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::vec::Vec4d::new($x as f64, $y as f64, $z as f64, $w as f64)
    };
}