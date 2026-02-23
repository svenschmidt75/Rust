#[derive(Debug, Copy, Clone)]
pub struct Color(u8, u8, u8, u8);

pub trait Texture {
    fn get_color(&self, u: f32, v: f32) -> Color;
}
