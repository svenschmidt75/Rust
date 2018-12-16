use Material::Material;
use Ray::Ray;
use Vector4f::Vector4f;
use Vertex4f::Vertex4f;
use operations::dot;

pub struct Metal {
    albedo: Vector4f
}

impl Metal {
    pub fn new(albedo: Vector4f) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> (bool, Ray, Vector4f) {
        let reflected = reflect(ray.direction, normal);
        let scattered_ray = Ray::new(intersection_point, reflected);
        let visible = dot(scattered_ray.direction, normal) > 0.0;
        (visible, scattered_ray, self.albedo)
    }
}

fn reflect(ray_direction: Vector4f, normal: Vector4f) -> Vector4f {
    let length = dot(ray_direction, normal);
    ray_direction - 2.0 * length * normal
}