use crate::vertex::Vertex4;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();

        // SS: perform matrix multiplication
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        result
    }
}

impl Mul<Vertex4> for Matrix4 {
    type Output = Vertex4;

    fn mul(self, rhs: Vertex4) -> Self::Output {
        Vertex4::new_vector(
            self[0][0] * rhs[0] + self[0][1] * rhs[1] + self[0][2] * rhs[2],
            self[1][0] * rhs[0] + self[1][1] * rhs[1] + self[1][2] * rhs[2],
            self[2][0] * rhs[0] + self[2][1] * rhs[1] + self[2][2] * rhs[2],
        )
    }
}
