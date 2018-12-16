#![allow(non_snake_case)]

extern crate rand;

// export these modules from the library
pub mod Vertex4f;
pub mod Vector4f;
pub mod Ray;
pub mod Triangle;
pub mod Color;
pub mod Sphere;
pub mod Shape;
pub mod ShapeList;
pub mod Hit;
pub mod Camera;
pub mod Material;
pub mod Lambertian;

mod operations;
mod Matrix4f;
mod CompareWithTolerance;
