use Color::Color;
use Ray::Ray;

pub trait Shape {
    fn getColor(&self) -> Color;
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<f64>;
}
