use crate::matrix4::Matrix4;
use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::triangle::Triangle;
use crate::vertex::Vertex4;

#[derive(Debug, Copy, Clone)]
pub struct UnitCube {
    triangles: [Triangle; 6],
    angle_per_second: f32,
}

impl UnitCube {
    pub fn new() -> Self {
        Self {
            triangles: [
                // SS: z = +1 face
                Triangle::new([
                    Vertex4::new_vertex(-1f32, -1f32, 1f32),
                    Vertex4::new_vertex(1f32, -1f32, 1f32),
                    Vertex4::new_vertex(-1f32, 1f32, 1f32),
                ]),
                Triangle::new([
                    Vertex4::new_vertex(-1f32, 1f32, 1f32),
                    Vertex4::new_vertex(1f32, -1f32, 1f32),
                    Vertex4::new_vertex(1f32, 1f32, 1f32),
                ]),
                // SS: x = +1 face
                Triangle::new([
                    Vertex4::new_vertex(1f32, 1f32, 1f32),
                    Vertex4::new_vertex(1f32, -1f32, 1f32),
                    Vertex4::new_vertex(1f32, 1f32, -1f32),
                ]),
                Triangle::new([
                    Vertex4::new_vertex(1f32, -1f32, 1f32),
                    Vertex4::new_vertex(1f32, -1f32, -1f32),
                    Vertex4::new_vertex(1f32, 1f32, -1f32),
                ]),
                // SS: y = +1 face
                Triangle::new([
                    Vertex4::new_vertex(-1f32, 1f32, 1f32),
                    Vertex4::new_vertex(1f32, 1f32, 1f32),
                    Vertex4::new_vertex(-1f32, 1f32, -1f32),
                ]),
                Triangle::new([
                    Vertex4::new_vertex(1f32, 1f32, 1f32),
                    Vertex4::new_vertex(1f32, 1f32, -1f32),
                    Vertex4::new_vertex(-1f32, 1f32, -1f32),
                ]),
            ],
            angle_per_second: 90.0,
        }
    }
}

impl Renderable for UnitCube {
    fn render(&self, ctx: &mut RenderContext, transform: Matrix4) {
        for triangle in self.triangles.iter() {
            triangle.render(ctx, transform);
        }
    }
}
