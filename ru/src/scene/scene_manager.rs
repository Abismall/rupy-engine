use crate::application::state::RenderMode;
use crate::geometry::{Shape, Uniforms};
use crate::material::material_manager::MaterialManager;
use crate::math::create_translation_matrix;
use crate::render::command::{
    create_index_buffer, create_vertex_buffer, RenderCommand, RenderCommandBuffer,
};
use crate::render::pipeline::{PipelineManager, DEFAULT_PIPELINE, OUTLINE_PIPELINE};
use nalgebra::Matrix4;
use nalgebra::Vector3;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct SceneObject {
    pub position: Vector3<f32>,
    pub material_hash: u64,
    pub bounding_box: Option<BoundingBox>,
    pub draw_outline: bool,
    pub disable_render: bool,
}

pub struct SceneManager {
    objects: HashMap<u64, SceneObject>,
    pub materials: MaterialManager,
    pub pipeline_manager: Arc<RwLock<PipelineManager>>,
    next_object_id: u64,
    pub debug_mode: RenderMode,
}

impl SceneManager {
    pub fn new(materials: MaterialManager, pipeline_manager: Arc<RwLock<PipelineManager>>) -> Self {
        Self {
            objects: HashMap::new(),
            materials,
            pipeline_manager,
            next_object_id: 0,
            debug_mode: RenderMode::Normal,
        }
    }

    pub fn populate_command_buffer(
        &mut self,
        command_buffer: &mut RenderCommandBuffer,
        view_proj_matrix: &Matrix4<f32>,
        queue: Arc<&wgpu::Queue>,
        device: Arc<&wgpu::Device>,
    ) {
        for object in self.objects.values() {
            if object.disable_render {
                continue;
            }
            if let Some(material) = self.materials.get_mut(object.material_hash) {
                let model_matrix = create_translation_matrix(object.position);

                let uniforms = Uniforms {
                    model: model_matrix.into(),
                    view_proj: (*view_proj_matrix).into(),
                    color: [
                        material.color.r,
                        material.color.g,
                        material.color.b,
                        material.color.a,
                    ],
                };
                queue.write_buffer(
                    &material.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[uniforms]),
                );

                let pipeline_manager = self.pipeline_manager.read().unwrap();

                let pipeline_option = match self.debug_mode {
                    RenderMode::Normal => pipeline_manager.get_pipeline(DEFAULT_PIPELINE),
                    RenderMode::OutlineOnly => pipeline_manager.get_pipeline(OUTLINE_PIPELINE),
                    _ => {
                        continue;
                    }
                };
                let vertex_buffer =
                    create_vertex_buffer(&device, material.geometry.vertex_buffer_data());
                let index_buffer = create_index_buffer(
                    &device,
                    bytemuck::cast_slice(material.geometry.index_buffer_data()),
                );
                if let Some(pipeline) = pipeline_option {
                    command_buffer.push(RenderCommand {
                        pipeline,
                        uniform_data: Arc::clone(&material.bind_group),
                        vertex_buffer: Arc::new(vertex_buffer),
                        index_buffer: Arc::new(index_buffer),
                        index_count: material.geometry.num_indices(),
                    });
                }
            }
        }
    }

    pub fn add_object(
        &mut self,
        position: Vector3<f32>,
        geometry: Shape,
        initial_color: crate::material::color::Color,
        bounding_box: Option<BoundingBox>,
        device: &wgpu::Device,
    ) -> u64 {
        let material_hash = self
            .materials
            .create_material(device, geometry, initial_color);
        let object_id = self.next_object_id;
        self.next_object_id += 1;

        self.objects.insert(
            object_id,
            SceneObject {
                position,
                material_hash,
                bounding_box,
                draw_outline: false,
                disable_render: false,
            },
        );
        object_id
    }

    pub fn set_debug_mode(&mut self, mode: RenderMode) {
        self.debug_mode = mode;
    }

    pub fn set_debug_mode_for_all(&mut self, mode: RenderMode) {
        self.set_debug_mode(mode);

        for object in self.objects.values_mut() {
            object.draw_outline = mode != RenderMode::Normal;
        }
    }
}
