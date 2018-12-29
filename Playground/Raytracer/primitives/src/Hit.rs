use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;
use crate::Material::Material;

#[derive(Clone)]
pub struct Hit<'a> {
    pub t: f64,
    pub intersection_point: Vertex4f,
    pub normal: Vector4f,
    pub material: &'a Material
}
