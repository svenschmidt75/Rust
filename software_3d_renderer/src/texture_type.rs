pub(crate) use crate::color::Color;

#[derive(Debug, Copy, Clone)]
pub enum TextureType {
    None,
    Solid(Color),
    Image(u32),
}
