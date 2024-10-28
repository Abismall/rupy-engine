use crate::{
    math::{mat4_id, Mat4},
    traits::buffers::UniformBuffer,
};
use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, Device};

/// Represents the uniform data for an individual object's model transformation.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ObjectUniforms {
    /// Model transformation matrix of the object.
    pub model: Mat4,
}
impl ObjectUniforms {
    // Initializes the buffer directly through the UniformBuffer trait
    pub fn create_buffer(device: &Device) -> Buffer {
        Self::create_static_uniform_buffer(device, &Self::default())
    }
}
impl Default for ObjectUniforms {
    fn default() -> Self {
        Self { model: mat4_id() }
    }
}

impl UniformBuffer for ObjectUniforms {}

// Ensure `CameraUniforms` derives Pod and Zeroable
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct CameraUniforms {
    pub view_projection_matrix: [[f32; 4]; 4],
}

impl CameraUniforms {
    // Initializes the buffer directly through the UniformBuffer trait
    pub fn create_buffer(device: &Device) -> Buffer {
        Self::create_static_uniform_buffer(device, &Self::default())
    }
}

impl UniformBuffer for CameraUniforms {}

impl Default for CameraUniforms {
    fn default() -> Self {
        Self {
            view_projection_matrix: mat4_id(),
        }
    }
}

impl CameraUniforms {
    /// Creates a new `CameraUniforms` with the given view-projection matrix.
    ///
    /// # Parameters
    /// - `view_projection_matrix`: The matrix combining the camera's view and projection transformations.
    ///
    /// # Returns
    /// A new instance of `CameraUniforms` with the specified view-projection matrix.
    pub fn new(view_projection_matrix: Mat4) -> Self {
        Self {
            view_projection_matrix,
        }
    }
}
/// Represents the transformation of an object in 3D space, including position, rotation, and scale.
#[derive(Debug, Clone)]
pub struct Transform {
    /// Position of the object in 3D space.
    pub position: [f32; 3],
    /// Rotation of the object in 3D space.
    pub rotation: [f32; 3],
    /// Scale of the object in 3D space.
    pub scale: [f32; 3],
}
