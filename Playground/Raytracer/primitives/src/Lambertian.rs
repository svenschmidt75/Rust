extern crate rand;

use rand::{Open01, random};

use crate::Material::Material;
use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;

pub struct Lambertian {
    albedo: Vector4f
}

impl Lambertian {
    pub fn new(albedo: Vector4f) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> (bool, Ray, Vector4f) {
        let target = intersection_point.as_vector() + normal + random_point_on_unit_sphere();
        let scattered_ray = Ray::new(intersection_point, target - intersection_point.as_vector());
        (true, scattered_ray, self.albedo)
    }
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
