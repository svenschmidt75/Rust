use Vertex4f::Vertex4f;
use Vector4f::Vector4f;


pub struct Ray {
	// reference point on ray
	pub reference_point: Vertex4f,

	// normalized direction vector
	pub direction: Vector4f,
}

impl Ray {
	pub fn new(vertex: Vertex4f, direction: Vector4f) -> Self {
		Ray{reference_point: vertex, direction: direction.normalize()}
	}
}


#[cfg(test)]
mod tests {

	use super::Ray;
	use Vertex4f::Vertex4f;
	use Vector4f::Vector4f;
	use operations;
    

    #[test]
    fn test_ray_direction_is_normal() {
		let ray_direction = Vector4f::new(3.0, 0.0, 0.0, 0.0);
		let ray = Ray::new(Vertex4f::new(1.0, 1.0, 1.0, 0.0), ray_direction);
		assert!(operations::float_cmp(1.0, ray.direction.norm(), 1E-8));

		let ray_direction = Vector4f::new(3.0, 3.0, -5.0, -1.3);
		let ray = Ray::new(Vertex4f::new(1.0, 1.0, 1.0, 0.0), ray_direction);
		assert!(operations::float_cmp(1.0, ray.direction.norm(), 1E-8))
    }

}
