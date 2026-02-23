use crate::texture::{Color, Texture};

pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    // SS: image data
}

impl ImageTexture {
    pub fn new(width: u32, height: u32) -> ImageTexture {
        Self { width, height }
    }
}

impl Texture for ImageTexture {
    fn get_color(&self, u: f32, v: f32) -> Color {
        todo!()
    }
}
