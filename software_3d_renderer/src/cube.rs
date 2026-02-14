use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::triangle::Triangle;
use crate::vertex::Vertex4;

#[derive(Debug, Clone, Copy)]
pub struct UnitCube {
    triangles: [Triangle; 2],
}

impl UnitCube {
    pub fn new() -> Self {
        Self {
            triangles: [
                // SS: z = +1 face
                Triangle::new([
                    Vertex4::new_vertex(-1f32, -1f32, 0.5f32),
                    Vertex4::new_vertex(1f32, -1f32, 0.5f32),
                    Vertex4::new_vertex(-1f32, 1f32, 0.5f32),
                ]),
                Triangle::new([
                    Vertex4::new_vertex(-1f32, 1f32, 0.5f32),
                    Vertex4::new_vertex(1f32, -1f32, 0.5f32),
                    Vertex4::new_vertex(1f32, 1f32, 0.5f32),
                ]),
            ],
        }
    }
}

impl Renderable for UnitCube {
    fn render(&self, ctx: &mut RenderContext) {
        for triangle in self.triangles.iter() {
            triangle.render(ctx);
        }
    }
}
