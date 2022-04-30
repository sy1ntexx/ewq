#[macro_export]
macro_rules! vecf {
    ($x:expr, $y:expr) => {
        $crate::vec::Vec2f::new($x, $y)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec::Vec3f::new($x, $y, $z)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::vec::Vec4f::new($x, $y, $z, $w)
    };
}

#[macro_export]
macro_rules! vecd {
    ($x:expr, $y:expr) => {
        $crate::vec::Vec2d::new($x, $y)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec::Vec3d::new($x, $y, $z)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::vec::Vec4d::new($x, $y, $z, $w)
    };
}