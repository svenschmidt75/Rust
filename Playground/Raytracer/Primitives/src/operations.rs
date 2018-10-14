use Vector4f::Vector4f;


pub fn cross(lhs: Vector4f, rhs: Vector4f) -> Vector4f {
	Vector4f::new(
		lhs.y * rhs.z - lhs.z * rhs.y,
		lhs.z * rhs.x - lhs.x * rhs.z,
		lhs.x * rhs.y - lhs.y * rhs.x,
		0.0)
}

pub fn dot(lhs: Vector4f, rhs: Vector4f) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
}

pub fn pointing_at_camera(camera_direction: Vector4f, n: Vector4f) -> bool {
    let dot = dot(camera_direction, n);
    dot < 0.0
}

pub fn float_cmp(expected: f64, b: f64, tolerance: f64) -> bool {
	let diff = (expected - b).abs();
	diff < tolerance
}
