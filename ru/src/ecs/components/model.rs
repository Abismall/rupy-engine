use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::shape::Geometry;

use super::uniform::UniformColor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub geometry: Geometry,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Material {
    pub name: String,
    pub color: UniformColor,
    pub texture_id: Option<u64>, // ID to look up in a texture manager
}
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Texture {
    pub file_path: String,
    #[serde(skip)]
    pub bind_group: Option<Arc<wgpu::BindGroup>>,
    #[serde(skip)]
    pub file: Option<Arc<wgpu::Texture>>,
}
