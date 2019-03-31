use crate::la::ops;
use crate::la::vector::Vector;

pub trait Matrix {
    fn transpose<'a>(&'a self) -> Box<dyn Matrix + 'a>;
    fn ncols(&self) -> usize;
    fn nrows(&self) -> usize;
    fn get(&self, row: usize, col: usize) -> f64;
    fn ax(&self, x: &Vector) -> Vector;
}

pub struct Transpose<'a> {
    matrix: &'a Matrix,
}

impl<'a> Transpose<'a> {
    pub fn new(matrix: &Matrix) -> Transpose {
        Transpose { matrix }
    }
}

impl<'a> Matrix for Transpose<'a> {
    fn transpose<'b>(&'b self) -> Box<dyn Matrix + 'b> {
        Box::new(Transpose::new(self))
    }

    fn ncols(&self) -> usize {
        self.matrix.nrows()
    }

    fn nrows(&self) -> usize {
        self.matrix.ncols()
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.matrix.get(col, row)
    }

    fn ax(&self, x: &Vector) -> Vector {
        ops::ax(self, x)
    }
}

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

    pub fn get(&self, row: usize, col: usize) -> f64 {
        let linear_index = self.linear_index(row, col);
        self.data[linear_index]
    }

    pub fn set(&mut self, row: usize, col: usize) -> &mut f64 {
        let linear_index = self.linear_index(row, col);
        &mut self.data[linear_index]
    }
}

impl<'a> Matrix for Matrix2D {
    fn transpose<'b>(&'b self) -> Box<dyn Matrix + 'b> {
        Box::new(Transpose::new(self))
    }

    fn ncols(&self) -> usize {
        self.ncols
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        (self as &Matrix2D).get(row, col)
    }

    fn ax(&self, x: &Vector) -> Vector {
        ops::ax(self, x)
    }
}

impl std::ops::AddAssign<&Matrix2D> for Matrix2D {
    fn add_assign(&mut self, other: &Self) {
        assert_eq!(self.nrows(), other.nrows(), "Matrices must have same number of rows");
        assert_eq!(self.ncols(), other.ncols(), "Matrices must have same number of columns");
        for idx in 0..self.data.len() {
            let v1 = self.data[idx];
            let v2 = other.data[idx];
            self.data[idx] = v1 + v2;
        }
    }
}

impl std::ops::DivAssign<usize> for Matrix2D {
    fn div_assign(&mut self, rhs: usize) {
        // SS: collect to force evaluation as lazy...
        self.data.iter_mut().for_each(|v| *v /= rhs as f64);
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
        *m.set(0, 0) = 1.0;
        *m.set(0, 1) = 2.0;
        *m.set(0, 2) = 3.0;
        *m.set(1, 0) = 4.0;
        *m.set(1, 1) = 5.0;
        *m.set(1, 2) = 6.0;

        // Act
        let tr = m.transpose();

        // Assert
        assert_eq!(3, tr.nrows());
        assert_eq!(2, tr.ncols());
        assert_approx_eq!(1.0, tr.get(0, 0), 1e-3f64);
        assert_approx_eq!(4.0, tr.get(0, 1), 1e-3f64);
        assert_approx_eq!(2.0, tr.get(1, 0), 1e-3f64);
        assert_approx_eq!(5.0, tr.get(1, 1), 1e-3f64);
        assert_approx_eq!(3.0, tr.get(2, 0), 1e-3f64);
        assert_approx_eq!(6.0, tr.get(2, 1), 1e-3f64);
    }

    #[test]
    fn test_two_transpose() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        // 1.0 | 2.0 | 3.0
        // 4.0 | 5.0 | 6.0
        *m.set(0, 0) = 1.0;
        *m.set(0, 1) = 2.0;
        *m.set(0, 2) = 3.0;
        *m.set(1, 0) = 4.0;
        *m.set(1, 1) = 5.0;
        *m.set(1, 2) = 6.0;

        // Act
        let tr = m.transpose();
        let tr2 = tr.transpose();

        // Assert
        assert_eq!(2, tr2.nrows());
        assert_eq!(3, tr2.ncols());
        assert_approx_eq!(1.0, tr2.get(0, 0), 1e-3f64);
        assert_approx_eq!(2.0, tr2.get(0, 1), 1e-3f64);
        assert_approx_eq!(3.0, tr2.get(0, 2), 1e-3f64);
        assert_approx_eq!(4.0, tr2.get(1, 0), 1e-3f64);
        assert_approx_eq!(5.0, tr2.get(1, 1), 1e-3f64);
        assert_approx_eq!(6.0, tr2.get(1, 2), 1e-3f64);
    }

    #[test]
    fn test_nrows() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

        // Act
        let nrows = m.nrows();

        // Assert
        assert_eq!(2, nrows)
    }

    #[test]
    fn test_ncols() {
        // Arrange
        let mut m = Matrix2D::new(2, 3);

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
        *m.set(0, 0) = 1.0;
        *m.set(0, 1) = 2.0;
        *m.set(0, 2) = 3.0;
        *m.set(1, 0) = 4.0;
        *m.set(1, 1) = 5.0;
        *m.set(1, 2) = 6.0;

        // Assert
        assert_eq!(3.0, m.get(0, 2));
        assert_eq!(5.0, m.get(1, 1))
    }

    #[test]
    fn test_addassign() {
        // Arrange
        let mut m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let mut m2 = Matrix2D::new_from_data(3, 3, vec![7.0, 2.0, 9.0, 1.0, 2.0, 5.0, 0.0, 2.0, 4.0]);

        // Act
        m1 += &m2;

        // Assert
        assert_approx_eq!(8.0, m1.get(0, 0));
        assert_approx_eq!(9.0, m1.get(1, 2));
        assert_approx_eq!(1.0, m1.get(2, 0));
        assert_approx_eq!(1.0, m1.get(2, 1));
        assert_approx_eq!(7.0, m1.get(2, 2));
    }

    #[test]
    fn test_divassign() {
        // Arrange
        let mut m1 = Matrix2D::new_from_data(3, 3, vec![1.0, 5.0, -1.0, 11.0, 3.0, 4.0, 1.0, -1.0, 3.0]);
        let scalar = 15.0;

        // Act
        m1 /= scalar as usize;

        // Assert
        assert_approx_eq!(1.0 / scalar, m1.get(0, 0));
        assert_approx_eq!(4.0 / scalar, m1.get(1, 2));
        assert_approx_eq!(1.0 / scalar, m1.get(2, 0));
        assert_approx_eq!(-1.0 / scalar, m1.get(2, 1));
        assert_approx_eq!(3.0 / scalar, m1.get(2, 2));
    }

}
