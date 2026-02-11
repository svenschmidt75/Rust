use crate::vertex::Vertex4;

pub fn dot_product(v1: Vertex4, v2: Vertex4) -> f32 {
    // SS: the dot product in only defined for vectors, so
    // we ignore the w component
    let x = v1[0] * v2[0];
    let y = v1[1] * v2[1];
    let z = v1[2] * v2[2];
    x + y + z
}

pub fn cross_product(v1: Vertex4, v2: Vertex4) -> Vertex4 {
    // SS: the cross product in only defined for vectors, so
    // we ignore the w component
    let cp0 = v1[1] * v2[2] - v1[2] * v2[1];
    let cp1 = v1[2] * v2[0] - v1[0] * v2[2];
    let cp2 = v1[0] * v2[1] - v1[1] * v2[0];
    Vertex4::new(cp0, cp1, cp2, 0.0)
}
