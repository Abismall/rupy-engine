use cube::Cube;
use nalgebra::Matrix4;
use serde::{Deserialize, Serialize};
use triangle::Triangle;

use crate::ecs::components::model::{Vertex2D, Vertex3D};

pub mod cube;
pub mod hexagon;
pub mod rectangle;
pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone, Deserialize)]
pub enum GeometryId {
    Triangle,
    Cube,
}
impl Serialize for GeometryId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Ok(match self {
            GeometryId::Triangle => serializer.collect_str("Triangle")?,
            GeometryId::Cube => serializer.collect_str("Cube")?,
        })
    }
}
pub enum VertexType {
    Vertex2D(Vec<Vertex2D>),
    Vertex3D(Vec<Vertex3D>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum Geometry {
    Triangle(Triangle),
    Cube(Cube),
}

impl Geometry {
    pub fn vertices(&self) -> VertexType {
        match self {
            Geometry::Triangle(triangle) => VertexType::Vertex2D(triangle.vertices().to_vec()),
            Geometry::Cube(cube) => VertexType::Vertex3D(cube.vertices().to_vec()),
        }
    }

    pub fn indices(&self) -> &[u16] {
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
