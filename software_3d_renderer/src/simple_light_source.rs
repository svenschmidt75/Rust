use crate::vertex4::Vertex4;

#[derive(Debug, Copy, Clone)]
pub struct SimpleLightSource {
    pub position: Vertex4,
    ambient: f32,
    multiplier: f32,
}

impl SimpleLightSource {
    pub fn new(position: Vertex4, ambient: f32, multiplier: f32) -> Self {
        Self {
            position,
            ambient,
            multiplier,
        }
    }

    pub fn get_intensity(&self, dot_product: f32) -> f32 {
        self.ambient + dot_product * self.multiplier
    }
}
