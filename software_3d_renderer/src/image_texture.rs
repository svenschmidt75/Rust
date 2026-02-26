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
}
