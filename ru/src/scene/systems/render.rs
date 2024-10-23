use wgpu::util::DeviceExt;

use crate::{
    core::error::AppError,
    model::{surface::SurfaceWrapper, window::WindowWrapper},
    scene::{
        components::{material::Material, resources::RenderResources, uniform::Uniforms},
        entities::models::Entity,
    },
};

use super::world::World;

pub struct RenderSystem {
    pub surface: Option<SurfaceWrapper>,
    pub window: WindowWrapper,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            surface: None,
            window: WindowWrapper::new(),
        }
    }

    // pub async fn run(&mut self, world: &World) -> std::result::Result<(), AppError> {
    //     log_debug!("Render system running!");

    //     // Ensure surface exists
    //     let wrapper = match &self.surface {
    //         Some(s) => s,
    //         None => {
    //             log_debug!(
    //                 "Error: {:?}",
    //                 AppError::NoSurfaceError(String::from("No value for surface"))
    //             );
    //             return Err(AppError::NoSurfaceError(String::from(
    //                 "No value for surface",
    //             )));
    //         }
    //     };

    //     // Get current frame
    //     let frame = wrapper.get_current_frame(&self.device)?;
    //     let view = wrapper.get_current_view(&frame)?;

    //     // Update global uniforms if needed
    //     self.queue.write_buffer(
    //         &self.global_uniform_buffer,
    //         0,
    //         bytemuck::bytes_of(&self.global_uniforms),
    //     );
    //     log_debug!("Updated global uniform buffer!");

    //     // Create a single command encoder
    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //             label: Some("Render Encoder"),
    //         });

    //     // Begin a single render pass
    //     {
    //         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: Some("Main Render Pass"),
    //             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //                 view: &view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    //                     store: wgpu::StoreOp::Store,
    //                 },
    //             })],
    //             depth_stencil_attachment: None, // Add if using depth
    //             timestamp_writes: None,
    //             occlusion_query_set: None,
    //         });

    //         log_debug!("Begin render pass");

    //         // Perform the query within the render pass
    //         world.query4::<Mesh, Uniforms, Material, RenderResources>(
    //             |entity, mesh, uniforms, material, render_resources| {
    //                 log_debug!("render_pass closure started for entity: {}", entity);
    //                 log_debug!("render_pass uniforms: {:?}", uniforms);
    //                 log_debug!("render_pass material: {:?}", material);
    //                 log_debug!("render_pass render_resources: {:?}", render_resources);

    //                 // Update entity's uniform buffer
    //                 self.queue.write_buffer(
    //                     &render_resources.uniform_buffer,
    //                     0,
    //                     bytemuck::bytes_of(uniforms),
    //                 );
    //                 log_debug!("Updated entity {}'s uniform buffer", entity);

    //                 // Set the pipeline
    //                 if let Some(pipeline) = &material.pipeline {
    //                     render_pass.set_pipeline(pipeline);
    //                     log_debug!("Set pipeline for entity {}", entity);
    //                 } else {
    //                     log_debug!("Material pipeline is None for entity {}", entity);
    //                     return;
    //                 }

    //                 // Set bind groups
    //                 render_pass.set_bind_group(0, &self.global_uniform_bind_group, &[]);
    //                 log_debug!("Set global uniform bind group for entity {}", entity);

    //                 render_pass.set_bind_group(1, &render_resources.uniform_bind_group, &[]);
    //                 log_debug!("Set material uniform bind group for entity {}", entity);

    //                 if let Some(texture_bind_group) = &render_resources.texture_bind_group {
    //                     render_pass.set_bind_group(2, texture_bind_group, &[]);
    //                     log_debug!("Set texture bind group for entity {}", entity);
    //                 } else {
    //                     log_debug!("Entity {} does not have a texture bind group", entity);
    //                 }

    //                 // Set vertex and index buffers
    //                 render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    //                 log_debug!("Set vertex buffer for entity {}", entity);

    //                 render_pass
    //                     .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //                 log_debug!("Set index buffer for entity {}", entity);

    //                 // Issue draw call
    //                 render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
    //                 log_debug!("Issued draw call for entity {}", entity);
    //             },
    //         );
    //     }

    //     log_debug!("Submitting command encoder");

    //     // Submit the commands
    //     self.queue.submit(Some(encoder.finish()));

    //     log_debug!("Render pass submitted successfully");

    //     Ok(())
    // }

    // /// Create the window and surface once initialization is complete
    // pub async fn create_render_surface(
    //     &mut self,
    //     el: &ActiveEventLoop,
    // ) -> std::result::Result<(), AppError> {
    //     let window = Arc::new(
    //         el.create_window(
    //             winit::window::Window::default_attributes()
    //                 .with_title("Rupy")
    //                 .with_inner_size(PhysicalSize::new(1200, 800)),
    //         )
    //         .expect("Failed to create window"),
    //     );

    //     let surface = self
    //         .instance
    //         .create_surface(window.clone())
    //         .expect("No surface received from instance!");

    //     let size = window.inner_size();
    //     let surface_config = surface
    //         .get_default_config(&self.adapter, size.width, size.height)
    //         .expect("Surface Configuration");

    //     self.surface = Some(SurfaceWrapper::new(surface, surface_config));
    //     self.window.current = Some(window);
    //     Ok(())
    // }
}

impl RenderSystem {
    pub fn setup_entity_render_resources(
        &mut self,
        device: &wgpu::Device,
        world: &mut World,
        entity: Entity,
        material: &Material,
        uniforms: &Uniforms,
        texture_view: Option<&wgpu::TextureView>,
        sampler: Option<&wgpu::Sampler>,
    ) -> Result<(), AppError> {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout = &material.bind_group_layouts[1];
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let texture_bind_group = if material.bind_group_layouts.len() == 3 {
            let texture_view = texture_view.ok_or(AppError::MissingTexture)?;
            let sampler = sampler.ok_or(AppError::MissingSampler)?;

            Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &material.bind_group_layouts[2],
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
                label: Some("Texture Bind Group"),
            }))
        } else {
            None
        };

        let render_resources = RenderResources {
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group,
        };

        world.add_component(entity, render_resources);

        Ok(())
    }
}
