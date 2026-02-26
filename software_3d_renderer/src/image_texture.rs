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

    pub fn get_pixel(&self, mut u: f32, mut v: f32) -> Color {
        // assert!(u >= 0.0 && u <= 1.0);
        // assert!(v >= 0.0 && v <= 1.0);
        if u < 0.0 {
            u = 0.0;
        } else if u > 1.0 {
            u = 1.0;
        }

        if v < 0.0 {
            v = 0.0;
        } else if v > 1.0 {
            v = 1.0;
        }

        // SS: convert texture coordinates (u, v) to image coordinates (x, y)
        let ix = u * (self.width as f32 - 1.0);
        let iy = v * (self.height as f32 - 1.0);

        // SS: texture origin is bottom-left, image origin is top-left
        let iy = (self.height - 1) as f32 - iy;

        let image_offset = (ix + (self.height as f32) * iy) as usize;
        let (r, g, b) = (
            self.image_data[image_offset],
            self.image_data[image_offset + 1],
            self.image_data[image_offset + 2],
        );
        Color::new(r, g, b, 255)
    }
}
