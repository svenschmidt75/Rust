use Color::Color;
use Ray::Ray;
use Vertex4f::Vertex4f;

pub trait Shape {
    fn getColor(&self) -> Color;
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
