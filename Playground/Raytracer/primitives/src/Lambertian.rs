use crate::Material::Material;
use crate::operations::random_point_on_unit_sphere;
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
    fn scatter(&self, _ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> Option<(Ray, Vector4f)> {
        let target = intersection_point.as_vector() + normal + random_point_on_unit_sphere();
        let scattered_ray = Ray::new(intersection_point, target - intersection_point.as_vector());
        Some((scattered_ray, self.albedo))
    }
}
