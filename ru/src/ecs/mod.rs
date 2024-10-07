use std::collections::HashMap;

use naga::Handle;
use wgpu::Texture;

use crate::{
    prelude::{Mat4, Vec3},
    shader::shader::Shader,
};
pub mod components;
pub mod material;
pub mod mesh;
pub mod systems;
pub mod vertex;
pub mod world;

pub struct Light {
    color: Vec3,
    intensity: f32,
    light_type: LightType, // Enum for point, directional, spot
}

pub enum LightType {
    Directional { direction: Vec3 },
    Point { radius: f32 },
    Spot { direction: Vec3, angle: f32 },
}
pub struct ShadowMap {
    depth_texture: Handle<Texture>,
    // framebuffer: Framebuffer,
    view_projection_matrix: Mat4,
}
pub struct Renderer {
    // frame_buffer: Framebuffer,
    // shadow_framebuffer: Framebuffer, WIP
    shaders: HashMap<String, Shader>,
    global_uniforms: GlobalUniforms,
}

pub struct GlobalUniforms {
    view_projection_matrix: Mat4,
    camera_position: Vec3,
    lights: Vec<LightUniform>, // Packed lights for shader
}

pub struct LightUniform {
    position: Vec3,
    color: Vec3,
    light_type: u32, // Encoded for use in shaders
}
