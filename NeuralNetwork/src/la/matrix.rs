use std::ops::{Index, IndexMut};

use crate::la::ops;
use crate::la::vector::Vector;

#[derive(Clone, Debug)]
pub struct Matrix2D {
    data: Vec<f64>,
    nrows: usize,
    ncols: usize,
}

impl Matrix2D {
    pub fn new(nrows: usize, ncols: usize) -> Matrix2D {
        Matrix2D {
            data: vec![0.0; ncols * nrows],
            nrows,
            ncols,
        }
    }

    pub fn new_from_data(nrows: usize, ncols: usize, data: Vec<f64>) -> Self {
        assert_eq!(nrows * ncols, data.len(), "Not enough data provided for matrix initialization");
        Matrix2D { data, nrows, ncols }
    }

    fn linear_index(&self, row: usize, col: usize) -> usize {
        // row-major memory layout
        let linear_index = row * self.ncols + col;
        assert!(linear_index < self.data.len(), "Matrix.linear_index: Index too large");
        linear_index
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn transpose(&self) -> Matrix2D {
        let mut t = Matrix2D::new(self.ncols, self.nrows);
        for col in 0..self.ncols {
            for row in 0..self.nrows {
                t[(col, row)] = self[(row, col)];
            }
        }
        t
    }

    pub fn ax(&self, v: &Vector) -> Vector {
        ops::ax(self, v)
    }
}

impl Index<(usize, usize)> for Matrix2D {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let linear_index = index.0 * self.ncols + index.1;
        if linear_index >= self.data.len() {
            panic!("Index ({}, {}) invalid", index.0, index.1)
        }
        &self.data[linear_index]
    }
}

impl IndexMut<(usize, usize)> for Matrix2D {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let linear_index = index.0 * self.ncols + index.1;
        if linear_index >= self.data.len() {
            panic!("Index ({}, {}) invalid", index.0, index.1)
        }
        &mut self.data[linear_index]
    }
}

fn binary_op<F>(lhs: &mut Matrix2D, rhs: &Matrix2D, f: F)
where
    F: Fn(f64, f64) -> f64,
{
    assert_eq!(lhs.nrows(), rhs.nrows(), "Matrices must have same number of rows");
    assert_eq!(lhs.ncols(), rhs.ncols(), "Matrices must have same number of columns");
    for idx in 0..lhs.data.len() {
        let v1 = lhs.data[idx];
        let v2 = rhs.data[idx];
        lhs.data[idx] = f(v1, v2);
    }
}

impl std::ops::AddAssign<&Matrix2D> for Matrix2D {
    fn add_assign(&mut self, other: &Self) {
        binary_op(self, other, |x, y| x + y);
    }
}

impl std::ops::SubAssign<&Matrix2D> for Matrix2D {
    fn sub_assign(&mut self, other: &Self) {
        binary_op(self, other, |x, y| x - y);
    }
}

impl std::ops::DivAssign<usize> for Matrix2D {
    fn div_assign(&mut self, rhs: usize) {
        // SS: collect to force evaluation as lazy...
        self.data.iter_mut().for_each(|v| *v /= rhs as f64);
    }
}

impl std::ops::Mul<&Matrix2D> for f64 {
    type Output = Matrix2D;

    fn mul(self, rhs: &Matrix2D) -> Self::Output {
        let mut result = Matrix2D::new(rhs.nrows, rhs.ncols);
        for idx in 0..rhs.data.len() {
            let value = &rhs.data[idx];
            result.data[idx] = *value * self;
        }
        result
    }
}

impl std::ops::Div<f64> for &Matrix2D {
    type Output = Matrix2D;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = Matrix2D::new(self.nrows, self.ncols);
        for idx in 0..self.data.len() {
            let value = &self.data[idx];
            result.data[idx] = *value / rhs;
        }
        result
    }
}

impl std::ops::Add<&Matrix2D> for Matrix2D {
    type Output = Matrix2D;

    fn add(self, rhs: &Matrix2D) -> Self::Output {
        assert_eq!(self.nrows(), rhs.nrows(), "Matrices must have same number of rows");
        assert_eq!(self.ncols(), rhs.ncols(), "Matrices must have same number of columns");
        let mut result = Matrix2D::new(rhs.nrows, rhs.ncols);
        for idx in 0..rhs.data.len() {
            let value = &self.data[idx];
            let other = &rhs.data[idx];
            result.data[idx] = value + other;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_transpose() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(1, 0)] = 4.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 6.0;

        // Act
        let tr = m.transpose();

        // Assert
        assert_eq!(3, tr.nrows());
        assert_eq!(2, tr.ncols());
        assert_approx_eq!(1.0, tr[(0, 0)], 1e-3f64);
        assert_approx_eq!(4.0, tr[(0, 1)], 1e-3f64);
        assert_approx_eq!(2.0, tr[(1, 0)], 1e-3f64);
        assert_approx_eq!(5.0, tr[(1, 1)], 1e-3f64);
        assert_approx_eq!(3.0, tr[(2, 0)], 1e-3f64);
        assert_approx_eq!(6.0, tr[(2, 1)], 1e-3f64);
    }

    #[test]
    fn test_two_transpose() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(1, 0)] = 4.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 6.0;

        // Act
        let tr = m.transpose();
        let tr2 = tr.transpose();

        // Assert
        assert_eq!(2, tr2.nrows());
        assert_eq!(3, tr2.ncols());
        assert_approx_eq!(1.0, tr2[(0, 0)], 1e-3f64);
        assert_approx_eq!(2.0, tr2[(0, 1)], 1e-3f64);
        assert_approx_eq!(3.0, tr2[(0, 2)], 1e-3f64);
        assert_approx_eq!(4.0, tr2[(1, 0)], 1e-3f64);
        assert_approx_eq!(5.0, tr2[(1, 1)], 1e-3f64);
        assert_approx_eq!(6.0, tr2[(1, 2)], 1e-3f64);
    }

    #[test]
    fn test_nrows() {
        // Arrange
        let m = Matrix2D::new(2, 3);

        // Act
        let nrows = m.nrows();

        // Assert
        assert_eq!(2, nrows)
    }

    #[test]
    fn test_ncols() {
        // Arrange
        let m = Matrix2D::new(2, 3);

        // Act
        let ncols = m.ncols();

        // Assert
        assert_eq!(3, ncols)
    }

    #[test]
    fn test_index() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(1, 0)] = 4.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 6.0;

        // Assert
        assert_eq!(3.0, m[(0, 2)]);
        assert_eq!(5.0, m[(1, 1)])
    }

    #[test]
    fn test_index_tuple() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(1, 0)] = 4.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 6.0;

        // Assert
        assert_eq!(3.0, m[(0, 2)]);
    }

    #[test]
    fn test_indexmut_tuple() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(1, 0)] = 4.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 6.0;

        // Assert
        assert_eq!(3.0, m[(0, 2)]);
    }

    #[test]
    fn test_addassign() {
        // Arrange
        let mut m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let mut m2 = Matrix2D::new_from_data(3, 3, vec![7.0, 2.0, 9.0, 1.0, 2.0, 5.0, 0.0, 2.0, 4.0]);

        // Act
        m1 += &m2;

        // Assert
        assert_approx_eq!(8.0, m1[(0, 0)]);
        assert_approx_eq!(9.0, m1[(1, 2)]);
        assert_approx_eq!(1.0, m1[(2, 0)]);
        assert_approx_eq!(1.0, m1[(2, 1)]);
        assert_approx_eq!(7.0, m1[(2, 2)]);
    }

    #[test]
    fn test_subassign() {
        // Arrange
        let mut m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let mut m2 = Matrix2D::new_from_data(3, 3, vec![7.0, 2.0, 9.0, 1.0, 2.0, 5.0, 0.0, 2.0, 4.0]);

        // Act
        m1 -= &m2;

        // Assert
        assert_approx_eq!(-6.0, m1[(0, 0)]);
        assert_approx_eq!(-1.0, m1[(1, 2)]);
        assert_approx_eq!(1.0, m1[(2, 0)]);
        assert_approx_eq!(-3.0, m1[(2, 1)]);
        assert_approx_eq!(-1.0, m1[(2, 2)]);
    }

    #[test]
    fn test_divassign() {
        // Arrange
        let mut m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let scalar = 15.0;

        // Act
        m1 /= scalar as usize;

        // Assert
        assert_approx_eq!(1.0 / scalar, m1[(0, 0)]);
        assert_approx_eq!(4.0 / scalar, m1[(1, 2)]);
        assert_approx_eq!(1.0 / scalar, m1[(2, 0)]);
        assert_approx_eq!(-1.0 / scalar, m1[(2, 1)]);
        assert_approx_eq!(3.0 / scalar, m1[(2, 2)]);
    }

    #[test]
    fn test_mul() {
        // Arrange
        let m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let scalar = 15.0;

        // Act
        let result = scalar * &m1;

        // Assert
        assert_approx_eq!(1.0 * scalar, result[(0, 0)]);
        assert_approx_eq!(4.0 * scalar, result[(1, 2)]);
        assert_approx_eq!(1.0 * scalar, result[(2, 0)]);
        assert_approx_eq!(-1.0 * scalar, result[(2, 1)]);
        assert_approx_eq!(3.0 * scalar, result[(2, 2)]);
    }
}
