use std::f64;

use Color::Color;
use Hit::Hit;
use Material::Material;
use operations;
use Ray::Ray;
use Shape::Shape;
use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

pub struct Sphere {
    color: Color,
    radius: f64,
    center: Vertex4f,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(color: Color, radius: f64, center: Vertex4f, material: Box<Material>) -> Self {
        Sphere { color, radius, center, material }
    }

    pub fn getNormalAt(&self, p: &Vertex4f) -> Vector4f {
        // p is assumed to be a point on the sphere
        let n = p.as_vector() - self.center.as_vector();
        n.normalize()
    }
}

impl Shape for Sphere {
    fn getColor(&self) -> Color {
        self.color
    }

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<Hit> {
        let mut hits = vec![];
        let r2 = self.radius * self.radius;

        // We make all calculations based on the sphere translated to the center.
        // Correct ray reference point for this.
        let oc = ray.origin - self.center;
        let b = operations::dot(oc, ray.direction);
        let c = operations::dot(oc, oc) - r2;
        let discriminant = b * b - c;
        if discriminant > 0.0 {
            let t1 = -b + discriminant.sqrt();
            let t2 = -b - discriminant.sqrt();
            if t1 < t2 {
                // insert the closer intersection first
                if t1 >= t_min && t1 <= t_max {
                    let intersection_point = ray.point_on_ray(t1);
                    hits.push(Hit { t: t1, intersection_point, normal: self.getNormalAt(&intersection_point), material: self.material.as_ref() });
                }
                if t2 >= t_min && t2 <= t_max {
                    let intersection_point = ray.point_on_ray(t2);
                    hits.push(Hit { t: t2, intersection_point, normal: self.getNormalAt(&intersection_point), material: self.material.as_ref() });
                }
            } else {
                if t2 >= t_min && t2 <= t_max {
                    let intersection_point = ray.point_on_ray(t2);
                    hits.push(Hit { t: t2, intersection_point, normal: self.getNormalAt(&intersection_point), material: self.material.as_ref() });
                }
                if t1 >= t_min && t1 <= t_max {
                    let intersection_point = ray.point_on_ray(t1);
                    hits.push(Hit { t: t1, intersection_point, normal: self.getNormalAt(&intersection_point), material: self.material.as_ref() });
                }
            }
        }
        hits
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Lambertian::Lambertian;

    #[test]
    fn test_unit_sphere_at_center_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        // Arrange
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(0.0, 0.0, 0.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.0, 0.0, 0.0, 0.0))));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));

        // Act
        let hit = &unit_sphere.intersect(&ray, 0.0, f64::MAX)[0];

        // Assert
        assert_eq!(1.0, hit.t)
    }

    #[test]
    fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        // Arrange

        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.0, 0.0, 0.0, 0.0))));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));

        // Act
        let hit = &unit_sphere.intersect(&ray, 0.0, f64::MAX)[0];

        // Assert
        assert_eq!(2.0, hit.t)
    }

    #[test]
    fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_negative_x_axis() {
        // Arrange

        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.0, 0.0, 0.0, 0.0))));
        let ray = Ray::new(Vertex4f::new(2.0, 0.0, 0.0, 0.0), Vector4f::new(-1.0, 0.0, 0.0, 0.0));

        // Act
        let hit = &unit_sphere.intersect(&ray, 0.0, f64::MAX)[0];

        // Assert
        assert_eq!(0.0, hit.t)
    }

    #[test]
    fn test_getNormalAt() {
        // Arrange
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(0.0, 0.0, 0.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.0, 0.0, 0.0, 0.0))));

        // Act
        // Assert
        let normal = unit_sphere.getNormalAt(&Vertex4f::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(1.0, normal.x);

        let normal = unit_sphere.getNormalAt(&Vertex4f::new(-1.0, 0.0, 0.0, 0.0));
        assert_eq!(-1.0, normal.x);

        let normal = unit_sphere.getNormalAt(&Vertex4f::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(1.0, normal.z);
    }
}
