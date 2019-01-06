use rand::random;

use crate::Vector4f::Vector4f;

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

pub fn float_cmp(expected: f64, b: f64, tolerance: f64) -> bool {
    let diff = (expected - b).abs();
    diff < tolerance
}

pub fn random_point_on_unit_sphere() -> Vector4f {
    let mut p: Vector4f;
    loop {
        let x = random::<f64>();
        let y = random::<f64>();
        let z = random::<f64>();
        // ensure vector is in range of (-1,1)
        p = 2.0 * Vector4f::new(x, y, z, 0.0) - Vector4f::new(1.0, 1.0, 1.0, 0.0);
        if p.norm() <= 1.0 {
            break;
        }
    }
    p
}

pub fn random_point_on_unit_disk() -> Vector4f {
    let mut p: Vector4f;
    loop {
        let x = random::<f64>();
        let y = random::<f64>();
        // ensure vector is in range of (-1,1)
        p = 2.0 * Vector4f::new(x, y, 0.0, 0.0) - Vector4f::new(1.0, 1.0, 0.0, 0.0);
        if p.norm() <= 1.0 {
            break;
        }
    }
    p
}
