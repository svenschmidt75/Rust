use Color::Color;
use Ray::Ray;

pub trait Shape {
    fn getColor(&self) -> Color;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}
