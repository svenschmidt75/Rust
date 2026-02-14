use crate::camera::Camera;
use crate::matrix4::Matrix4;
use crate::vertex;
use crate::vertex::Vertex4;

#[derive(Debug)]
pub struct RenderContext {
    pub framebuffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    camera: Camera,
    viewport_matrix: Matrix4,
}

impl RenderContext {
    pub fn new(width: u32, height: u32) -> Self {
        // SS: create the transformation of the unit cube into screen space
        let mut viewport_matrix = Matrix4::new();
        viewport_matrix[0][0] = width as f32 / 2.0;
        viewport_matrix[1][1] = height as f32 / 2.0;
        viewport_matrix[2][2] = 1.0;
        viewport_matrix[3][3] = 1.0;
        viewport_matrix[0][3] = (width - 1) as f32 / 2.0;
        viewport_matrix[1][3] = (height - 1) as f32 / 2.0;

        RenderContext {
            framebuffer: vec![0; (width * height * 4) as usize],
            width,
            height,
            camera: Camera::new(
                Vertex4::new_vertex(0.0, 0.0, 5.0),
                Vertex4::new_vector(0.0, 0.0, -1.0),
                Vertex4::new_vector(0.0, 1.0, 0.0),
            ),
            viewport_matrix,
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;

        // SS: rerender scene
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let idx = (y * self.width + x) as usize;
        let idx2 = idx * 4;

        if idx2 >= (self.width * self.height * 4) as usize {
            println!("Invalid index {}", idx2);
            return;
        }

        self.framebuffer[idx2] = r;
        self.framebuffer[idx2 + 1] = g;
        self.framebuffer[idx2 + 2] = b;
        self.framebuffer[idx2 + 3] = a;
    }

    pub fn world_to_screen(&self, world_vertices: &[vertex::Vertex4]) -> Vec<[f32; 2]> {
        // SS: transform world to camera space
        world_vertices
            .iter()
            .map(|v| {
                let camera_space_vertex = self.camera.world_to_camera(*v);
                let viewport_space_vertex = self.viewport_matrix * camera_space_vertex;
                viewport_space_vertex
            })
            .map(|v| {
                // SS: the viewport transform maps to the screen pixel coordinates
                // (- width / 2,   height / 2)   -- (width / 2,   height / 2)
                // (- width / 2, - height / 2) -- (width / 2, - height / 2),
                // but we need to return render window coordinates with origin
                // in the top-left corner.
                [
                    v[0] + (self.width / 2) as f32,
                    (self.height / 2) as f32 - v[1],
                ]
            })
            .collect()
    }
}
