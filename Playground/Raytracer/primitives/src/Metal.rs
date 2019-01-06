use crate::Material::Material;
use crate::operations::dot;
use crate::operations::random_point_on_unit_sphere;
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
    fn scatter(&self, ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> Option<(Ray, Vector4f)> {
        let reflected = reflect(ray.direction, normal);
        let scattered_ray = Ray::new(intersection_point, reflected + self.fuzz * random_point_on_unit_sphere());
        let visible = dot(scattered_ray.direction, normal) > 0.0;
        if visible {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

fn reflect(ray_direction: Vector4f, normal: Vector4f) -> Vector4f {
    let length = dot(ray_direction, normal);
    ray_direction - 2.0 * length * normal
}
