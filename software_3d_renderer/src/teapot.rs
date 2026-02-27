use crate::matrix4::Matrix4;
use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::triangle::Triangle;

#[derive(Debug)]
pub struct Teapot {
    triangles: Vec<Triangle>,
}

impl Teapot {
    pub fn new(triangles: &[Triangle]) -> Self {
        Teapot {
            triangles: triangles.to_vec(),
        }
    }
}

impl Renderable for Teapot {
    fn render(&self, ctx: &mut RenderContext, transform: Matrix4) {
        for triangle in self.triangles.iter() {
            triangle.render(ctx, transform);
        }
    }
}
