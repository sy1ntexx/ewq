use num_traits::Float;

mod d2;
pub use d2::*;

mod d3;
pub use d3::*;

mod d4;
pub use d4::*;

pub trait VectorConst<F>
where
    F: Float,
{
    const SIZE: usize = 0;

    fn get<const I: usize>(&self) -> F;
    fn set<const I: usize>(&mut self, v: F);
}

pub trait Vector<F>: Clone
where
    F: Float,
{
    const SIZE: usize = 0;

    fn get(&self, i: usize) -> F;
    fn set(&mut self, i: usize, v: F);
}
