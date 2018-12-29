use crate::Ray::Ray;
use crate::Vector4f::Vector4f;
use crate::Vertex4f::Vertex4f;

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection_point: Vertex4f, normal: Vector4f) -> (bool, Ray, Vector4f);
}