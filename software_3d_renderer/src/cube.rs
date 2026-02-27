use crate::matrix4::Matrix4;
use crate::raster_vertex::RasterVertex;
use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::texture_type::{Color, TextureType};
use crate::triangle::Triangle;
use crate::vertex4::Vertex4;

#[derive(Debug, Copy, Clone)]
pub struct UnitCube {
    triangles: [Triangle; 12],
}

impl UnitCube {
    pub fn new_with_image(texture_id: u32) -> Self {
        Self {
            triangles: [
                // SS: z = +1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                // SS: x = +1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                // SS: y = +1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                // SS: z = -1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                // SS: x = -1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, 1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                // SS: y = -1 face
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
                Triangle::new(
                    [
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 0.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(1f32, -1f32, 1f32),
                            Color::new(0, 0, 0, 255),
                            [1.0, 1.0],
                        ),
                        RasterVertex::new(
                            Vertex4::new_vertex(-1f32, -1f32, -1f32),
                            Color::new(0, 0, 0, 255),
                            [0.0, 0.0],
                        ),
                    ],
                    TextureType::Image(texture_id),
                ),
            ],
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
