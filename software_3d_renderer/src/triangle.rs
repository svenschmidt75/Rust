use crate::color::Color;
use crate::lin_alg::{cross_product, dot_product};
use crate::matrix4::Matrix4;
use crate::raster_vertex::RasterVertex;
use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::texture_type::TextureType;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    vertices: [RasterVertex; 3],
    texture: TextureType,
}

impl Triangle {
    pub fn new(vertices: [RasterVertex; 3], texture: TextureType) -> Self {
        // SS: Triangle vertices must be oriented. This is because when rasterizing the triangle,
        // we calculate signed areas w.r.t. to points inside the triangle. If they are not oriented,
        // rasterization will fail!
        Triangle { vertices, texture }
    }
}

impl Renderable for Triangle {
    fn render(&self, ctx: &mut RenderContext, transform: Matrix4) {
        // SS: approach described in Fundamentals of Computer Graphics, 5th edition
        // 9.1.2 Triangle Rasterization
        // We scan left-right, top-bottom  the bounding box of the triangle. For each point,
        // we calculate the signed area (edge_function) to check whether the point is inside
        // the triangle. We calculate barycentric coordinates for interpolation purposes
        // (color, texture, ...)
        // Interpolation of linear properties like texture coordinates etc.: When we rasterize,
        // we work in screen space. Say we want to texture a triangle with an image. Each vertex
        // contains the texture coordinates (u,v) (0 <= u,v <= 1) on the image. We need to calculate
        // (u,v) in the triangle coordinates. We cannot simply do so in screen space due to the
        // perspective divide. The key is that we linearly interpolate u/w, v/w. Since we need
        // (u,v), we have to divide by 1/w (which is 1/z). So we have to linearly interpolate 1/w
        // using barycentric coordinates and also u/w and v/w. This gives perspective correct
        // interpolation.
        // The magic that makes this happen is the fact that 1/z is linear in screen space. To see
        // this, we look at what the perspective projection does to the z component of a vertex:
        // (f+n)/(n-f)*z + 2*f*n/(n-f) = z'
        // After homogenization, i.e. the perspective divide with 1/w=1/z, we end up with
        // (f+n)/(n-f) + 2*f*n/(n-f)*1/z = z'/z, which is linear in 1/z!

        // SS: apply transformations to triangle vertices
        let transformed_vertices = self.vertices.map(|t| transform * t.vertex);

        // SS: transform the vertices in world space to camera space and calculate the triangle
        // normal in camera space.
        let v0 = ctx.world_to_camera(transformed_vertices[0]);
        let v1 = ctx.world_to_camera(transformed_vertices[1]);
        let v2 = ctx.world_to_camera(transformed_vertices[2]);
        let mut normal = cross_product(v1 - v0, v2 - v0).normalized();

        // SS: we have to render triangles that are oriented both clockwise and counter-clockwise,
        // otherwise we end up with holes in the rendering when the triangle vertices are ordered
        // clockwise instead of counter-clockwise.
        if normal[2] < 0.0 {
            // SS: triangle is oriented clockwise, flip the normal
            normal = -normal;
        }

        // SS: the dot product between light direction and viewing direction is the light intensity,
        // a value between 0 and 1. We use this to do simple diffuse shading.
        let light_source = ctx.get_light_source();
        let light_position_camera_space = ctx.world_to_camera(light_source.position);
        let light_direction = -(light_position_camera_space - v0).normalized();
        let intensity = light_source.get_intensity(dot_product(normal, light_direction).max(0.0));

        let screen_vertices = ctx.world_to_screen(&transformed_vertices);

        // SS: triangle vertices are u, v, w
        let v0 = screen_vertices[0];
        let v1 = screen_vertices[1];
        let v2 = screen_vertices[2];

        // SS: bounding box of triangle
        let min_x = screen_vertices
            .iter()
            .map(|v| v[0])
            .reduce(|a, b| a.min(b))
            .unwrap_or(f32::INFINITY)
            .floor()
            .max(0.0) as u32;
        let max_x = screen_vertices
            .iter()
            .map(|v| v[0])
            .reduce(|a, b| a.max(b))
            .unwrap_or(f32::NEG_INFINITY)
            .ceil()
            .max(0.0) as u32;
        let min_y = screen_vertices
            .iter()
            .map(|v| v[1])
            .reduce(|a, b| a.min(b))
            .unwrap_or(f32::INFINITY)
            .floor()
            .max(0.0) as u32;
        let max_y = screen_vertices
            .iter()
            .map(|v| v[1])
            .reduce(|a, b| a.max(b))
            .unwrap_or(f32::NEG_INFINITY)
            .ceil()
            .max(0.0) as u32;

        // SS: calculate barycentric coordinates

        // SS: start point on bounding box
        let p0 = [min_x as f32, min_y as f32];

        // SS: initialize the edge function
        let (mut w0_row, w0_dx, w0_dy) = edge_function(v1, v2, p0);
        let (mut w1_row, w1_dx, w1_dy) = edge_function(v2, v0, p0);
        let (mut w2_row, w2_dx, w2_dy) = edge_function(v0, v1, p0);

        // SS: calculate the triangle area
        let area_doubled = w0_row + w1_row + w2_row;

        // SS: check for degenerate triangles (a line or point)
        if area_doubled.abs() < f32::EPSILON {
            return;
        }

        let inv_area = 1.0 / area_doubled;

        // SS: vertex colors
        let c0 = self.vertices[0].color;
        let c1 = self.vertices[1].color;
        let c2 = self.vertices[2].color;

        // SS: linear interpolation for 1/z
        let one_over_z0 = 1.0 / screen_vertices[0][3];
        let one_over_z1 = 1.0 / screen_vertices[1][3];
        let one_over_z2 = 1.0 / screen_vertices[2][3];

        // SS: linear interpolation for (u/z, v/z)
        let one_over_u0 = self.vertices[0].tex_coords[0] * one_over_z0;
        let one_over_v0 = self.vertices[0].tex_coords[1] * one_over_z0;

        let one_over_u1 = self.vertices[1].tex_coords[0] * one_over_z1;
        let one_over_v1 = self.vertices[1].tex_coords[1] * one_over_z1;

        let one_over_u2 = self.vertices[2].tex_coords[0] * one_over_z2;
        let one_over_v2 = self.vertices[2].tex_coords[1] * one_over_z2;

        // SS: scan the entire triangle bounding box
        for y in min_y..max_y {
            // SS: current edge function values for row
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..max_x {
                // SS: check if point is inside, regardless of CW or CCW orientation
                // so we don't end up with holes in the triangle when vertices are
                // ordered CW instead of CCW
                let is_inside = if area_doubled > 0.0 {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if is_inside {
                    // SS: calculate the barycentric coordinates for interpolation
                    // Fundamentals of Computer Graphics, 5th edition, equation (2.33)
                    let alpha = w0 * inv_area;
                    let beta = w1 * inv_area;
                    let gamma = w2 * inv_area;

                    // SS: linearly interpolate 1/z
                    let one_over_z = alpha * one_over_z0 + beta * one_over_z1 + gamma * one_over_z2;

                    // SS: compare with z buffer here and skip if pixel is occluded
                    if !ctx.compare_with_z_buffer(x, y, one_over_z) {
                        continue;
                    }

                    let cr = alpha * c0.r as f32 + beta * c1.r as f32 + gamma * c2.r as f32;
                    let cg = alpha * c0.g as f32 + beta * c1.g as f32 + gamma * c2.g as f32;
                    let cb = alpha * c0.b as f32 + beta * c1.b as f32 + gamma * c2.b as f32;

                    // SS: determine color
                    let (r, g, b, a) = match self.texture {
                        TextureType::None => {
                            // SS: use interpolated vertex colors
                            (cr, cg, cb, 255)
                        }
                        TextureType::Solid(color) => {
                            // SS: blend vertex colors with solid texture color
                            (
                                (color.r as f32 + cr) / 2.0,
                                (color.g as f32 + cg) / 2.0,
                                (color.b as f32 + cb) / 2.0,
                                255,
                            )
                        }
                        TextureType::Image(id) => {
                            let one_over_u =
                                alpha * one_over_u0 + beta * one_over_u1 + gamma * one_over_u2;
                            let one_over_v =
                                alpha * one_over_v0 + beta * one_over_v1 + gamma * one_over_v2;
                            let (u, v) = (one_over_u / one_over_z, one_over_v / one_over_z);

                            // SS: texture lookup
                            let image_texture = ctx.texture_manager.get_texture(id);
                            let Color { r, g, b, a } = image_texture.get_pixel(u, v);

                            // SS: blend texture color with vertex color
                            (
                                (cr + r as f32) / 2.0,
                                (cg + g as f32) / 2.0,
                                (cb + b as f32) / 2.0,
                                a,
                            )
                        }
                    };

                    // SS: apply light intensity
                    let r = (r * intensity).min(255.0);
                    let g = (g * intensity).min(255.0);
                    let b = (b * intensity).min(255.0);

                    ctx.set_pixel(x as u32, y as u32, r as u8, g as u8, b as u8, a as u8);
                }

                // SS: advance edge function values by x -> x + 1
                w0 += w0_dx;
                w1 += w1_dx;
                w2 += w2_dx;
            }

            // SS: advance edge function values by y -> y + 1
            w0_row += w0_dy;
            w1_row += w1_dy;
            w2_row += w2_dy;
        }
    }
}

fn edge_function(a: [f32; 4], b: [f32; 4], p: [f32; 2]) -> (f32, f32, f32) {
    /* Calculate E(p): Checks which side point p is on of edge (a, b).
     * Returns initial value of edge function and the changes of E when p is advanced
     * in x or y direction:
     * E(p + dx) = E(p) + dy
     * E(p + dy) = E(p) - dx
     * The edge function is actually twice the signed area of the triangle (a, b, p),
     * see Fundamentals of Computer Graphics, 5th edition, equation (2.27)
     */
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];

    // SS: E(p)
    let init = (p[0] - a[0]) * dy - (p[1] - a[1]) * dx;

    // dy: how much E changes when x += 1
    // -dx: how much E changes when y += 1
    (init, dy, -dx)
}
