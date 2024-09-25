use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    geometry::{Geometry, Uniforms},
    log_debug,
    render::pipeline_manager::PipelineManager,
};
use nalgebra::Matrix4;
use wgpu::{util::DeviceExt, Device, RenderPipeline, TextureFormat};

use super::Material;

pub struct MaterialManager {
    materials: HashMap<String, Material>,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline_manager: Arc<RwLock<PipelineManager>>, // Use Arc<RwLock> for shared access
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_texture_view: wgpu::TextureView,
}

impl MaterialManager {
    pub fn new(
        device: &Arc<Device>,
        texture_format: TextureFormat,
        pipeline_manager: Arc<RwLock<PipelineManager>>,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let (depth_texture, depth_texture_view) =
            MaterialManager::create_depth_textures(surface_config, device);

        Self {
            materials: HashMap::new(),
            bind_group_layout,
            pipeline_manager, // Store Arc<RwLock<PipelineManager>>
            depth_texture,
            depth_texture_view,
        }
    }
    fn create_depth_textures(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &Arc<Device>,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        (depth_texture, depth_texture_view)
    }
    pub fn resize_depth_texture(
        &mut self,
        device: &Arc<Device>,
        surface_config: &wgpu::SurfaceConfiguration,
    ) {
        let (depth_texture, depth_texture_view) =
            MaterialManager::create_depth_textures(surface_config, device);
        self.depth_texture = depth_texture;
        self.depth_texture_view = depth_texture_view;
        log_debug!(
            "Depth texture resized to: {}x{}",
            surface_config.width,
            surface_config.height
        );
    }
    pub fn create_material(
        &mut self,
        device: &wgpu::Device,
        geometry: Geometry,
        initial_color: crate::material::Color,
        name: String,
    ) {
        // Now we proceed to create buffers for the material
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: geometry.vertex_buffer_data(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: geometry.index_buffer_data(),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Initialize model matrix as identity
        let model_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let uniforms = Uniforms {
            model: model_matrix, // Add model matrix here
            view_proj: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            color: [
                initial_color.r,
                initial_color.g,
                initial_color.b,
                initial_color.a,
            ],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        // Create the Material, now that the pipeline is fully owned and cloned
        self.materials.insert(
            name,
            Material::new(
                uniform_bind_group,
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                geometry,
                initial_color,
            ),
        );
        log_debug!("Available materials: {:?}", self.materials.keys());
    }
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,

        view: &wgpu::TextureView,
        material_name: &str,
        view_proj_matrix: Matrix4<f32>, // Accept the view-projection matrix
        queue: &wgpu::Queue,
        pipeline: &RenderPipeline,
    ) {
        log_debug!("Searching Material {} ", material_name);
        if let Some(material) = self.materials.get(material_name) {
            log_debug!("Material {} found", material_name);
            let model_matrix = material.geometry.model_matrix();
            let uniforms = Uniforms {
                model: model_matrix.into(),
                view_proj: view_proj_matrix.into(),
                color: material.color.into(),
            };
            log_debug!("Found uniforms: {:?}", uniforms);
            queue.write_buffer(
                &material.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, // Set to a brighter color like white or light gray
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Default::default(),
                }),
                timestamp_writes: Default::default(),
                occlusion_query_set: Default::default(),
            });

            // Set the pipeline, bind groups, and buffers
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &material.bind_group, &[]);

            // Set the vertex buffer
            render_pass.set_vertex_buffer(0, material.vertex_buffer.slice(..));

            // Set the index buffer and draw
            render_pass
                .set_index_buffer(material.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..material.geometry.num_indices(), 0, 0..1);
        } else {
            log_debug!("Material {} not found", material_name); // You could add this log for debugging
        }
    }
    pub fn set_material_color(
        &mut self,
        material_name: &str,
        new_color: super::color::Color,
        queue: &wgpu::Queue,
        view_proj: [[f32; 4]; 4], // Pass the view-projection matrix as a parameter
    ) {
        if let Some(material) = self.materials.get_mut(material_name) {
            // Update the color of the material
            material.set_color(new_color.into());

            // Use the actual model matrix (assumed to be part of the material or object state)
            let model_matrix = material.geometry.model_matrix(); // Get model matrix from the material or state

            // Prepare the updated uniforms with the actual model and view_proj matrices
            let uniforms = Uniforms {
                model: model_matrix.into(), // Use the material's model matrix
                view_proj,                  // Use the provided view-projection matrix
                color: [new_color.r, new_color.g, new_color.b, new_color.a], // Set the new color
            };

            // Write the updated uniform buffer data to the GPU
            queue.write_buffer(
                &material.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );
        }
    }
}
