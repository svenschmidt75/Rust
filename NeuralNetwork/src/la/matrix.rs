pub struct Matrix {
    data: Vec<f64>,
    nrows: usize,
    ncols: usize,
}

impl Matrix {

    pub fn new(nrows: usize, ncols: usize) -> Matrix {
        Matrix { data: vec![0.0; ncols * nrows], nrows, ncols }
    }

    fn linear_index(&self, row: usize, col: usize) -> usize {
        // row-major memory layout
        let linear_index = row * self.ncols + col;
        if linear_index >= self.data.len() {
            panic!("Matrix.linear_index: Index too large")
        }
        linear_index
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        // Arrange
        let mut m = Matrix::new(2, 3);

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
