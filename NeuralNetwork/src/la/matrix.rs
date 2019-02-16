pub trait Matrix {
    fn transpose<'a>(&'a self) -> Box<dyn Matrix + 'a>;
    fn ncols(&self) -> usize;
    fn nrows(&self) -> usize;
    fn get(&self, row: usize, col: usize) -> f64;
    fn set(&mut self, row: usize, col: usize) -> &mut f64;
}

pub struct Transpose<'a> {
    matrix: &'a Matrix2D
}

impl<'a> Transpose<'a> {

    pub fn new(matrix: &Matrix2D) -> Transpose {
        Transpose { matrix }
    }

}

impl<'a> Matrix for Transpose<'a> {

    fn transpose<'b>(&'b self) -> Box<dyn Matrix + 'b> {
        Box::new(Transpose::new(self.matrix))
    }

    fn ncols(&self) -> usize {
        unimplemented!()
    }

    fn nrows(&self) -> usize {
        unimplemented!()
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        unimplemented!()
    }

    fn set(&mut self, row: usize, col: usize) -> &mut f64 {
        unimplemented!()
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
        assert_eq!(
            nrows * ncols,
            data.len(),
            "Not enough data provided for matrix initialization"
        );
        Matrix2D { data, nrows, ncols }
    }

    fn linear_index(&self, row: usize, col: usize) -> usize {
        // row-major memory layout
        let linear_index = row * self.ncols + col;
        assert!(
            linear_index < self.data.len(),
            "Matrix.linear_index: Index too large"
        );
        linear_index
    }

}

impl<'a> Matrix for Matrix2D {

    fn transpose(&self) -> Box<dyn Matrix> {
        unimplemented!()
    }

    fn ncols(&self) -> usize {
        self.ncols
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        let linear_index = self.linear_index(row, col);
        self.data[linear_index]
    }

    fn set(&mut self, row: usize, col: usize) -> &mut f64 {
        let linear_index = self.linear_index(row, col);
        &mut self.data[linear_index]
    }

}


#[cfg(test)]
mod tests {
    use super::*;

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

}
