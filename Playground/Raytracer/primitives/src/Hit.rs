use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

pub struct Hit {
    pub t: f64,
    pub intersection_point: Vertex4f,
    pub normal: Vector4f
}
