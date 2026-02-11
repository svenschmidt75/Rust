use std::cmp::{max, min};
use crate::render_context::RenderContext;
use crate::renderable::Renderable;
use sfml::graphics::Vertex;
use crate::vertex;

pub struct Triangle {
    vertices: [vertex::Vertex4; 3],
}

impl Triangle {
    pub fn new(vertices: [vertex::Vertex4; 3]) -> Self {
        Triangle { vertices }
    }
}

impl Renderable for Triangle {
    fn render(&self, ctx: &mut RenderContext) {
        let screen_vertices = ctx.world_to_screen(&self.vertices);

        // SS: order vertices in y coordinate
        let mut v: [[i32; 2]; 3] = [screen_vertices[0], screen_vertices[1], screen_vertices[2]];

        // SS: 0 and 1 are in descending order w.r.t. screen y coordinate
        if v[0][1] < v[1][1] {
            let tmp = v[0];
            v[0] = v[1];
            v[1] = tmp;
        }

        // SS: order 1 and 2
        if v[1][1] < v[2][1] {
            let tmp = v[1];
            v[1] = v[2];
            v[2] = tmp;
        }

        // SS: 1 and 2 are in descending order w.r.t. screen y coordinate.
        // Check 0 and 1...
        if v[0][1] < v[1][1] {
            let tmp = v[0];
            v[0] = v[1];
            v[1] = tmp;
        }

        let [[x0, y0], [x1, y1], [x2, y2]] = v;

        // SS: screen vertices are ordered in screen y in descending order
        let delta1 = ((x1 as f64 - x0 as f64) / (y1 as f64 - y0 as f64)).abs();
        let delta2 = ((x2 as f64 - x0 as f64) / (y2 as f64 - y0 as f64)).abs();

        let min_y = min(y0, y1);
        let max_y = max(y0, y1);

        for y in min_y..(max_y + 1) {
            let x1 = x0 + ((y as f64 - min_y as f64) * delta1) as i32;
            let x2 = x0 + ((y as f64 - min_y as f64) * delta2) as i32;

            let min_x = min(x1, x2);
            let max_x = max(x1, x2);

            // SS: render scanline
            for x in min_x..(max_x + 1) {
                ctx.set_pixel(x as u32, y as u32, 120, 120, 120, 255);
            }

        }
    }
}
