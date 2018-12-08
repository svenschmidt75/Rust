use std::ops::{Index, IndexMut};
use std::fmt;
use std::ops;
use Vertex4f;


#[derive(Copy, Clone)]
pub struct Matrix4f {
	data: [f64; 16],
}

#[allow(dead_code)]
impl Matrix4f {
	pub fn new() -> Self {
		// returns identity matrix
		Matrix4f{data: [1.0, 0.0, 0.0, 0.0,
						0.0, 1.0, 0.0, 0.0,
						0.0, 0.0, 1.0, 0.0,
						0.0, 0.0, 0.0, 1.0]}
	}

	pub fn getRotateX(theta: f64) -> Self {
		let cos = theta.cos();
		let sin = theta.sin();
		Matrix4f{data: [1.0, 0.0,   0.0, 0.0,
						0.0, cos, - sin, 0.0,
						0.0, sin,   cos, 0.0,
						0.0, 0.0,   0.0, 1.0]}
	}

	pub fn getRotateY(theta: f64) -> Self {
		let cos = theta.cos();
		let sin = theta.sin();
		Matrix4f{data: [  cos, 0.0,   sin, 0.0,
						  0.0, 1.0,   0.0, 0.0,
						- sin, 0.0,   cos, 0.0,
						  0.0, 0.0,   0.0, 1.0]}
	}

	pub fn getRotateZ(theta: f64) -> Self {
		let cos = theta.cos();
		let sin = theta.sin();
		Matrix4f{data: [cos, - sin, 0.0, 0.0,
						sin,   cos, 0.0, 0.0,
						0.0,   0.0, 1.0, 0.0,
						0.0,   0.0, 0.0, 1.0]}
	}

}

impl fmt::Display for Matrix4f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "\n");
        let _ = write!(f, "({}, {}, {}, {})\n", self[(0, 0)], self[(0, 1)], self[(0, 2)], self[(0, 3)]);
        let _ = write!(f, "({}, {}, {}, {})\n", self[(1, 0)], self[(1, 1)], self[(1, 2)], self[(1, 3)]);
        let _ = write!(f, "({}, {}, {}, {})\n", self[(2, 0)], self[(2, 1)], self[(2, 2)], self[(2, 3)]);
        write!(f, "({}, {}, {}, {})\n", self[(3, 0)], self[(3, 1)], self[(3, 2)], self[(3, 3)])
    }
}

impl Index<(u32, u32)> for Matrix4f {
	type Output = f64;
    
    fn index(&self, index: (u32, u32)) -> &Self::Output {
    	let flat_index = index.0 * 4 + index.1;
		assert!(flat_index < 16);
    	&self.data[flat_index as usize]
    }
}

impl IndexMut<(u32, u32)> for Matrix4f {
   
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
    	let flat_index = index.0 * 4 + index.1;
		assert!(flat_index < 16);
    	&mut self.data[flat_index as usize]
    }
}

impl ops::Mul<Matrix4f> for Matrix4f {
	type Output = Matrix4f;

    fn mul(self, rhs: Matrix4f) -> Self::Output {
    	let m00 = self[(0, 0)] * rhs[(0, 0)] +
				  self[(0, 1)] * rhs[(1, 0)] +
				  self[(0, 2)] * rhs[(2, 0)] +
				  self[(0, 3)] * rhs[(3, 0)];

    	let m01 = self[(0, 0)] * rhs[(0, 1)] +
				  self[(0, 1)] * rhs[(1, 1)] +
				  self[(0, 2)] * rhs[(2, 1)] +
				  self[(0, 3)] * rhs[(3, 1)];

    	let m02 = self[(0, 0)] * rhs[(0, 2)] +
				  self[(0, 1)] * rhs[(1, 2)] +
				  self[(0, 2)] * rhs[(2, 2)] +
				  self[(0, 3)] * rhs[(3, 2)];

    	let m03 = self[(0, 0)] * rhs[(0, 3)] +
				  self[(0, 1)] * rhs[(1, 3)] +
				  self[(0, 2)] * rhs[(2, 3)] +
				  self[(0, 3)] * rhs[(3, 3)];

    	let m10 = self[(1, 0)] * rhs[(0, 0)] +
				  self[(1, 1)] * rhs[(1, 0)] +
				  self[(1, 2)] * rhs[(2, 0)] +
				  self[(1, 3)] * rhs[(3, 0)];

    	let m11 = self[(1, 0)] * rhs[(0, 1)] +
				  self[(1, 1)] * rhs[(1, 1)] +
				  self[(1, 2)] * rhs[(2, 1)] +
				  self[(1, 3)] * rhs[(3, 1)];

    	let m12 = self[(1, 0)] * rhs[(0, 2)] +
				  self[(1, 1)] * rhs[(1, 2)] +
				  self[(1, 2)] * rhs[(2, 2)] +
				  self[(1, 3)] * rhs[(3, 2)];

    	let m13 = self[(1, 0)] * rhs[(0, 3)] +
				  self[(1, 1)] * rhs[(1, 3)] +
				  self[(1, 2)] * rhs[(2, 3)] +
				  self[(1, 3)] * rhs[(3, 3)];

    	let m20 = self[(2, 0)] * rhs[(0, 0)] +
				  self[(2, 1)] * rhs[(1, 0)] +
				  self[(2, 2)] * rhs[(2, 0)] +
				  self[(2, 3)] * rhs[(3, 0)];

    	let m21 = self[(2, 0)] * rhs[(0, 1)] +
				  self[(2, 1)] * rhs[(1, 1)] +
				  self[(2, 2)] * rhs[(2, 1)] +
				  self[(2, 3)] * rhs[(3, 1)];

    	let m22 = self[(2, 0)] * rhs[(0, 2)] +
				  self[(2, 1)] * rhs[(1, 2)] +
				  self[(2, 2)] * rhs[(2, 2)] +
				  self[(2, 3)] * rhs[(3, 2)];

    	let m23 = self[(2, 0)] * rhs[(0, 3)] +
				  self[(2, 1)] * rhs[(1, 3)] +
				  self[(2, 2)] * rhs[(2, 3)] +
				  self[(2, 3)] * rhs[(3, 3)];

    	let m30 = self[(3, 0)] * rhs[(0, 0)] +
				  self[(3, 1)] * rhs[(1, 0)] +
				  self[(3, 2)] * rhs[(2, 0)] +
				  self[(3, 3)] * rhs[(3, 0)];

    	let m31 = self[(3, 0)] * rhs[(0, 1)] +
				  self[(3, 1)] * rhs[(1, 1)] +
				  self[(3, 2)] * rhs[(2, 1)] +
				  self[(3, 3)] * rhs[(3, 1)];

    	let m32 = self[(3, 0)] * rhs[(0, 2)] +
				  self[(3, 1)] * rhs[(1, 2)] +
				  self[(3, 2)] * rhs[(2, 2)] +
				  self[(3, 3)] * rhs[(3, 2)];

    	let m33 = self[(3, 0)] * rhs[(0, 3)] +
				  self[(3, 1)] * rhs[(1, 3)] +
				  self[(3, 2)] * rhs[(2, 3)] +
				  self[(3, 3)] * rhs[(3, 3)];

		Matrix4f{data: [m00, m01, m02, m03,
						m10, m11, m12, m13,
						m20, m21, m22, m23,
						m30, m31, m32, m33,]}
    }

}


impl ops::Mul<Vertex4f::Vertex4f> for Matrix4f {
	type Output = Vertex4f::Vertex4f;

    fn mul(self, rhs: Vertex4f::Vertex4f) -> Self::Output {
    	Vertex4f::Vertex4f::new(
    		self[(0, 0)] * rhs[0] + self[(0, 1)] * rhs[1] + self[(0, 2)] * rhs[2] + self[(0, 3)] * rhs[3],
    		self[(1, 0)] * rhs[0] + self[(1, 1)] * rhs[1] + self[(1, 2)] * rhs[2] + self[(1, 3)] * rhs[3],
    		self[(2, 0)] * rhs[0] + self[(2, 1)] * rhs[1] + self[(2, 2)] * rhs[2] + self[(2, 3)] * rhs[3],
    		self[(3, 0)] * rhs[0] + self[(3, 1)] * rhs[1] + self[(3, 2)] * rhs[2] + self[(3, 3)] * rhs[3]
    		)
    }
}


#[cfg(test)]
mod tests {

	use super::Matrix4f;
	use Vertex4f;
	use std;

	fn float_cmp(expected: f64, b: f64, tolerance: f64) -> bool {
		let diff = (expected - b).abs();
		match diff < tolerance {
			true => true,
			_ => {
				println!("Expected {} but was {}, diff: {}", expected, b, diff);
				false
			}
		}
	}


	#[test]
	fn test_immutable_indexing() {
		let m = Matrix4f::new();
		let item00 = m[(0, 0)];
		assert!(float_cmp(1.0, item00, 1E-8));

		let item33 = m[(3, 3)];
		assert!(float_cmp(1.0, item33, 1E-8));
	}

	#[test]
	fn test_mutable_indexing() {
		let mut m = Matrix4f::new();
		let item00 = 1.1;
		m[(0, 0)] = item00;
		assert!(float_cmp(item00, m[(0, 0)], 1E-8));

		let item33 = 2.1;
		m[(3, 3)] = item33;
		assert!(float_cmp(item33, m[(3, 3)], 1E-8));
	}

	#[test]
	fn test_rotationX() {
		let angle = 90.0;
		let radians: f64 = std::f64::consts::PI / 180.0 * angle;
		let m = Matrix4f::getRotateX(radians);
		let vector = Vertex4f::Vertex4f::new(0.0, 0.0, 1.0, 1.0);
		let rotated_vector = m * vector;
		assert!(float_cmp(0.0, rotated_vector[0], 1E-8));
		assert!(float_cmp(-1.0, rotated_vector[1], 1E-8));
		assert!(float_cmp(0.0, rotated_vector[2], 1E-8));
		assert!(float_cmp(1.0, rotated_vector[3], 1E-8));
	}

	#[test]
	fn test_rotationY() {
		let angle = 90.0;
		let radians: f64 = std::f64::consts::PI / 180.0 * angle;
		let m = Matrix4f::getRotateY(radians);
		let vector = Vertex4f::Vertex4f::new(1.0, 0.0, 0.0, 1.0);
		let rotated_vector = m * vector;
		println!("{}", rotated_vector);
		assert!(float_cmp(0.0, rotated_vector[0], 1E-8));
		assert!(float_cmp(0.0, rotated_vector[1], 1E-8));
		assert!(float_cmp(-1.0, rotated_vector[2], 1E-8));
		assert!(float_cmp(1.0, rotated_vector[3], 1E-8));
	}

	#[test]
	fn test_rotationZ() {
		let angle = 90.0;
		let radians: f64 = std::f64::consts::PI / 180.0 * angle;
		let m = Matrix4f::getRotateZ(radians);
		let vector = Vertex4f::Vertex4f::new(1.0, 0.0, 0.0, 1.0);
		let rotated_vector = m * vector;
		assert!(float_cmp(0.0, rotated_vector[0], 1E-8));
		assert!(float_cmp(1.0, rotated_vector[1], 1E-8));
		assert!(float_cmp(0.0, rotated_vector[2], 1E-8));
		assert!(float_cmp(1.0, rotated_vector[3], 1E-8));
	}

	#[test]
	fn test_multiplication() {
		let mut m = Matrix4f::new();
		m[(0, 0)] = 12.0 / 11.0;
		m[(0, 1)] = - 6.0 / 11.0;
		m[(0, 2)] = - 1.0 / 11.0;
		m[(0, 3)] = 0.0;
		m[(1, 0)] = 5.0 / 22.0;
		m[(1, 1)] = 3.0 / 22.0;
		m[(1, 2)] = - 5.0 / 22.0;
		m[(1, 3)] = 0.0;
		m[(2, 0)] = - 2.0 / 11.0;
		m[(2, 1)] = 1.0 / 11.0;
		m[(2, 2)] = 2.0 / 11.0;
		m[(2, 3)] = 0.0;
		m[(3, 0)] = 0.0;
		m[(3, 1)] = 0.0;
		m[(3, 2)] = 0.0;
		m[(3, 3)] = 1.0;

		let mut m_inv = Matrix4f::new();
		m_inv[(0, 0)] = 1.0;
		m_inv[(0, 1)] = 2.0;
		m_inv[(0, 2)] = 3.0;
		m_inv[(0, 3)] = 0.0;
		m_inv[(1, 0)] = 0.0;
		m_inv[(1, 1)] = 4.0;
		m_inv[(1, 2)] = 5.0;
		m_inv[(1, 3)] = 0.0;
		m_inv[(2, 0)] = 1.0;
		m_inv[(2, 1)] = 0.0;
		m_inv[(2, 2)] = 6.0;
		m_inv[(2, 3)] = 0.0;
		m_inv[(3, 0)] = 0.0;
		m_inv[(3, 1)] = 0.0;
		m_inv[(3, 2)] = 0.0;
		m_inv[(3, 3)] = 1.0;

		let result = m * m_inv;

		let identity = Matrix4f::new();

		assert!(float_cmp(identity[(0, 0)], result[(0, 0)], 1E-8));
		assert!(float_cmp(identity[(0, 1)], result[(0, 1)], 1E-8));
		assert!(float_cmp(identity[(0, 2)], result[(0, 2)], 1E-8));
		assert!(float_cmp(identity[(0, 3)], result[(0, 3)], 1E-8));

		assert!(float_cmp(identity[(1, 0)], result[(1, 0)], 1E-8));
		assert!(float_cmp(identity[(1, 1)], result[(1, 1)], 1E-8));
		assert!(float_cmp(identity[(1, 2)], result[(1, 2)], 1E-8));
		assert!(float_cmp(identity[(1, 3)], result[(1, 3)], 1E-8));

		assert!(float_cmp(identity[(2, 0)], result[(2, 0)], 1E-8));
		assert!(float_cmp(identity[(2, 1)], result[(2, 1)], 1E-8));
		assert!(float_cmp(identity[(2, 2)], result[(2, 2)], 1E-8));
		assert!(float_cmp(identity[(2, 3)], result[(2, 3)], 1E-8));

		assert!(float_cmp(identity[(3, 0)], result[(3, 0)], 1E-8));
		assert!(float_cmp(identity[(3, 1)], result[(3, 1)], 1E-8));
		assert!(float_cmp(identity[(3, 2)], result[(3, 2)], 1E-8));
		assert!(float_cmp(identity[(3, 3)], result[(3, 3)], 1E-8));
	}

}