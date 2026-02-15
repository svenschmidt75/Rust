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
    projection_matrix: Matrix4,
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
            projection_matrix: Matrix4::identity(),
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn orthographic(&mut self, l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) {
        // SS: map a certain region of camera space (orthographic view volume) into
        // the canonical view volume, the unit cube centered at the origin.
        // Fundamentals of Computer Graphics, 5th edition, equation (8.3)
        let mut projection_matrix = Matrix4::new();

        // SS: scaling component
        projection_matrix[0][0] = 2.0 / (r - l);
        projection_matrix[1][1] = 2.0 / (t - b);
        projection_matrix[2][2] = 2.0 / (n - f);
        projection_matrix[3][3] = 1.0;

        // SS: translation component
        projection_matrix[0][3] = -(r + l) / (r - l);
        projection_matrix[1][3] = -(t + b) / (t - b);
        projection_matrix[2][3] = -(n + f) / (n - f);

        self.projection_matrix = projection_matrix;
    }

    pub fn perspective(&mut self, l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) {
        // SS: map a certain region of camera space (view frustum) into
        // the canonical view volume, the unit cube centered at the origin.
        // Fundamentals of Computer Graphics, 5th edition, equation (8.3)
        let mut projection_matrix = Matrix4::new();

        // SS: scaling component
        projection_matrix[0][0] = 2.0 * n / (r - l);
        projection_matrix[1][1] = 2.0 * n / (t - b);
        projection_matrix[2][3] = 2.0 * f * n / (f - n);
        projection_matrix[3][2] = -1.0;

        // SS: translation component
        //        projection_matrix[0][2] = - (r + l) / (r - l);
        //        projection_matrix[1][2] = - (t + b) / (t - b);
        // SS: set to 0 due to symmetry (l = -r and b = -t)
        projection_matrix[0][2] = 0.0;
        projection_matrix[1][2] = 0.0;

        projection_matrix[2][2] = (n + f) / (n - f);

        self.projection_matrix = projection_matrix;
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
                // SS: move to camera space
                let camera_space_vertex = self.camera.world_to_camera(*v);

                // SS: move to view frustum ([l,r] x [b,t] x [n,f])
                let projection_space_vertex = self.projection_matrix * camera_space_vertex;

                // SS: perspective divide (into NDC: -1 to 1)
                let x = projection_space_vertex[0];
                let y = projection_space_vertex[1];
                let z = projection_space_vertex[2];
                let w = projection_space_vertex[3];
                let ndc = Vertex4::new_vertex(x / w, y / w, z / w);

                // SS: map to viewport (pixels on screen)
                let viewport_space_vertex = self.viewport_matrix * ndc;
                viewport_space_vertex
            })
            .map(|v| {
                // SS: the viewport transform maps to the screen pixel coordinates
                // (- width / 2,   height / 2)   -- (width / 2,   height / 2)
                // (- width / 2, - height / 2) -- (width / 2, - height / 2),
                // but we need to return render window coordinates with origin
                // in the top-left corner.
                let screen_x = v[0];
                let screen_y = self.height as f32 - v[1];
                [screen_x, screen_y]
            })
            .collect()
    }
}
