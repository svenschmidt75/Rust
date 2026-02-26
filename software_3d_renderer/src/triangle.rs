use crate::lin_alg::cross_product;
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

        // SS: apply transformations to triangle vertices
        let transformed_vertices = self.vertices.map(|t| transform * t.vertex);

        // SS: do we transform the normal or do we recalculate the normal?
        let u1 = transformed_vertices[1] - transformed_vertices[0];
        let u2 = transformed_vertices[2] - transformed_vertices[0];
        let normal = cross_product(u1, u2);

        // SS: backface-culling
        let camera = ctx.get_camera();
        if !camera.is_visible(normal) {
            // SS: triangle not visible to camera
            return;
        }

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
            .floor() as i32;
        let max_x = screen_vertices
            .iter()
            .map(|v| v[0])
            .reduce(|a, b| a.max(b))
            .unwrap_or(f32::NEG_INFINITY)
            .ceil() as i32;
        let min_y = screen_vertices
            .iter()
            .map(|v| v[1])
            .reduce(|a, b| a.min(b))
            .unwrap_or(f32::INFINITY)
            .floor() as i32;
        let max_y = screen_vertices
            .iter()
            .map(|v| v[1])
            .reduce(|a, b| a.max(b))
            .unwrap_or(f32::NEG_INFINITY)
            .ceil() as i32;

        // SS: start point on bounding box
        let p0 = [min_x as f32, min_y as f32];

        // SS: initialize the edge function
        let (mut w0_row, w0_dx, w0_dy) = edge_function(v1, v2, p0);
        let (mut w1_row, w1_dx, w1_dy) = edge_function(v2, v0, p0);
        let (mut w2_row, w2_dx, w2_dy) = edge_function(v0, v1, p0);

        // SS: calculate the triangle area
        let area_doubled = w0_row + w1_row + w2_row;
        let inv_area = 1.0 / area_doubled;

        let c1 = match self.texture {
            TextureType::None => self.vertices[0].color,
            TextureType::Solid(color) => color,
            TextureType::Image(id) => self.vertices[0].color,
        };

        let c2 = match self.texture {
            TextureType::None => self.vertices[1].color,
            TextureType::Solid(color) => color,
            TextureType::Image(id) => self.vertices[1].color,
        };

        let c3 = match self.texture {
            TextureType::None => self.vertices[2].color,
            TextureType::Solid(color) => color,
            TextureType::Image(id) => self.vertices[2].color,
        };

        // SS: scan the entire triangle bounding box
        for y in min_y..max_y {
            // SS: current edge function values for row
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..max_x {
                // SS: if all edge function values are >=0, the current point (x, y) is inside
                // the triangle, so render it
                if (w0 >= 0.0) && (w1 >= 0.0) && (w2 >= 0.0) {
                    // SS: calculate the barycentric coordinates for interpolation
                    // Fundamentals of Computer Graphics, 5th edition, equation (2.33)
                    let alpha = w0 * inv_area;
                    let beta = w1 * inv_area;
                    let gamma = w2 * inv_area;

                    // texture.get_color(alpha, beta, gamma);

                    let cx =
                        alpha * c1.r as f32 + beta * c1.g as f32 + gamma * c1.b as f32;
                    let cy =
                        alpha * c2.r as f32 + beta * c2.g as f32 + gamma * c2.b as f32;
                    let cz =
                        alpha * c3.r as f32 + beta * c3.g as f32 + gamma * c3.b as f32;

                    ctx.set_pixel(x as u32, y as u32, cx as u8, cy as u8, cz as u8, 255);
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

fn edge_function(a: [f32; 2], b: [f32; 2], p: [f32; 2]) -> (f32, f32, f32) {
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
