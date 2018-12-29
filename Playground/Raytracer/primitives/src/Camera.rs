use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;

pub struct Camera {
    origin: Vertex4f,
    upper_left: Vertex4f,
    up: Vector4f,
    right: Vector4f,
}

impl Camera {
    pub fn new() -> Camera {
        /* The camera coordinate system is right-handed, with x pointing to the right, y pointing up and
         * the negative z axis pointing into the screen.
         * The camera is positioned at (0, 0, 0), the display screen at z=-1 with x in [-2, -2]
         * and y in [-2, -2].
         */
        let upper_left_corner = Vertex4f::new(-2.0, 1.0, -1.0, 0.0);
        let vertical = Vector4f::new(0.0, -2.0, 0.0, 0.0);
        let horizontal = Vector4f::new(4.0, 0.0, 0.0, 0.0);
        let camera_origin = Vertex4f::new(0.0, 0.0, 0.0, 0.0);
        Camera { origin: camera_origin, upper_left: upper_left_corner, up: vertical, right: horizontal }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_direction = self.upper_left + u * self.right + v * self.up;
        Ray::new(self.origin, ray_direction.as_vector())
    }

}
