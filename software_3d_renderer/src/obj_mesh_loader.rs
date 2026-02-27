use crate::raster_vertex::RasterVertex;
use crate::texture_type::{Color, TextureType};
use crate::triangle::Triangle;
use crate::vertex4::Vertex4;

pub fn load(data: &[&str]) -> Vec<Triangle> {
    // SS: read vertex data
    let vertices = data
        .iter()
        .filter(|line| line.starts_with("v "))
        .map(|line| {
            // SS: face line
            let parts: Vec<&str> = line.split_whitespace().collect();
            let vertices = [
                parts[1].parse::<f32>().unwrap(),
                parts[2].parse::<f32>().unwrap(),
                parts[3].parse::<f32>().unwrap(),
            ];
            let vertex = Vertex4::new_vertex(vertices[0], vertices[1], vertices[2]);
            RasterVertex::new(vertex, Color::new(0, 0, 255, 255), [0.0, 0.0])
        })
        .collect::<Vec<_>>();

    // SS: read faces (triangles)
    let triangles = data
        .iter()
        .filter(|line| line.starts_with("f "))
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let vertex_indices = [
                parts[1].parse::<usize>().unwrap() - 1,
                parts[2].parse::<usize>().unwrap() - 1,
                parts[3].parse::<usize>().unwrap() - 1,
            ];
            let triangle_vertices = [
                vertices[vertex_indices[0]],
                vertices[vertex_indices[1]],
                vertices[vertex_indices[2]],
            ];
            Triangle::new(triangle_vertices, TextureType::None)
        })
        .collect::<Vec<_>>();
    triangles
}
