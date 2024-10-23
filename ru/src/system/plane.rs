// use std::sync::Arc;

// use crate::graphics::{binding::BindGroupType, pipeline::PipelineCache};
// use crate::log_debug;
// use crate::model::primitive::Primitive;
// use crate::traits::buffers::{IndexBuffer, VertexBuffer};
// use wgpu::util::DeviceExt;

// #[derive(Debug)]
// pub struct Plane {
//     pub is_visible: bool,
//     pub vertex_buffer: Option<wgpu::Buffer>,
//     pub index_buffer: Option<wgpu::Buffer>,
//     pub num_indices: u32,
//     pub pipeline: Option<Arc<wgpu::RenderPipeline>>,
// }
// // Define the vertex data for the plane

// impl Plane {
//     pub fn new() -> Self {
//         Self {
//             is_visible: true,
//             vertex_buffer: None,
//             index_buffer: None,
//             num_indices: 0,
//             pipeline: None,
//         }
//     }

//     pub fn initialize(
//         &mut self,
//         device: &wgpu::Device,
//         pipeline_cache: &mut PipelineCache,
//         global_uniform_bind_group_layout: &wgpu::BindGroupLayout,
//         surface_format: &wgpu::TextureFormat,
//         depth_stencil_state: Option<wgpu::DepthStencilState>,
//     ) {
//         log_debug!("Initializing plane.");

//         // Define the plane vertices and indices

//         // Assuming `Vec3`, `Vec4`, and `Vec2` are properly defined and imported
//         let plane_vertices = [
//             Primitive {
//                 position: [-10.0, 0.0, -10.0],
//                 color: [0.5, 0.5, 0.5, 1.0],
//                 uv: [0.0, 0.0],
//             },
//             Primitive {
//                 position: [10.0, 0.0, -10.0],
//                 color: [0.5, 0.5, 0.5, 1.0],
//                 uv: [1.0, 0.0],
//             },
//             Primitive {
//                 position: [10.0, 0.0, 10.0],
//                 color: [0.5, 0.5, 0.5, 1.0],
//                 uv: [1.0, 1.0], // Corrected UV coordinate
//             },
//             Primitive {
//                 position: [-10.0, 0.0, 10.0],
//                 color: [0.5, 0.5, 0.5, 1.0],
//                 uv: [0.0, 1.0],
//             },
//         ];

//         let plane_indices: [u16; 6] = [
//             0, 1, 2, // First triangle
//             2, 3, 0, // Second triangle
//         ];

//         // Create the vertex buffer
//         let vertex_buffer = Primitive::create_static_vertex_buffer(device, &plane_vertices);

//         // Create the index buffer
//         let index_buffer = Primitive::create_static_index_buffer(device, &plane_indices);

//         self.vertex_buffer = Some(vertex_buffer);
//         self.index_buffer = Some(index_buffer);
//         self.num_indices = plane_indices.len() as u32;

//         // Load or create the shader module using the pipeline cache
//         let shader_path = "static/shaders/plane.wgsl";
//         let shader_module = pipeline_cache
//             .load_shader(device, &BindGroupType::Untextured, shader_path)
//             .expect("Failed to load plane shader");

//         let pipeline = pipeline_cache
//             .get_or_create_pipeline(
//                 device,
//                 &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                     label: Some("Plane Render Pipeline"),
//                     bind_group_layouts: &[&global_uniform_bind_group_layout],
//                     push_constant_ranges: &[],
//                 }),
//                 &shader_module,
//                 shader_path,
//                 wgpu::PrimitiveTopology::TriangleList,
//                 wgpu::FrontFace::Ccw,
//                 Some(wgpu::Face::Back),
//                 wgpu::PolygonMode::Fill,
//                 surface_format,
//                 &BindGroupType::Textured,
//                 depth_stencil_state.as_ref(),
//             )
//             .unwrap();
//         self.pipeline = Some(pipeline);

//         log_debug!("Plane initialized.");
//     }

//     pub fn render<'a>(
//         &'a self,
//         render_pass: &mut wgpu::RenderPass<'a>,
//         global_bind_group: &'a wgpu::BindGroup,
//     ) {
//         if !self.is_visible {
//             return;
//         }

//         if let (Some(pipeline), Some(vertex_buffer), Some(index_buffer)) =
//             (&self.pipeline, &self.vertex_buffer, &self.index_buffer)
//         {
//             render_pass.set_pipeline(pipeline);
//             render_pass.set_bind_group(0, global_bind_group, &[]);
//             render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
//             render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
//             render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
//         }
//     }

//     pub fn toggle_visibility(&mut self) {
//         self.is_visible = !self.is_visible;
//     }
// }
