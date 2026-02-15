use sfml::graphics::ShaderType::Vertex;
use crate::lin_alg::cross_product;
use crate::matrix4::Matrix4;
use crate::vertex::Vertex4;

#[derive(Debug, Clone, Copy)]
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

    pub fn from_look_at(r: f32, theta: f32, phi: f32) -> Self {
        // SS: r is the radius of the orbital camera from the world's origin,
        // theta is the longitudinal angle (equator) and phi the latitude
        // angle ranging from +pi/2 (North Pole) to -pi/2 (South Pole).
        // Note that angles are expected to be in radians.

        // SS: calculate new camera position in the world on the sphere around
        // the origin of radius r
        let xp = r * phi.cos() * theta.sin();
        let yp = r * phi.sin();
        let zp = r * phi.cos() * theta.cos();
        let position = Vertex4::new_vertex(xp, yp, zp);

        // SS: calculate direction vector such that the camera looks at the world's origin
        let w = position.normalized();

        // SS: t is world up
        let up = Vertex4::new_vector(0.0, 1.0, 0.0);

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
            direction: w,
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
