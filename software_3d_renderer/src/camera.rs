use crate::lin_alg::cross_product;
use crate::matrix4::Matrix4;
use crate::vertex::Vertex4;

pub struct Camera {
    location: Vertex4,
    direction: Vertex4,
    up: Vertex4,
    w: Vertex4,
    u: Vertex4,
    v: Vertex4,
    matrix: Matrix4,
}

impl Camera {
    pub fn new(location: Vertex4, direction: Vertex4, up: Vertex4) -> Self {
        let w = -direction.normalized();
        let u = cross_product(up, w).normalized();
        let v = cross_product(w, u);

        

        Self {
            location,
            direction,
            up,
            w,
            u,
            v,
        }
    }

    pub fn world_to_camera(&self, vertex: Vertex4) -> Vertex4 {
        // SS: translate

        // SS: rotate
        vertex
    }
}
