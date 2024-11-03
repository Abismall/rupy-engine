use crate::ecs::components::uniform::{ColorUniform, ModelUniform, Uniforms, ViewProjectionMatrix};
use crate::log_error;
use crate::pipeline::cache::PipelineManager;
use crate::shape::Geometry;
use crate::{
    core::error::AppError,
    graphics::{
        binding::{uniform_bind_group, uniform_bind_group_layout},
        buffer::{index_buffer, uniform_buffer, vertex_buffer},
    },
    texture::library::TextureFileCache,
};
use nalgebra::Matrix4;
use std::{collections::HashMap, sync::Arc};
use wgpu::{Device, Origin3d, TextureAspect, TextureDimension, TextureFormat};

use super::Material;
pub struct MaterialManager {
    pub materials: HashMap<String, Material>,
    pub pipelines: Arc<PipelineManager>,
    device: Arc<wgpu::Device>,
}

impl MaterialManager {
    pub fn new(pipelines: Arc<PipelineManager>, device: &Arc<Device>) -> Self {
        Self {
            materials: HashMap::new(),
            pipelines,
            device: device.clone(),
        }
    }

    pub fn materials(&self) -> &HashMap<String, Material> {
        &self.materials
    }

    pub fn create_material(
        &mut self,
        geometry: Geometry,
        view_proj: Matrix4<f32>,
        initial_color: [f32; 4],
        name: String,
        texture_bind_group: Option<Arc<wgpu::BindGroup>>,
    ) -> Result<(), AppError> {
        let device = &self.device;
        let model_matrix = geometry.model_matrix();

        let vertex_buffer = vertex_buffer(device, geometry.vertex_buffer_data());

        let index_buffer = index_buffer(device, geometry.index_buffer_data());

        let uniform_buffer = uniform_buffer(
            device,
            &Uniforms {
                model: ModelUniform {
                    matrix: model_matrix.into(),
                },
                color: ColorUniform {
                    rgba: [
                        initial_color[0] as f32,
                        initial_color[1] as f32,
                        initial_color[2] as f32,
                        initial_color[3] as f32,
                    ],
                },
                view_projection: ViewProjectionMatrix {
                    matrix: view_proj.into(),
                },
            },
        );

        let uniform_bind_group =
            uniform_bind_group(device, &uniform_buffer, &uniform_bind_group_layout(device));

        self.materials.insert(
            name,
            Material::new(
                uniform_bind_group,
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                geometry,
                initial_color,
                texture_bind_group,
            ),
        );
        Ok(())
    }
    pub fn set_material_texture(
        &mut self,
        material_name: &str,
        path: &str,
        texture_cache: &mut TextureFileCache,
        format: TextureFormat,
        dimension: TextureDimension,
        mip_level_count: u32,
        depth_or_array_layers: u32,
        sample_count: u32,
        origin: Origin3d,
        aspect: TextureAspect,
        mip_level: u32,
        offset: u64,
    ) {
        if let Some(material) = self.materials.get_mut(material_name) {
            let (_, texture_bind_group) = match texture_cache.get_or_load_texture(
                path,
                format,
                dimension,
                mip_level_count,
                depth_or_array_layers,
                sample_count,
                origin,
                aspect,
                mip_level,
                offset,
            ) {
                Ok(data) => data,
                Err(e) => {
                    log_error!("Failed to load texture: {:?}", e);
                    return;
                }
            };
            material.texture_bind_group = Some(texture_bind_group);
        }
    }

    pub fn set_material_color(
        &mut self,
        material_name: &str,
        new_color: [f32; 4],
        queue: &wgpu::Queue,
        view_proj: Matrix4<f32>,
    ) {
        if let Some(material) = self.materials.get_mut(material_name) {
            material.set_color(new_color.into());

            let model_matrix = material.geometry.model_matrix();

            let uniforms = Uniforms {
                model: ModelUniform {
                    matrix: model_matrix.into(),
                },
                color: ColorUniform {
                    rgba: [new_color[0], new_color[1], new_color[2], new_color[3]],
                },
                view_projection: ViewProjectionMatrix {
                    matrix: view_proj.into(),
                },
            };
            queue.write_buffer(
                &material.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );
        }
    }
}
