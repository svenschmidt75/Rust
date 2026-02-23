use crate::texture::{Color, Texture};

pub struct ColorTexture {
    color: Color,
}

impl Texture for ColorTexture {
    fn get_color(&self, u: f32, v: f32) -> Color {
        self.color
    }

}
