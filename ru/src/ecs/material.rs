use naga::Handle;
use wgpu::Texture;

use crate::{
    prelude::{Quat, Vec3},
    shader::shader::Shader,
};

pub struct Material {
    albedo_texture: Handle<Texture>,
    normal_map: Option<Handle<Texture>>,
    shader_program: Handle<Shader>,
    properties: MaterialProperties,
}

pub struct MaterialProperties {
    shininess: f32,
    roughness: f32,
    metallic: f32,
}

pub struct Transform {
    position: Vec3,
    rotation: Quat<f32>,
    scale: Vec3,
}
