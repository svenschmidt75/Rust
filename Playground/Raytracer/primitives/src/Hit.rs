use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub intersection_point: Vertex4f,
    pub normal: Vector4f
}

impl Hit {
    pub fn new() -> Hit {
        Hit { t: 0.0, intersection_point: Vertex4f::new(0.0, 0.0, 0.0, 0.0), normal: Vector4f::new(0.0, 0.0, 0.0, 0.0) }
    }
}
