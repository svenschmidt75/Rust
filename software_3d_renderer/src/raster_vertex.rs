use crate::texture_type::Color;
use crate::vertex4::Vertex4;

#[derive(Debug, Clone, Copy)]
pub struct RasterVertex {
    pub vertex: Vertex4,
    pub color: Color,
    pub tex_coords: [f32; 2],
}

impl RasterVertex {
    pub fn new(vertex: Vertex4, color: Color, tex_coords: [f32; 2]) -> Self {
        Self {
            vertex,
            color,
            tex_coords,
        }
    }
}
