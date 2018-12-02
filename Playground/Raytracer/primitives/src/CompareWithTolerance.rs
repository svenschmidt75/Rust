use Vertex4f::Vertex4f;

pub trait CompareWithTolerance {
	fn cmp(self, other: Vertex4f, tol: f64) -> bool;
}
