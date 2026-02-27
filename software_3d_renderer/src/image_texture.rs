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
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        // SS: convert texture coordinates (u, v) to image coordinates (x, y)
        let x = (u * (self.width as f32 - 1.0)).round() as usize;
        let mut y = (v * (self.height as f32 - 1.0)).round() as usize;

        // SS: texture origin is bottom-left, image origin is top-left
//        y = (self.height as usize - 1) - y;

        let pixel_index = y * self.width as usize + x;
        let byte_offset = pixel_index * 4;

        // SS: bounds check just in case of float precision edge cases
        if byte_offset + 2 < self.image_data.len() {
            let r = self.image_data[byte_offset];
            let g = self.image_data[byte_offset + 1];
            let b = self.image_data[byte_offset + 2];
            let a = self.image_data[byte_offset + 3];
            Color::new(r, g, b, a)
        } else {
            // SS: magenta error color
            Color::new(255, 0, 255, 255)
        }
    }
}
