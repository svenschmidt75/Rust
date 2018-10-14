#![allow(non_snake_case)]

use std::ops::{Index, IndexMut};
use std::fmt;
use std::ops;

use CompareWithTolerance::CompareWithTolerance;
use operations;
use Vector4f::Vector4f;


pub struct Vertex4f {
	pub x: f64,
	pub y: f64,
	pub z: f64,

	// vertices always have w = 1
	pub w: f64,
}

impl Vertex4f {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Vertex4f{x: x, y: y, z: z, w: w}
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

	// TODO SS: Should that be described as a trait?
	// Deref<Vector4f> for Vertex4f?
	pub fn as_vector(&self) -> Vector4f {
		Vector4f::new(self.x, self.y, self.z, self.w)
	}
	
}

impl fmt::Display for Vertex4f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// We want Vertex4f to behave as value type, i.e. instead of moving, it gets copied
// when passing to a function. The Copy trait depends on the Clone trait.

impl Clone for Vertex4f {
    fn clone(&self) -> Self {
    	Vertex4f::new(self.x, self.y, self.z, self.w)
    }
}

impl Copy for Vertex4f {}


// compare Vertex4f

impl CompareWithTolerance for Vertex4f {
	fn cmp(self, other: Vertex4f, tol: f64) -> bool {
		operations::float_cmp(self.x, other.x, tol) &&
		operations::float_cmp(self.y, other.y, tol) &&
		operations::float_cmp(self.z, other.z, tol) &&
		operations::float_cmp(self.w, other.w, tol)
	}
}

// Operator overloading

impl ops::Sub<Vertex4f> for Vertex4f {
	type Output = Vector4f;

    fn sub(self, rhs: Self) -> Self::Output {
    	Vector4f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl ops::Add<Vertex4f> for Vertex4f {
	type Output = Vector4f;

    fn add(self, rhs: Self) -> Self::Output {
    	Vector4f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl ops::Neg for Vertex4f {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Vertex4f::new(- self.x, - self.y, - self.z, - self.w)
	}
}


// Indexing

impl Index<u32> for Vertex4f {
	type Output = f64;
    
    fn index(&self, index: u32) -> &Self::Output {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			3 => &self.w,
			_ => panic!("Index {} out of bounds", index),
		}
    }
}

impl IndexMut<u32> for Vertex4f {
   
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			3 => &mut self.w,
			_ => panic!("Index {} out of bounds", index),
		}
    }
}
