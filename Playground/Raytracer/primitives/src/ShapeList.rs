use crate::Color::Color;
use crate::Hit::Hit;
use crate::Ray::Ray;
use crate::Shape::Shape;

pub struct ShapeList(Vec<Box<dyn Shape>>);

impl ShapeList {
    pub fn new(shapes: Vec<Box<dyn Shape>>) -> ShapeList {
        ShapeList(shapes)
    }
}

impl Shape for ShapeList {
    fn getColor(&self) -> Color {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<Hit> {
        let mut current_t_max = t_max;
        let mut current_hit  = None;
        for shape in &self.0 {
            let mut hits = shape.intersect(ray, t_min, current_t_max);
            if hits.is_empty() == false {
                current_hit = Some(hits.remove(0));
                current_t_max = current_hit.as_ref().unwrap().t;
            }
        }
        if let Some(h) = current_hit {
            vec![h]
        }
        else {
            vec![]
        }
    }
}