use crate::image_texture::ImageTexture;

#[derive(Debug)]
pub struct TextureManager {
    textures: Vec<ImageTexture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture: ImageTexture) -> u32 {
        self.textures.push(texture);
        (self.textures.len() - 1) as u32
    }

    pub fn get_texture(&self, texture_id: u32) -> &ImageTexture {
        &self.textures[texture_id as usize]
    }

}
