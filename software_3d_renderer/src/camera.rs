use std::panic::Location;
use crate::vertex::Vertex;

pub struct Camera {
    location: Vertex,
    direction: Vertex,
    up: Vertex,
}

impl Camera {
    pub fn new(location: Vertex, direction: Vertex, up: Vertex) -> Self {
        Self {
            location,
            direction,
            up,
        }
    }

    pub fn world_to_camera(&self, vertex: Vertex) -> Vertex {
        // SS: translate

        // SS: rotate
        vertex
    }

}