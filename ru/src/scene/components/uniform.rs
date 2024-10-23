use crate::math::{mat4_id, Mat4};
use bytemuck::{Pod, Zeroable};
pub const GLOBAL_UNIFORMS_CACHE_KEY: &str = "GlobalUniforms";
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub model: Mat4,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct GlobalUniforms {
    pub view_projection_matrix: Mat4,
}
impl Default for GlobalUniforms {
    fn default() -> Self {
        Self {
            view_projection_matrix: mat4_id(),
        }
    }
}
impl GlobalUniforms {
    pub fn new(view_projection_matrix: Mat4) -> Self {
        Self {
            view_projection_matrix,
        }
    }
}
