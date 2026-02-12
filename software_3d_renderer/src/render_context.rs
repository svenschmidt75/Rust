use crate::camera::Camera;
use crate::vertex;
use crate::vertex::Vertex4;

#[derive(Debug)]
pub struct RenderContext {
    pub framebuffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    camera: Camera,
}

impl RenderContext {
    pub fn new(width: u32, height: u32) -> Self {
        RenderContext {
            framebuffer: vec![0; (width * height * 4) as usize],
            width,
            height,
            camera: Camera::new(
                Vertex4::new_vertex(0.0, 0.0, 5.0),
                Vertex4::new_vector(0.0, 0.0, -1.0),
                Vertex4::new_vector(0.0, 1.0, 0.0),
            ),
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;

        // SS: rerender scene
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let idx = (y * self.width + x) as usize;
        let idx2 = idx * 4;
        self.framebuffer[idx2] = r;
        self.framebuffer[idx2 + 1] = g;
        self.framebuffer[idx2 + 2] = b;
        self.framebuffer[idx2 + 3] = a;
    }

    pub fn world_to_screen(&self, world_vertices: &[vertex::Vertex4]) -> Vec<[f32; 2]> {
        vec![[100f32, 0f32], [0f32, 100f32], [100f32, 100f32]]
    }

}
