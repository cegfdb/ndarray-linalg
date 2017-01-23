
use ndarray::{Ix2, Array, LinalgScalar};
use std::fmt::Debug;
use num_traits::float::Float;

use matrix::Matrix;
use square::SquareMatrix;
use error::LinalgError;
use qr::ImplQR;
use svd::ImplSVD;
use norm::ImplNorm;
use solve::ImplSolve;

pub trait TriangularMatrix: Matrix + SquareMatrix {
    /// solve a triangular system with upper triangular matrix
    fn solve_upper(&self, Self::Vector) -> Result<Self::Vector, LinalgError>;
    /// solve a triangular system with lower triangular matrix
    fn solve_lower(&self, Self::Vector) -> Result<Self::Vector, LinalgError>;
}

impl<A> TriangularMatrix for Array<A, Ix2>
    where A: ImplQR + ImplNorm + ImplSVD + ImplSolve + LinalgScalar + Float + Debug
{
    fn solve_upper(&self, b: Self::Vector) -> Result<Self::Vector, LinalgError> {
        self.check_square()?;
        let (n, _) = self.size();
        let layout = self.layout()?;
        let a = self.as_slice_memory_order().unwrap();
        let x = ImplSolve::solve_triangle(layout, 'U' as u8, n, a, b.into_raw_vec())?;
        Ok(Array::from_vec(x))
    }
    fn solve_lower(&self, b: Self::Vector) -> Result<Self::Vector, LinalgError> {
        self.check_square()?;
        let (n, _) = self.size();
        let layout = self.layout()?;
        let a = self.as_slice_memory_order().unwrap();
        let x = ImplSolve::solve_triangle(layout, 'L' as u8, n, a, b.into_raw_vec())?;
        Ok(Array::from_vec(x))
    }
}

pub fn drop_upper(mut a: Array<f64, Ix2>) -> Array<f64, Ix2> {
    for ((i, j), val) in a.indexed_iter_mut() {
        if i < j {
            *val = 0.0;
        }
    }
    a
}

pub fn drop_lower(mut a: Array<f64, Ix2>) -> Array<f64, Ix2> {
    for ((i, j), val) in a.indexed_iter_mut() {
        if i > j {
            *val = 0.0;
        }
    }
    a
}
