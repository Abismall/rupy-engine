use wgpu::{BindGroup, ShaderModule};

use crate::log_debug;
use crate::math::Vec3;
use crate::object::object::Object;
use crate::pipeline::cache::PipelineCache;
use crate::render::Renderable;

use super::SceneDescription;

pub struct Scene {
    objects: Vec<Object>,  // Non-generic Object
    camera_position: Vec3, // Camera position
}

impl Scene {
    // Create a scene from a SceneDescription
    pub fn from_description(description: SceneDescription) -> Self {
        let objects = description
            .objects
            .into_iter()
            .map(|desc| {
                Object::new(
                    desc.position,
                    desc.rotation,
                    desc.scale,
                    desc.mesh,
                    desc.pipeline_cache_key,
                )
            })
            .collect();

        Scene {
            objects,
            camera_position: description.camera_position,
        }
    }

    pub fn render_scene_objects(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        swapchain_format: wgpu::TextureFormat,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        global_bind_group: &wgpu::BindGroup,
    ) {
        log_debug!("Rendering method has been called");
        let device = device;
        // Iterate over and render each object
        for object in &mut self.objects {
            object.mesh.update(); // Update the model matrix, etc.
            object.render(
                device,
                pipeline_cache,
                swapchain_format,
                vertex_shader_src,
                fragment_shader_src,
                encoder,
                output_view,
                global_bind_group,
            );
        }
    }
}
