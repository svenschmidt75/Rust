use Matrix4f::Matrix4f;
use Vertex4f::Vertex4f;


pub fn translate(m: Matrix4f, v: Vertex4f) -> Vertex4f {
	m * v
}
