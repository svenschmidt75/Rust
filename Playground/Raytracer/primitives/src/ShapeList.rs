use Ray::Ray;
use Shape::Shape;
use Color::Color;
use Hit::Hit;

struct ShapeList(Vec<Box<Shape>>);

impl Shape for ShapeList {
    fn getColor(&self) -> Color {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<Hit> {
        // intersect ray with all shapes in shapelist
        //


        vec![]
    }
}