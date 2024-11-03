use cube::Cube;
use nalgebra::Matrix4;
use triangle::Triangle;

pub mod cube;
pub mod hexagon;
pub mod rectangle;
pub mod sphere;
pub mod triangle;
#[derive(Debug)]

pub enum Geometry {
    Triangle(Triangle),
    Cube(Cube),
}

impl Geometry {
    /// Returns the vertex buffer data as a byte slice for the current geometry
    pub fn vertex_buffer_data(&self) -> &Vec<crate::ecs::components::vertex::Vertex> {
        match self {
            Geometry::Triangle(triangle) => triangle.vertices(),
            Geometry::Cube(cube) => cube.vertices(),
        }
    }

    /// Returns the index buffer data as a byte slice for the current geometry
    pub fn index_buffer_data(&self) -> &[u16] {
        match self {
            Geometry::Triangle(triangle) => triangle.indices(),
            Geometry::Cube(cube) => cube.indices(),
        }
    }

    /// Returns the number of indices in the current geometry
    pub fn num_indices(&self) -> u32 {
        match self {
            Geometry::Triangle(triangle) => triangle.indices().len() as u32,
            Geometry::Cube(cube) => cube.indices().len() as u32,
        }
    }

    /// Returns the model matrix of the current geometry
    pub fn model_matrix(&self) -> Matrix4<f32> {
        match self {
            Geometry::Triangle(triangle) => triangle.model_matrix(),
            Geometry::Cube(cube) => cube.model_matrix(),
        }
    }
}
