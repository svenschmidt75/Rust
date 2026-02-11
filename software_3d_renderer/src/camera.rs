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
    pub fn new(position: Vertex4, direction: Vertex4, up: Vertex4) -> Self {
        // SS: compute right-handed coordinates
        let w = -direction.normalized();
        let u = cross_product(up, w).normalized();
        let v = cross_product(w, u);

        // SS: compute camera matrix transform
        let mut m1 = Matrix4::identity();
        m1[0][3] = - position[0];
        m1[1][3] = - position[1];
        m1[2][3] = - position[2];

        let mut m2 = Matrix4::identity();
        m2[0][0] = u[0];
        m2[0][1] = u[1];
        m2[0][2] = u[2];

        m2[1][0] = v[0];
        m2[1][1] = v[1];
        m2[1][2] = v[2];

        m2[2][0] = w[0];
        m2[2][1] = w[1];
        m2[2][2] = w[2];

        let camera_matrix = m2 * m1;

        Self {
            location: position,
            direction,
            up,
            w,
            u,
            v,
            matrix: camera_matrix,
        }
    }

    pub fn world_to_camera(&self, vertex: Vertex4) -> Vertex4 {
        // SS: transform vertex from world to camera space
        self.matrix * vertex
    }
}
