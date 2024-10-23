use crate::{math::Mat4, traits::buffers::UniformBuffer};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct GlobalUniforms {
    pub view: [[f32; 4]; 4],       // Camera view matrix
    pub projection: [[f32; 4]; 4], // Camera projection matrix
    pub model: [[f32; 4]; 4],      // Model transformation matrix (optional)
}

impl UniformBuffer for GlobalUniforms {}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct ModelUniforms {
    pub(crate) transform: Mat4,
}

impl UniformBuffer for ModelUniforms {}
