#![allow(non_snake_case)]

use std::fmt;
use std::ops;


pub struct Vector4f {
	pub x: f64,
	pub y: f64,
	pub z: f64,

	// vectors always have w = 0
	pub w: f64,
}

impl Vector4f {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Vector4f{x: x, y: y, z: z, w: w}
	}

	pub fn norm(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
	}

	pub fn x(&self) -> f64 {
		self.x
	}

	pub fn y(&self) -> f64 {
		self.y
	}

	pub fn z(&self) -> f64 {
		self.z
	}

	pub fn w(&self) -> f64 {
		self.w
	}
	
	pub fn normalize(&self) -> Self {
		let n = self.norm();
		Vector4f::new(self.x / n, self.y / n, self.z / n, self.w / n)
	}

}

impl fmt::Display for Vector4f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// We want Vector4f to behave as value type, i.e. instead of moving, it gets copied
// when passing to a function. The Copy trait depends on the Clone trait.

impl Clone for Vector4f {
    fn clone(&self) -> Self {
    	Vector4f::new(self.x, self.y, self.z, self.w)
    }
}

impl Copy for Vector4f {}



// Operator overloading

impl ops::Div<f64> for Vector4f {
	type Output = Vector4f;

    fn div(self, rhs: f64) -> Self::Output {
    	Vector4f::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl ops::Add<Vector4f> for Vector4f {
	type Output = Vector4f;

    fn add(self, rhs: Self) -> Self::Output {
    	Vector4f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.y, self.w + rhs.w)
    }
}


#[cfg(test)]
mod tests {

	use super::Vector4f;
	use operations;


	#[test]
	fn test_norm() {
		let vec = Vector4f::new(1.34, 2.53, -9.547, 1.0);
		assert!(operations::float_cmp(10.017070879254074, vec.norm(), 1E-8));
	}

	#[test]
	fn test_normalize() {
		let vec = Vector4f::new(1.34, 2.53, -9.547, 1.0).normalize();
		assert!(operations::float_cmp(1.0, vec.norm(), 1E-8));
	}
}