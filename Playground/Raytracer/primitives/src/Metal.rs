use rand::{Open01, random};

use crate::Material::Material;
use crate::operations::dot;
use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;

pub struct Metal {
    albedo: Vector4f,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vector4f, mut fuzz: f32) -> Metal {
        if fuzz > 1.0
        {
            fuzz = 1.0
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> (bool, Ray, Vector4f) {
        let reflected = reflect(ray.direction, normal);
        let scattered_ray = Ray::new(intersection_point, reflected + self.fuzz * random_point_on_unit_sphere());
        let visible = dot(scattered_ray.direction, normal) > 0.0;
        (visible, scattered_ray, self.albedo)
    }
}

fn reflect(ray_direction: Vector4f, normal: Vector4f) -> Vector4f {
    let length = dot(ray_direction, normal);
    ray_direction - 2.0 * length * normal
}

fn random_point_on_unit_sphere() -> Vector4f {
    let mut p: Vector4f;
    loop {
        let Open01(x) = random::<Open01<f64>>();
        let Open01(y) = random::<Open01<f64>>();
        let Open01(z) = random::<Open01<f64>>();
        // ensure vector is in range of (-1,1)
        p = 2.0 * Vector4f::new(x, y, z, 0.0) - Vector4f::new(1.0, 1.0, 1.0, 0.0);
        if p.norm() <= 1.0 {
            break;
        }
    }
    p
}
