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
    fn new(color: Color, radius: f64, center: Vertex4f) -> Self {
        Sphere{color: color, radius: radius, center: center}
    }
    
    fn getNormal(n: Vertex4f) -> Vector4f {
        let norm = n.as_vector().norm();
        Vector4f::new(n.x / norm, n.y / norm, n.z / norm, n.w / norm)
    }
    
    fn points_to_normal(&self, p: Vertex4f, ray: &Ray) -> bool {
        let normal_at_point = Sphere::getNormal(p);
        operations::pointing_at_camera(ray.direction, normal_at_point)
    }

}

fn calculate_sqrt_of_discriminant(sphere: &Sphere, ray: &Ray) -> Option<(f64, f64)> {
    let r2 = sphere.radius * sphere.radius;

    // We make all calculations based on the sphere translated to the center.
    // Correct ray reference point for this.
    let ray_reference_point = ray.reference_point - sphere.center; 
    let p2 = ray_reference_point.norm() * ray_reference_point.norm();
    let pd = operations::dot(ray_reference_point, ray.direction);
    let tmp = r2 - p2 + pd * pd;
    match tmp < 0.0 {
        true  => None,
        false => Some((tmp.sqrt(), - tmp.sqrt()))
    }
}

fn calculate_intersection_points(sphere: &Sphere, ray: &Ray, discriminant_sqrt: f64) -> Vertex4f {
    let pd = operations::dot(ray.reference_point - sphere.center, ray.direction);
    let lambda = - pd + discriminant_sqrt;
    let intersection_point = Vertex4f::new(
                ray.reference_point.x + lambda * ray.direction.x,
                ray.reference_point.y + lambda * ray.direction.y,
                ray.reference_point.z + lambda * ray.direction.z,
                ray.reference_point.w + lambda * ray.direction.w,
        );
    intersection_point
}

impl Shape for Sphere {
	fn getColor(&self) -> Color {
		self.color
	}
    
	fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ts = calculate_sqrt_of_discriminant(self, ray);

        // SS: the smaller t is the one we want
        if let None = ts {
            None
        }
            else {


            }






                        .iter()
                        .map(|discriminant| calculate_intersection_points(self, ray, *discriminant))
                        .filter(|intersection_vertex| {
                            let v = *intersection_vertex - self.center;
                            let v = v.normalize();
                            operations::pointing_at_camera(ray.direction, v)
                        })
                        .collect();
        match vertices.len() {
            0 => None,
            1 => Some(vertices[0]),
            _ => panic!("More than one vertex intersect sphere"),
        }
    }
}


#[cfg(test)]
mod tests {

	use super::Sphere;
	use operations;
    use Shape::Shape;
    use Color::Color;
    use Vertex4f::Vertex4f;
    use Vector4f::Vector4f;
    use Ray::Ray;
    use CompareWithTolerance::CompareWithTolerance;


    #[test]
    fn test_vector_does_not_point_to_camera_when_parallel() {
        let camera_direction = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let n = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        assert!(operations::pointing_at_camera(camera_direction, n) == false);
    }

    #[test]
    fn test_vector_points_to_camera_when_opposite() {
        let camera_direction = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let n = Vector4f::new(-1.0, 1.0, 0.0, 0.0);
        assert!(operations::pointing_at_camera(camera_direction, n) == true);
    }

    #[test]
    fn test_calculate_sqrt_of_discriminant() {
        
    }

	#[test]
	fn test_unit_sphere_at_center_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(0.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));
        let result = unit_sphere.intersect(&ray).unwrap();
        assert!(result.cmp(Vertex4f::new(-1.0, 0.0, 0.0, 0.0), 1E-8));
	}

	#[test]
	fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_positive_x_axis() {
        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(-2.0, 0.0, 0.0, 0.0), Vector4f::new(1.0, 0.0, 0.0, 0.0));
        let result = unit_sphere.intersect(&ray).unwrap();
        assert!(result.cmp(Vertex4f::new(0.0, 0.0, 0.0, 0.0), 1E-8));
	}

	#[test]
	fn test_unit_sphere_centered_at_x_eq_1_intersects_with_ray_on_x_axis_pointing_in_negative_x_axis() {
        // unit sphere centered at x = 1
        let unit_sphere = Sphere::new(Color::new(1.0, 1.0, 1.0), 1.0, Vertex4f::new(1.0, 0.0, 0.0, 0.0));
        let ray = Ray::new(Vertex4f::new(2.0, 0.0, 0.0, 0.0), Vector4f::new(-1.0, 0.0, 0.0, 0.0));
        let result = unit_sphere.intersect(&ray).unwrap();
        assert!(result.cmp(Vertex4f::new(2.0, 0.0, 0.0, 0.0), 1E-8));
	}
}
