use crate::object::buffer::BufferManager;
use crate::object::manager::ObjectManager;
use crate::object::object::Object;
use crate::pipeline::manager::PipelineManager;
use crate::render::command::RenderCommand;
use crate::render::traits::Renderable;
use crate::scene::description::CameraDescription;
use crate::{log_error, render::layout::BindGroupLayoutManager};
use std::sync::{Arc, RwLock};

use super::description::SceneDescription;
#[derive(Debug)]

pub struct Scene {
    pub objects: Arc<RwLock<ObjectManager>>,
    pub desc: SceneDescription,
}

impl Scene {
    pub fn with_render_export<F>(&self, mut f: F)
    where
        F: FnMut(&[Object], &CameraDescription),
    {
        let object_manager = self.objects.read().unwrap();

        f(object_manager.get_render_data(), &self.desc.camera);
    }

    pub fn update_model_matrices(&self) {
        let mut object_manager = self.objects.write().unwrap();
        object_manager.update_object_model_matrices();
    }

    pub fn add_object(&self, object: Object) {
        let mut object_manager = self.objects.write().unwrap();
        object_manager.add_object(object);
    }

    pub fn remove_object(&self, index: usize) {
        let mut object_manager = self.objects.write().unwrap();
        object_manager.remove_object(index);
    }

    pub fn get_object(&self, index: usize) -> Option<Object> {
        let object_manager = self.objects.read().unwrap();
        object_manager.get_object(index).cloned()
    }
    pub fn populate(&mut self) {
        let mut obj_lock = self.objects.write().unwrap();

        for obj_desc in &self.desc.objects {
            let object = Object::new(
                obj_desc.position,
                obj_desc.rotation,
                obj_desc.scale,
                Arc::clone(&obj_desc.shape),
                obj_desc.bind_group_key,
                obj_desc.bind_group_layout_key,
                obj_desc.vertex_buffer_key,
                obj_desc.index_buffer_key,
            );

            obj_lock.add_object(object);
        }
    }
    pub fn build_render_commands(
        &self,
        device: &wgpu::Device,
        pipelines: Arc<RwLock<PipelineManager>>,
        buffers: Arc<RwLock<BufferManager>>,
        layouts: Arc<RwLock<BindGroupLayoutManager>>,
        swapchain_format: wgpu::TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        depth_enabled: bool,
    ) -> Vec<RenderCommand> {
        let mut pipeline_manager = pipelines.write().unwrap();
        let mut buffer_manager = buffers.write().unwrap();
        let layout_manager = layouts.read().unwrap();

        let mut render_commands = Vec::new();

        self.with_render_export(|objects, _camera| {
            for object in objects {
                if let Some(layout_entry) = layout_manager.get_layouts(object.bind_group_layout_key)
                {
                    render_commands.push(RenderCommand {
                        vertex_buffer: Arc::clone(&buffer_manager.create_or_get_buffer(
                            object.vertex_buffer_key,
                            device,
                            object.vertex_buffer_data().len() as wgpu::BufferAddress,
                        )),
                        index_buffer: Arc::clone(&buffer_manager.create_or_get_buffer(
                            object.index_buffer_key,
                            device,
                            object.index_buffer_data().len() as wgpu::BufferAddress,
                        )),
                        index_count: object.num_indices(),
                        pipeline: Arc::clone(&pipeline_manager.get_or_create_pipeline(
                            device,
                            object.bind_group_key,
                            swapchain_format,
                            vertex_shader_src,
                            fragment_shader_src,
                            object.bind_group_layout_key,
                            depth_enabled,
                        )),
                        bind_group: Arc::clone(&pipeline_manager.get_or_create_bind_group(
                            device,
                            &layout_entry.layers,
                            object.bind_group_layout_key,
                        )),
                    });
                } else {
                    log_error!("Failed to retrieve shared resources for object");
                }
            }
        });

        render_commands
    }
}

pub struct SceneFactory;

impl SceneFactory {
    pub fn create_scene(desc: SceneDescription, objects: Arc<RwLock<ObjectManager>>) -> Scene {
        Scene { objects, desc }
    }
}
