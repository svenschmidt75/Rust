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

    pub fn iter(&self) -> Iter {
        Iter { v: self, pos: 0 }
    }
}

pub struct Iter<'a> {
    v: &'a Vector,
    pos: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos <= self.v.dim() - 1 {
            true => {
                let value = self.v[self.pos];
                self.pos = self.pos + 1;
                Some(value)
            },
            _ => None
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
        let output: Vec<_> = self.data.iter().zip(rhs.data.iter()).map(|(&x1, &x2)| x1 + x2).collect();
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
        let result: Vec<_> = vec.iter().map(|x| x*x).collect();

        // Assert
        assert_eq!(vec![1.0, 4.0, 9.0, 16.0], result)
    }

}
