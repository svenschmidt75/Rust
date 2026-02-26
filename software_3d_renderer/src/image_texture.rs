use crate::color::Color;

#[derive(Debug)]
pub struct ImageTexture {
    width: u32,
    height: u32,
    image_data: Vec<u8>,
}

impl ImageTexture {
    pub fn new(width: u32, height: u32, image_data: &[u8]) -> Self {
        Self {
            width,
            height,
            image_data: image_data.to_vec(),
        }
    }

    pub fn get_pixel(&self, u: f32, v: f32) -> Color {
        assert!(u >= 0.0 && u <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);
        Color::new(255, 0, 0, 255)
    }
}
