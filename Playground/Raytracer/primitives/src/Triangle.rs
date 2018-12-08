use operations;
use Vector4f::Vector4f;
use Vertex4f::Vertex4f;

#[allow(dead_code)]
pub struct Triangle {
    v1: Vertex4f,
    v2: Vertex4f,
    v3: Vertex4f,
    normal: Vector4f,
}

impl Triangle {
    pub fn new(v1: Vertex4f, v2: Vertex4f, v3: Vertex4f) -> Self {
        // calculate normal, implied by the right-hand rule (v1, v2, v3), i.e. the triangle
        // is spanned by vectors (v1, v2) and (v2, v3).
        let v1v2 = v2 - v1;
        let v2v3 = v3 - v2;
        let normal = operations::cross(v1v2, v2v3);
        let normalized_normal = normal / normal.norm();
        assert!(normal.norm() > 0.0);
        Triangle { v1, v2, v3, normal: normalized_normal }
    }

    pub fn normal(&self) -> Vector4f {
        self.normal
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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

    fn almost_equal(lhs: Vector4f, rhs: Vector4f, tolerance: f64) -> bool {
        return
            float_cmp(lhs.x(), rhs.x(), tolerance) &&
                float_cmp(lhs.y(), rhs.y(), tolerance) &&
                float_cmp(lhs.z(), rhs.z(), tolerance);
    }

    #[test]
    fn test_normal_for_right_hand_side_triangle() {
        let v1 = Vertex4f::new(0.0, 0.0, 0.0, 1.0);
        let v2 = Vertex4f::new(1.0, 0.0, 0.0, 1.0);
        let v3 = Vertex4f::new(1.0, 1.0, 0.0, 1.0);
        let triangle = Triangle::new(v1, v2, v3);
        let exptected_normal = Vector4f::new(0.0, 0.0, 1.0, 1.0);
        assert!(almost_equal(exptected_normal, triangle.normal(), 1E-8));
    }

    #[test]
    fn test_normal_for_left_hand_side_triangle() {
        let v1 = Vertex4f::new(0.0, 0.0, 0.0, 1.0);
        let v2 = Vertex4f::new(1.0, 0.0, 0.0, 1.0);
        let v3 = Vertex4f::new(1.0, 1.0, 0.0, 1.0);
        let triangle = Triangle::new(v1, v3, v2);
        let exptected_normal = Vector4f::new(0.0, 0.0, -1.0, 1.0);
        assert!(almost_equal(exptected_normal, triangle.normal(), 1E-8));
    }
}