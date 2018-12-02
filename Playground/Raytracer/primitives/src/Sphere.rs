use Color::Color;
use operations;
use Ray::Ray;
use Shape::Shape;
use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

pub struct Sphere {
    color: Color,
    radius: f64,
    center: Vertex4f,
}

impl Sphere {
    pub fn new(color: Color, radius: f64, center: Vertex4f) -> Self {
        Sphere { color: color, radius: radius, center: center }
    }

    pub fn getNormal(n: Vertex4f) -> Vector4f {
        let norm = n.as_vector().norm();
        Vector4f::new(n.x / norm, n.y / norm, n.z / norm, n.w / norm)
    }

    fn points_to_normal(&self, p: Vertex4f, ray: &Ray) -> bool {
        let normal_at_point = Sphere::getNormal(p);
        operations::pointing_at_camera(ray.direction, normal_at_point)
    }
}

fn calculate_intersection_points(sphere: &Sphere, ray: &Ray, discriminant_sqrt: f64) -> Vertex4f {
    let pd = operations::dot(ray.origin - sphere.center, ray.direction);
    let lambda = -pd + discriminant_sqrt;
    let intersection_point = Vertex4f::new(
        ray.origin.x + lambda * ray.direction.x,
        ray.origin.y + lambda * ray.direction.y,
        ray.origin.z + lambda * ray.direction.z,
        ray.origin.w + lambda * ray.direction.w,
    );
    intersection_point
}

impl Shape for Sphere {
    fn getColor(&self) -> Color {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let r2 = self.radius * self.radius;

        // We make all calculations based on the sphere translated to the center.
        // Correct ray reference point for this.
        let oc = ray.origin - self.center;
        let c = operations::dot(oc, oc) - r2;
        let b = 2.0 * operations::dot(oc, ray.direction);
        let tmp = b * b - 4.0 * c;
        match tmp < 0.0 {
            true => vec![],
            false => vec![(-b + tmp.sqrt()) / 2.0, (-b - tmp.sqrt()) / 2.0]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_does_not_point_to_camera_when_parallel() {
        let camera_direction = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let n = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(operations::pointing_at_camera(camera_direction, n), false)
    }

    #[test]
    fn test_vector_points_to_camera_when_opposite() {
        let camera_direction = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let n = Vector4f::new(-1.0, 1.0, 0.0, 0.0);
        assert_eq!(operations::pointing_at_camera(camera_direction, n), true)
    }

    #[test]
    fn test_unit_sphere_at_center_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        // Arrange
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(0.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));

        // Act
        let t = unit_sphere.intersect(&ray)[1];

        // Assert
        assert_eq!(1.0, t)
    }

    #[test]
    fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        // Arrange

        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));

        // Act
        let t = unit_sphere.intersect(&ray)[1];

        // Assert
        assert_eq!(2.0, t)
    }

    #[test]
    fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_negative_x_axis() {
        // Arrange

        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(2.0, 0.0, 0.0, 0.0), Vector4f::new(-1.0, 0.0, 0.0, 0.0));

        // Act
        let t = unit_sphere.intersect(&ray)[1];

        // Assert
        assert_eq!(0.0, t)
    }
}
