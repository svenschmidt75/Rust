use Vector4f::Vector4f;
use Vertex4f::Vertex4f;
use Material::Material;
use Lambertian::Lambertian;

#[derive(Clone)]
pub struct Hit<'a> {
    pub t: f64,
    pub intersection_point: Vertex4f,
    pub normal: Vector4f,
    pub material: &'a Material
}
