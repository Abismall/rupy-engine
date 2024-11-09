use cube::Cube;
use nalgebra::Matrix4;
use serde::{Deserialize, Serialize};
use triangle::Triangle;

pub mod cube;
pub mod hexagon;
pub mod rectangle;
pub mod sphere;
pub mod triangle;
#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum Geometry {
    Triangle(Triangle),
    Cube(Cube),
}

impl Geometry {
    pub fn vertex_buffer_data(&self) -> &Vec<crate::ecs::components::vertex::Vertex> {
        match self {
            Geometry::Triangle(triangle) => triangle.vertices(),
            Geometry::Cube(cube) => cube.vertices(),
        }
    }

    pub fn index_buffer_data(&self) -> &[u16] {
        match self {
            Geometry::Triangle(triangle) => triangle.indices(),
            Geometry::Cube(cube) => cube.indices(),
        }
    }

    pub fn num_indices(&self) -> u32 {
        match self {
            Geometry::Triangle(triangle) => triangle.indices().len() as u32,
            Geometry::Cube(cube) => cube.indices().len() as u32,
        }
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        match self {
            Geometry::Triangle(triangle) => triangle.model_matrix(),
            Geometry::Cube(cube) => cube.model_matrix(),
        }
    }
}
