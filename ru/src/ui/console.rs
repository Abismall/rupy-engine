// use std::sync::Arc;

// use glyphon::TextBounds;
// use wgpu::{BindGroup, BindGroupLayout, DepthStencilState, TextureFormat};

// use crate::{
//     graphics::{
//         pipeline::pipeline_cache::PipelineCache, texture::sampler::create_sampler_from_type,
//     },
//     log_debug, log_error,
//     prelude::TextureCache,
//     scene::components::{material::Material, mesh::Mesh},
//     ui::{
//         create_ui_boundaries,
//         layout::{HorizontalAlign, UIComponent, VerticalAlign},
//     },
// };

// use super::menu::Menu;

// #[derive(Debug)]
// pub struct Console {
//     pub is_visible: bool,
//     pub ui: Option<Menu>, // The console's UI components are stored in a `Menu`
//     pub text: String,     // Holds the text that is permanently displayed
//     pub input_buffer: String, // Holds temporary text inputs (e.g., current input line)
// }

// impl Console {
//     /// Creates a new console with default size, position, and UI components
//     pub fn new() -> Self {
//         Self {
//             is_visible: false,
//             ui: None,
//             text: Default::default(),
//             input_buffer: Default::default(), // Initialize the input buffer
//         }
//     }

//     pub fn initialize_ui(
//         &mut self,
//         device: &wgpu::Device,
//         uniform_buffer: &wgpu::Buffer,
//         size: (f32, f32),                   // Pass the window size as a parameter
//         texture_cache: &mut TextureCache,   // Use the texture cache
//         pipeline_cache: &mut PipelineCache, // Use the pipeline cache
//         global_uniform_bind_group_layout: &BindGroupLayout,
//         surface_format: &TextureFormat,
//         depth_stencil_state: Option<DepthStencilState>,
//     ) {
//         let mut console_ui = Menu::new("Console".to_string(), size);
//         let shader_path = "static/shaders/primitive/console.wgsl";

//         // Fetch texture from cache, or load it if necessary
//         let texture = match texture_cache.get_cache_entry("paper_canvas.png") {
//             Some(texture) => texture,
//             None => {
//                 log_error!("Texture 'paper_canvas.png' not found in cache");
//                 return;
//             }
//         };

//         let texture_view = texture
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());
//         let sampler = create_sampler_from_type(
//             device,
//             crate::graphics::texture::sampler::SamplerType::Linear,
//         );

//         // Create bind group layout directly (no cache)
//         let bind_group_layout = Arc::new(Self::create_bind_group_layout(device));

//         // Create the bind group manually (no cache)
//         let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             layout: &bind_group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: uniform_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::TextureView(&texture_view),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 2,
//                     resource: wgpu::BindingResource::Sampler(&sampler),
//                 },
//             ],
//             label: Some("Console UI BindGroup"),
//         });

//         // Create the mesh and vertices for the UIComponent
//         let (vertices, indices) = create_ui_boundaries((1.0, 0.3));

//         // Create and add the UIComponent with the new structure
//         let component_mesh = Mesh::new(device, &vertices, &indices);

//         console_ui.layout.add_component(UIComponent::new(
//             Arc::new(Material::new( /* Update with your new material initialization */ )),
//             (0, 0), // Grid position
//             HorizontalAlign::Left,
//             VerticalAlign::Top,
//             component_mesh,
//         ));

//         // Create the shader module and pipeline
//         let shader_module = pipeline_cache
//             .load_shader_module(device, shader_path)
//             .unwrap();

//         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("Console Pipeline Layout"),
//             bind_group_layouts: &[global_uniform_bind_group_layout, &bind_group_layout],
//             push_constant_ranges: &[],
//         });

//         let pipeline = pipeline_cache.get_or_create_render_pipeline(
//             device,
//             &pipeline_layout,
//             &shader_module,
//             surface_format,
//             depth_stencil_state.as_ref(),
//         );

//         console_ui.set_bind_group(Arc::new(bind_group));
//         console_ui.set_pipeline(Arc::new(pipeline));

//         // Assign the UI to the console
//         self.ui = Some(console_ui);
//         log_debug!("Console initialized.");
//     }

//     /// Show the console
//     pub fn show(&mut self) {
//         self.is_visible = true;
//     }

//     /// Check if the console is visible
//     pub fn is_visible(&self) -> bool {
//         self.is_visible
//     }

//     /// Hide the console
//     pub fn hide(&mut self) {
//         self.is_visible = false;
//     }

//     /// Toggle visibility
//     pub fn toggle(&mut self) {
//         if self.is_visible {
//             self.hide();
//         } else {
//             self.show();
//         };
//     }

//     /// Adds new text to the console's permanent text and clears the input buffer
//     pub fn submit_input(&mut self) {
//         self.text.push_str(&self.input_buffer); // Append input buffer to the permanent text
//         self.input_buffer.clear(); // Clear the input buffer
//     }

//     /// Appends a new character to the input buffer
//     pub fn add_input_char(&mut self, new_char: char) {
//         self.input_buffer.push(new_char); // Add a character to the input buffer
//     }

//     /// Clears the entire console's text buffer
//     pub fn clear(&mut self) {
//         self.text.clear();
//     }

//     /// Provides the current console text and input buffer with bounds
//     pub fn get_text_bounds(
//         &self,
//         viewport_width: i32,
//         viewport_height: i32,
//     ) -> (String, TextBounds) {
//         let combined_text = format!("{}\n> {}", self.text, self.input_buffer); // Display input buffer in the console
//         let bounds = TextBounds {
//             left: 10,
//             top: 10,
//             right: viewport_width,
//             bottom: viewport_height,
//         };
//         (combined_text, bounds)
//     }

//     pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
//         if let Some(ui) = &mut self.ui {
//             match (&ui.pipeline, &ui.bind_group) {
//                 (Some(pipe), Some(bind)) => {
//                     // Bind the global uniforms first
//                     render_pass.set_bind_group(0, &bind, &[]);

//                     // Now bind the component textures, samplers, and other data
//                     render_pass.set_pipeline(pipe);
//                     let _ = ui.render(render_pass);
//                 }
//                 _ => {
//                     return;
//                 }
//             };
//         }
//     }

//     /// Create the bind group layout for the console (uniform buffer, texture, sampler)
//     pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
//         let entries = &[
//             // Uniform Buffer Binding (e.g., for transform matrix or other data)
//             wgpu::BindGroupLayoutEntry {
//                 binding: 0,                                      // Binding at 0 for the uniform buffer
//                 visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, // Accessible by both vertex and fragment shaders
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Uniform, // Uniform buffer type
//                     has_dynamic_offset: false,            // No dynamic offset needed
//                     min_binding_size: None,               // No minimum binding size
//                 },
//                 count: None,
//             },
//             // Texture Binding (e.g., for background or text glyph textures)
//             wgpu::BindGroupLayoutEntry {
//                 binding: 1,                               // Binding at 1 for the texture
//                 visibility: wgpu::ShaderStages::FRAGMENT, // Accessible by the fragment shader only
//                 ty: wgpu::BindingType::Texture {
//                     multisampled: false,                            // No multisampling
//                     view_dimension: wgpu::TextureViewDimension::D2, // 2D texture
//                     sample_type: wgpu::TextureSampleType::Float { filterable: true }, // Float texture data, filterable
//                 },
//                 count: None,
//             },
//             // Sampler Binding (for the texture)
//             wgpu::BindGroupLayoutEntry {
//                 binding: 2,                               // Binding at 2 for the sampler
//                 visibility: wgpu::ShaderStages::FRAGMENT, // Accessible by the fragment shader only
//                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), // Sampler for filtering the texture
//                 count: None,
//             },
//         ];

//         device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: Some("Console BindGroup Layout"),
//             entries,
//         })
//     }
// }
