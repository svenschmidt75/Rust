use std::f32::consts;

use crate::operations;
use crate::operations::random_point_on_unit_disk;
use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;

pub struct Camera {
    origin: Vertex4f,
    lower_left: Vertex4f,
    vertical: Vector4f,
    horizontal: Vector4f,
    u: Vector4f,
    v: Vector4f,
    w: Vector4f,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Vertex4f, lookat: Vertex4f, vup: Vector4f, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        /* The camera coordinate system is right-handed, with x pointing to the right, y pointing up and
         * the negative z axis pointing into the screen.
         * The camera is positioned at (0, 0, 0), the display screen at z=-1 with x in [-2, -2]
         * and y in [-2, -2].
         */
        let theta = vfov * consts::PI / 180f32;
        let half_height = (theta / 2f32).tan();
        let half_width = aspect * half_height;
        let camera_origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = (operations::cross(vup, w)).normalize();
        let v = operations::cross(w, u);
        // SS: projection plane is at w=1, hence -w because the lower left is at the camera position at w=0
        let lower_left_corner = camera_origin.as_vector() - half_width * u - half_height * v - focus_dist * w;
        let vertical = 2f32 * half_height * v;
        let horizontal = 2f32 * half_width * u;
        Camera { origin: camera_origin, lower_left: Vertex4f::new(lower_left_corner.x, lower_left_corner.y, lower_left_corner.z, lower_left_corner.w), vertical, horizontal, u, v, w, lens_radius: aperture / 2f32 }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_point_on_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        let ray_direction = self.lower_left + u * self.horizontal + v * self.vertical - self.origin - offset;
        Ray::new(self.origin + offset, ray_direction)
    }
}
