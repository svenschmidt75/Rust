use std::ops::{Index, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vertex4 {
    position: [f32; 4],
}

impl Vertex4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            position: [x, y, z, w],
        }
    }

    pub fn new_vertex(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn norm(&self) -> f32 {
        let dx = self.position[0] * self.position[0];
        let dy = self.position[1] * self.position[1];
        let dz = self.position[2] * self.position[2];
        let dw = self.position[3] * self.position[3];
        (dx + dy + dz + dw).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        let x = self.position[0] / norm;
        let y = self.position[1] / norm;
        let z = self.position[2] / norm;
        let w = self.position[3] / norm;
        Self::new(x, y, z, w)
    }

}

impl Index<usize> for Vertex4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.position[0],
            1 => &self.position[1],
            2 => &self.position[2],
            3 => &self.position[3],
            _ => panic!("Index out of bounds! Must be within 0 and 3."),
        }
    }
}

impl Neg for Vertex4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.position[0], -self.position[1], -self.position[2], -self.position[3])
    }

}