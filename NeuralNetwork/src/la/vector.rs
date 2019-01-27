use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone, Debug)]
pub struct Vector {
    data: Vec<f64>
}

impl Vector {

    pub fn new(size: usize) -> Vector {
        Vector { data: vec![0.0; size] }
    }

    pub fn dim(&self) -> usize {
        self.data.len()
    }

}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.data.len() {
            panic!("Index {} too large", index)
        }
        &self.data[index]
    }

}

impl From<Vec<f64>> for Vector {

    fn from(data: Vec<f64>) -> Self {
        Vector { data }
    }
}

impl IndexMut<usize> for Vector {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.data.len() {
            panic!("Index {} too large", index)
        }
        &mut self.data[index]
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        // Arrange
        let vec: Vector = vec![1.0, 2.0, 3.0, 4.0].into();

        // Act
        let lv = vec[0];
        let uv = vec[3];

        // Assert
        assert_eq!(1.0, lv);
        assert_eq!(4.0, uv);
    }

    #[test]
    fn test_indexmut() {
        // Arrange
        let mut vec: Vector = Vector::new(10);

        // Act
        vec[0] = 1.0;
        vec[9] = 2.0;

        // Assert
        assert_eq!(1.0, vec[0]);
        assert_eq!(2.0, vec[9])
    }

    #[test]
    fn test_dim() {
        // Arrange
        let vec: Vector = Vector::new(10);

        // Act
        let dim = vec.dim();

        // Assert
        assert_eq!(10, dim)
    }

}
