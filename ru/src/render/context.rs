// use std::{hash::Hasher, sync::Arc};

// use wgpu::{util::DeviceExt, BindGroup, Buffer, BufferUsages, CommandEncoder, RenderPipeline};

// use crate::{
//     camera::{entity::Camera, perspective::CameraPerspective, CameraUniform},
//     log_error,
//     pipeline::cache::PipelineCache,
// };

// use super::renderer::Renderer;

// pub struct RenderContext {
//     renderer: Arc<Renderer>,
//     pipeline_cache: PipelineCache,
//     pub camera: Camera,                 // Camera to handle view and projection
//     pub camera_buffer: Buffer,          // Buffer for uploading camera matrices to the GPU
//     pub perspective: CameraPerspective, // The perspective properties for the camera
// }

// impl RenderContext {
//     /// Creates a new RenderContext
//     pub async fn new(
//         instance: &wgpu::Instance,
//         surface: wgpu::Surface<'static>,
//         camera: Camera,
//         perspective: CameraPerspective,
//     ) -> Result<Self, AppError> {
//         let renderer = Renderer::new(instance, surface).await?;
//         let pipeline_cache = PipelineCache::new();
//         let device = &renderer.device;
//         let camera_uniform =
//             CameraUniform::new(camera.view_matrix(), camera.projection_matrix(&perspective));
//         let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Camera Uniform Buffer"),
//             contents: bytemuck::cast_slice(&[camera_uniform]),
//             usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
//         });

//         Ok(Self {
//             renderer: Arc::new(renderer),
//             pipeline_cache,
//             camera_buffer,
//             perspective,
//             camera,
//         })
//     }

//     /// Creates a new Render Pipeline using Shader Reflection
//     pub fn create_pipeline_with_reflection(
//         &mut self,
//         shader: Arc<Shader>,
//         shader_reflection: &ShaderReflection,
//         use_texture: bool,
//     ) -> Result<Arc<RenderPipeline>, AppError> {
//         // Obtain bind group layouts from the reflection data
//         let global_bind_group_layout = shader_reflection
//             .create_bind_group_layout(&self.renderer.device, "Global Bind Group Layout");

//         let mesh_bind_group_layout = shader_reflection
//             .create_bind_group_layout(&self.renderer.device, "Mesh Bind Group Layout");

//         let texture_bind_group_layout = if use_texture {
//             Some(
//                 shader_reflection
//                     .create_bind_group_layout(&self.renderer.device, "Texture Bind Group Layout"),
//             )
//         } else {
//             None
//         };

//         // Generate a cache key (can be any suitable hash mechanism)
//         let pipeline_cache_key = self.generate_pipeline_cache_key(use_texture, &shader);

//         self.pipeline_cache.get_or_create_pipeline(
//             &self.renderer.device,
//             pipeline_cache_key,
//             &global_bind_group_layout,
//             &mesh_bind_group_layout,
//             texture_bind_group_layout.as_ref(),
//             &shader.vertex_shader,
//             shader.fragment_shader.as_ref().ok_or_else(|| {
//                 AppError::ShaderSourceFileError("Fragment shader missing".to_string())
//             })?,
//             self.renderer.surface_config.read().unwrap().format,
//         )
//     }
//     pub fn get_output_view(&self) -> Result<wgpu::TextureView, AppError> {
//         // Acquire the current frame from the swap chain/surface
//         let surface = self.renderer.surface.write().unwrap(); // Lock the surface to get access to it.

//         match surface.get_current_texture() {
//             Ok(surface_texture) => {
//                 // Create a texture view that represents the surface where we'll render the frame
//                 let output_view = surface_texture
//                     .texture
//                     .create_view(&wgpu::TextureViewDescriptor::default());
//                 Ok(output_view)
//             }
//             Err(error) => {
//                 log_error!("Failed to acquire current surface texture: {:?}", error);
//                 Err(AppError::SurfaceCreationError)
//             }
//         }
//     }
//     /// Render function that records commands for rendering a frame.
//     pub fn render(
//         &self,
//         encoder: &mut CommandEncoder,
//         pipeline: &RenderPipeline,
//         bind_group: &BindGroup,
//         camera_bind_group: &BindGroup,
//     ) -> Result<(), AppError> {
//         let view = self.get_output_view()?;
//         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//             label: Some("Render Pass"),
//             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                 view: &view, // Get the output texture view
//                 resolve_target: None,
//                 ops: wgpu::Operations {
//                     load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                     store: wgpu::StoreOp::Store,
//                 },
//             })],
//             depth_stencil_attachment: None,
//             timestamp_writes: None,
//             occlusion_query_set: None,
//         });

//         render_pass.set_pipeline(pipeline);
//         render_pass.set_bind_group(0, camera_bind_group, &[]);
//         render_pass.set_bind_group(1, bind_group, &[]);

//         render_pass.draw(0..3, 0..1); // Example: Draw a triangle
//         Ok(())
//     }

//     /// Function to submit a command buffer for execution
//     pub fn submit_command(&self, encoder: CommandEncoder) {
//         self.renderer.submit_command(encoder);
//     }

//     /// Helper function to generate a cache key for the pipeline.
//     fn generate_pipeline_cache_key(&self, use_texture: bool, shader: &Shader) -> u64 {
//         // This is a simple placeholder for a hash function to create a unique key for the pipeline.
//         let key = if use_texture {
//             shader.name.clone() + "_textured"
//         } else {
//             shader.name.clone() + "_untextured"
//         };
//         let hash = std::collections::hash_map::DefaultHasher::new();
//         std::hash::Hash::hash(&key, &mut std::collections::hash_map::DefaultHasher::new());
//         hash.finish()
//     }
// }
