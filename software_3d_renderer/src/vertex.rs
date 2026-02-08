pub struct Vertex {
    position: [f32; 4],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            position: [x, y, z, w],
        }
    }
}
