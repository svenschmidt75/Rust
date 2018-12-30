use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;
use std::f32::consts;
use crate::operations;

pub struct Camera {
    origin: Vertex4f,
    lower_left: Vertex4f,
    vertical: Vector4f,
    horizontal: Vector4f,
}

impl Camera {
    pub fn new(lookfrom: Vertex4f, lookat: Vertex4f, vup: Vector4f, vfov: f32, aspect: f32) -> Camera {
        /* The camera coordinate system is right-handed, with x pointing to the right, y pointing up and
         * the negative z axis pointing into the screen.
         * The camera is positioned at (0, 0, 0), the display screen at z=-1 with x in [-2, -2]
         * and y in [-2, -2].
         */
        let theta = vfov * consts::PI / 180f32;
        let half_height = (theta / 2f32).tan();
        let half_width =  aspect * half_height;
        let camera_origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = (operations::cross(vup, w)).normalize();
        let v = operations::cross(w, u);
        let lower_left_corner = camera_origin.as_vector() - half_width * u - half_height * v - w;
        let vertical = 2f32 * half_height * v;
        let horizontal = 2f32 * half_width * u;
        Camera { origin: camera_origin, lower_left: Vertex4f::new(lower_left_corner.x, lower_left_corner.y, lower_left_corner.z, lower_left_corner.w), vertical: vertical, horizontal: horizontal }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_direction = self.lower_left + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, ray_direction)
    }

}
