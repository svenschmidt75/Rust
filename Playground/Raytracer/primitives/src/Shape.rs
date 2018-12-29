use crate::Color::Color;
use crate::Ray::Ray;
use crate::Hit::Hit;

pub trait Shape {
    fn getColor(&self) -> Color;
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<Hit>;
}
