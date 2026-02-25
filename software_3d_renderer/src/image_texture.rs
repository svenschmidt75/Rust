use crate::texture::{Color, Texture};

pub struct ImageTexture<'a> {
    width: u32,
    height: u32,
    image_data: &'a [u8],
}

impl<'a> ImageTexture<'a> {
    pub fn new(width: u32, height: u32, image_data: &'a [u8]) -> Self {
        Self { width, height, image_data }
    }
}

impl<'a> Texture for ImageTexture<'a> {
    fn get_color(&self, u: f32, v: f32) -> Color {
        todo!()
    }
}
