use std::ops::Index;
use std::ops::IndexMut;

use assert_approx_eq::assert_approx_eq;

use crate::la::ops;

#[derive(Clone, Debug)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn new(size: usize) -> Vector {
        Vector {
            data: vec![0.0; size],
        }
    }

    pub fn dim(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter {
        Iter { v: self, pos: 0 }
    }

    pub fn norm(&self) -> f64 {
        let output: f64 = self.data.iter().map(|&x| x * x).sum();
        output.sqrt()
    }

    pub fn hadamard(&self, v: &Vector) -> Vector {
        ops::hadamard(self, v)
    }
}

pub struct Iter<'a> {
    v: &'a Vector,
    pos: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos <= self.v.dim() - 1 {
            true => {
                let value = &self.v[self.pos];
                self.pos = self.pos + 1;
                Some(value)
            }
            _ => None,
        }
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

impl std::ops::Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim(), "Vectors must have same dimension");
        let output: Vec<_> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&x1, &x2)| x1 + x2)
            .collect();
        output.into()
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim(), "Vectors must have same dimension");
        let output: Vec<_> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&x1, &x2)| x1 - x2)
            .collect();
        output.into()
    }
}

impl std::ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let output: Vec<_> = self
            .data
            .iter()
            .map(|&x| x / rhs)
            .collect();
        output.into()
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

    #[test]
    fn test_iterator() {
        // Arrange
        let vec: Vector = vec![1.0, 2.0, 3.0, 4.0].into();

        // Act
        let result: Vec<_> = vec.iter().map(|x| x * x).collect();

        // Assert
        assert_eq!(vec![1.0, 4.0, 9.0, 16.0], result)
    }

    #[test]
    fn test_vector_add() {
        // Arrange
        let vec1: Vector = vec![1.0, 2.0, 3.0, 4.0].into();
        let vec2: Vector = vec![1.1, 2.2, 3.3, 4.4].into();

        // Act
        let result = &vec1 + &vec2;

        // Assert
        assert_eq!(4, result.dim());
        assert_eq!(1.0 + 1.1, result[0]);
        assert_eq!(2.0 + 2.2, result[1]);
        assert_eq!(3.0 + 3.3, result[2]);
        assert_eq!(4.0 + 4.4, result[3]);
    }

    #[test]
    fn test_vector_sub() {
        // Arrange
        let vec1: Vector = vec![1.0, 2.0, 3.0, 4.0].into();
        let vec2: Vector = vec![1.1, 2.2, 3.3, 4.4].into();

        // Act
        let result = &vec1 - &vec2;

        // Assert
        assert_eq!(4, result.dim());
        assert_eq!(1.0 - 1.1, result[0]);
        assert_eq!(2.0 - 2.2, result[1]);
        assert_eq!(3.0 - 3.3, result[2]);
        assert_eq!(4.0 - 4.4, result[3]);
    }

    #[test]
    fn test_vector_div() {
        // Arrange
        let v: Vector = vec![1.1, 2.2, 3.3, 4.4].into();

        // Act
        let result = &v / 0.01;

        // Assert
        assert_eq!(4, result.dim());
        assert_approx_eq!(1.1 / 0.01, result[0], 1e-3f64);
        assert_approx_eq!(2.2 / 0.01, result[1], 1e-3f64);
        assert_approx_eq!(3.3 / 0.01, result[2], 1e-3f64);
        assert_approx_eq!(4.4 / 0.01, result[3], 1e-3f64);
    }

    #[test]
    fn test_vector_norm() {
        // Arrange
        let vec1: Vector = vec![1.0, 2.0, 3.0, 4.0].into();

        // Act
        let result = vec1.norm();

        // Assert
        assert_eq!(30f64.sqrt(), result)
    }

    #[test]
    fn test_hadamard() {
        // Arrange
        let vec1: Vector = vec![1.0, 2.0, 3.0, 4.0].into();
        let vec2: Vector = vec![1.1, 2.2, 3.3, 4.4].into();

        // Act
        let result = vec1.hadamard(&vec2);

        // Assert
        assert_eq!(4, result.dim());
        assert_eq!(1.0 * 1.1, result[0]);
        assert_eq!(2.0 * 2.2, result[1]);
        assert_eq!(3.0 * 3.3, result[2]);
        assert_eq!(4.0 * 4.4, result[3]);
    }
}
