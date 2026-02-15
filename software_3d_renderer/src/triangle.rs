use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use crate::vertex;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    vertices: [vertex::Vertex4; 3],
}

impl Triangle {
    pub fn new(vertices: [vertex::Vertex4; 3]) -> Self {
        // SS: Triangle vertices must be oriented. This is because when rasterizing the triangle,
        // we calculate signed areas w.r.t. to points inside the triangle. If they are not oriented,
        // rasterization will fail!
        Triangle { vertices }
    }

}

impl Renderable for Triangle {
    fn render(&self, ctx: &mut RenderContext, delta: f32) {
        // SS: approach described in Fundamentals of Computer Graphics, 5th edition
        // 9.1.2 Triangle Rasterization
        // We scan left-right, top-bottom  the bounding box of the triangle. For each point,
        // we calculate the signed area (edge_function) to check whether the point is inside
        // the triangle. We calculate barycentric coordinates for interpolation purposes
        // (color, texture, ...)
        let screen_vertices = ctx.world_to_screen(&self.vertices);

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

        let red = [255, 0, 0];
        let green = [0, 255, 0];
        let blue = [0, 0, 255];

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

                    let cx =
                        alpha * red[0] as f32 + beta * green[0] as f32 + gamma * blue[0] as f32;
                    let cy =
                        alpha * red[1] as f32 + beta * green[1] as f32 + gamma * blue[1] as f32;
                    let cz =
                        alpha * red[2] as f32 + beta * green[2] as f32 + gamma * blue[2] as f32;

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
