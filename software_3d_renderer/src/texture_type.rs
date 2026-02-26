#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Copy, Clone)]
pub enum TextureType {
    None,
    Solid(Color),
    Image(u32),
}
