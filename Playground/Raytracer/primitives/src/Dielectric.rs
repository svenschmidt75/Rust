use rand::{Open01, random};

use Material::Material;
use operations::dot;
use Ray::Ray;
use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

// Snell's Law for Refraction
// n * sin (theta) = n' * sin (theta')
// air: n=1
// glass: n=1.3-1.7
// diamond: n=2.4

impl Material for Dielectric {
    fn scatter(&self, incoming_ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> (bool, Ray, Vector4f) {
        let mut outward_normal: Vector4f;
        let mut ni_over_nt: f32;
        if dot(incoming_ray.direction, normal) > 0.0 {
            outward_normal = - normal;
            ni_over_nt = self.refraction_index;
        }
        else {
            outward_normal = normal;
            // SS: air = 1.0
            ni_over_nt = 1.0 / self.refraction_index;
        }
        let attenuation = Vector4f::new(1.0, 1.0, 1.0, 1.0);
        let (has_refraction_ray, refraction_ray) = refract(incoming_ray.direction, outward_normal, ni_over_nt);
        if has_refraction_ray {
            let scattered = Ray::new(incoming_ray.origin, refraction_ray);
            (true, scattered, attenuation)
        }
        else {
            let reflected_ray = reflect(incoming_ray.direction, normal);
            let scattered = Ray::new(incoming_ray.origin, reflected_ray);
            (true, scattered, attenuation)
        }
    }
}

fn reflect(ray_direction: Vector4f, normal: Vector4f) -> Vector4f {
    let length = dot(ray_direction, normal);
    ray_direction - 2.0 * length * normal
}

// SS: use cos^{2}(x) = 1 - sin^{2}(x)
fn refract(incoming_ray: Vector4f, normal: Vector4f, ni_over_nt: f32) -> (bool, Vector4f) {
    let uv = incoming_ray.normalize();
    let dt = dot(uv, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt();
        (true, refracted)
    }
    (false, Vector4f::new(0.0, 0.0, 0.0, 0.0))
}
