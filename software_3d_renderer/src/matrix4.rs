use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    fn new() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

    fn identity() -> Self {
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
