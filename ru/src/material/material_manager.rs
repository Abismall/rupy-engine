use std::{collections::HashMap, sync::Arc};

use crate::{
    geometry::{Shape, Uniforms},
    log_error,
    material::GeometricHasher,
    render::pipeline::create_bind_group,
};
use nalgebra::Matrix4;
use wgpu::{util::DeviceExt, BufferUsages, Device};

use super::{color::Color, Material};

pub struct MaterialManager {
    materials: HashMap<u64, Material>,
    bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_texture_view: wgpu::TextureView,
}
impl MaterialManager {
    pub fn get_mut(&mut self, material_hash: u64) -> Option<&mut Material> {
        self.materials.get_mut(&material_hash)
    }
}

impl MaterialManager {
    pub fn new(device: &Arc<Device>, surface_config: &wgpu::SurfaceConfiguration) -> Self {
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
            depth_texture,
            depth_texture_view,
        }
    }
}

impl MaterialManager {
    pub fn get(&mut self, material_hash: u64) -> &Material {
        match self.materials.get(&material_hash) {
            Some(material) => material,
            None => panic!("Could not locate a requested material: {}", material_hash),
        }
    }
    pub fn create_or_update_buffers(&mut self, geometry: &Shape, material_hash: &u64) {
        if let Some(material) = self.materials.get_mut(&material_hash) {
            material
                .geometry
                .set_vertex_buffer(geometry.vertex_buffer());
            material.geometry.set_index_buffer(geometry.index_buffer());
        }
    }
    pub fn update_buffers(&mut self, geometry: &Shape, material_hash: u64) {
        if let Some(material) = self.materials.get_mut(&material_hash) {
            material
                .geometry
                .set_vertex_buffer(geometry.vertex_buffer());
            material.geometry.set_index_buffer(geometry.index_buffer());
        }
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
}

impl MaterialManager {
    pub fn create_material(
        &mut self,
        device: &wgpu::Device,
        geometry: Shape,
        initial_color: crate::material::Color,
    ) -> u64 {
        let material_hash = GeometricHasher::hash(&geometry);

        let model_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let uniforms = Uniforms {
            model: model_matrix,
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

        self.create_or_update_buffers(&geometry, &material_hash);
        self.materials.insert(
            material_hash,
            Material::new(
                create_bind_group(&device, &self.bind_group_layout, &uniform_buffer),
                uniform_buffer,
                geometry,
                initial_color,
            ),
        );
        material_hash
    }
    pub fn update_material_uniforms(
        &self,
        material_hash: u64,
        queue: &wgpu::Queue,
        model_matrix: Matrix4<f32>,
        view_proj_matrix: Matrix4<f32>,
        color: Option<super::color::Color>,
    ) {
        if let Some(material) = self.materials.get(&material_hash) {
            let uniforms = match color {
                Some(color) => {
                    Uniforms {
                        model: model_matrix.into(),
                        view_proj: view_proj_matrix.into(),
                        color: [color.r, color.g, color.b, color.a],
                    };
                }
                None => {
                    Uniforms {
                        model: model_matrix.into(),
                        view_proj: view_proj_matrix.into(),
                        color: [
                            material.color.r,
                            material.color.g,
                            material.color.b,
                            material.color.a,
                        ],
                    };
                }
            };

            queue.write_buffer(
                &material.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );
        }
    }
    pub fn get_material_color(&self, material_hash: u64) -> Color {
        if let Some(material) = self.materials.get(&material_hash) {
            return material.color;
        }
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn set_material_color(
        &mut self,
        material_hash: u64,
        new_color: Color,
        queue: &wgpu::Queue,
        view_proj_matrix: Matrix4<f32>,
    ) {
        if let Some(material) = self.materials.get_mut(&material_hash) {
            material.set_color(new_color);

            let model_matrix = material.geometry.model_matrix();

            let uniforms = Uniforms {
                model: model_matrix.into(),
                view_proj: view_proj_matrix.into(),
                color: [new_color.r, new_color.g, new_color.b, new_color.a],
            };

            queue.write_buffer(
                &material.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );
        }
    }
}

impl MaterialManager {
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        material_hash: u64,
        view_proj_matrix: Matrix4<f32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        pipeline: &wgpu::RenderPipeline,
    ) {
        if let Some(material) = self.materials.get(&material_hash) {
            self.update_material_uniforms(
                material_hash,
                queue,
                material.geometry.model_matrix(),
                view_proj_matrix,
                Some(material.color),
            );

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
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

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&material.geometry.vertex_buffer()),
                usage: BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&material.geometry.vertex_buffer()),
                usage: BufferUsages::INDEX,
            });
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &material.bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..material.geometry.num_indices(), 0, 0..1);
        } else {
            log_error!("Material with hash: {} not found", material_hash);
        }
    }
}
