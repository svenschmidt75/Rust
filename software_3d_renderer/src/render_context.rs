use sfml::graphics::Vertex;
use crate::vertex;

pub struct RenderContext {
    pub framebuffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl RenderContext {
    pub fn new(width: u32, height: u32) -> Self {
        RenderContext {
            framebuffer: vec![0; (width * height * 4) as usize],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let idx = (y * self.width + x) as usize;
        let idx2 = idx * 4;
        self.framebuffer[idx2] = r;
        self.framebuffer[idx2 + 1] = g;
        self.framebuffer[idx2 + 2] = b;
        self.framebuffer[idx2 + 3] = a;
    }

    pub fn world_to_screen(&self, world_vertices: &[vertex::Vertex]) -> Vec<[i32; 2]> {
        vec![[0, 100], [50, 0], [100, 0]]
    }

}
