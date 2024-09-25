pub(crate) mod spatial;
pub(crate) mod traits;
pub(crate) mod triangle;
use nalgebra::Matrix4;
use triangle::ShadedTriangleStructure;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Uniforms {
    pub model: [[f32; 4]; 4],     // Model matrix
    pub view_proj: [[f32; 4]; 4], // View-projection matrix
    pub color: [f32; 4],          // RGBA color
}
#[derive(Debug)]

pub enum Geometry {
    ShadedTriangle(ShadedTriangleStructure),
    // Add more geometries here in the future, like Sphere, Cube, etc.
}

impl Geometry {
    /// Returns the vertex buffer data as a byte slice for the current geometry
    pub fn vertex_buffer_data(&self) -> &[u8] {
        match self {
            Geometry::ShadedTriangle(shaded) => {
                // Use bytemuck to convert the vertices into a &[u8] slice
                bytemuck::cast_slice(shaded.vertices())
            }
        }
    }

    /// Returns the index buffer data as a byte slice for the current geometry
    pub fn index_buffer_data(&self) -> &[u8] {
        match self {
            Geometry::ShadedTriangle(shaded) => bytemuck::cast_slice(shaded.indices()),
        }
    }

    /// Returns the number of indices in the current geometry
    pub fn num_indices(&self) -> u32 {
        match self {
            Geometry::ShadedTriangle(shaded) => shaded.indices().len() as u32,
        }
    }

    /// Returns the model matrix of the current geometry
    pub fn model_matrix(&self) -> Matrix4<f32> {
        match self {
            Geometry::ShadedTriangle(shaded) => shaded.model_matrix(),
        }
    }
}
