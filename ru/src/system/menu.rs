// use std::sync::Arc;

// use wgpu::{BindGroup, BindGroupLayout, DepthStencilState, TextureFormat};

// use crate::{
//     graphics::{
//         binding::{
//             sampler_entry, texture_view_entry, uniform_buffer_entry, BindGroupCache, BindGroupType,
//         },
//         pipeline::{create_pipeline, PipelineCache},
//         texture::create_sampler,
//     },
//     log_debug, log_error,
//     prelude::TextureCache,
//     system::create_ui_boundaries,
// };

// use super::ui::{
//     component::UIComponent,
//     layout::{HorizontalAlign, VerticalAlign},
//     Menu,
// };

// #[derive(Debug)]
// pub struct MainMenu {
//     pub is_visible: bool,
//     pub ui: Option<Menu>, // The console's UI components are stored in a `Menu`
// }

// impl MainMenu {
//     /// Creates a new console with default size, position, and UI components
//     pub fn new() -> Self {
//         Self {
//             is_visible: false,
//             ui: None,
//         }
//     }
//     pub fn initialize_ui(
//         &mut self,
//         device: &wgpu::Device,
//         uniform_buffer: &wgpu::Buffer,
//         size: (f32, f32),
//         texture_cache: &mut TextureCache,
//         pipeline_cache: &mut PipelineCache,
//         bind_group_cache: &mut BindGroupCache,
//         global_uniform_bind_group_layout: &BindGroupLayout,
//         surface_format: &TextureFormat,
//         depth_stencil_state: Option<DepthStencilState>,
//     ) {
//         log_debug!("Initializing menu.");
//         let mut console_ui = Menu::new("Main".to_string(), size);
//         let shader_path = "static/shaders/primitive/console.wgsl";
//         // Fetch texture from cache, or load it if necessary
//         let texture = match texture_cache.get_cache_entry("retro_triangle.png") {
//             Some(texture) => texture,
//             None => {
//                 log_error!("Texture not found in cache");
//                 return;
//             }
//         };

//         let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
//         let sampler = create_sampler(device, "ClampToEdge");

//         // Create bind group layout via the cache
//         let bind_group_layout = Arc::new(Self::create_bind_group_layout(device));

//         // Use bind group cache to fetch or create bind group
//         let bind_group = bind_group_cache
//             .get_or_create_bind_group(
//                 device,
//                 shader_path, // Identifier for the cache
//                 &[
//                     uniform_buffer_entry(0, uniform_buffer),
//                     texture_view_entry(1, &texture_view),
//                     sampler_entry(2, &sampler),
//                 ],
//                 &BindGroupType::Textured,
//                 bind_group_layout.clone(),
//                 Some("MainMenu Textured Bind group"),
//             )
//             .unwrap();

//         let (vertices, indices) = create_ui_boundaries((1.0, 1.0));

//         console_ui.add_component(UIComponent::new(
//             device,
//             indices,
//             vertices,
//             [1.0, 1.0, 1.0, 0.1].into(),
//             (0, 0),
//             HorizontalAlign::Left,
//             VerticalAlign::Top,
//         ));
//         // Use the pipeline cache to create or retrieve the pipeline

//         let shader_module = pipeline_cache
//             .load_shader(device, &BindGroupType::Textured, shader_path)
//             .unwrap();
//         let pipeline = pipeline_cache
//             .get_or_create_pipeline(
//                 device,
//                 &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                     label: Some("Console Pipeline Layout"),
//                     bind_group_layouts: &[&global_uniform_bind_group_layout, &bind_group_layout],
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

//         // Set up the pipeline for the UI component
//         console_ui.set_pipeline(pipeline);
//         // Set up the UI layout with the bind group
//         console_ui.set_bind_group(bind_group.clone());
//         // Set up the pipeline for the UI component
//         // Assign the UI to the console
//         self.ui = Some(console_ui);
//         log_debug!("Console initialized.");
//     }
//     pub fn show(&mut self) {
//         self.is_visible = true;
//     }

//     pub fn is_visible(&self) -> bool {
//         self.is_visible
//     }

//     pub fn hide(&mut self) {
//         self.is_visible = false;
//     }

//     pub fn toggle(&mut self) {
//         if self.is_visible {
//             self.hide();
//         } else {
//             self.show();
//         };
//     }
//     pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
//         if let Some(ui) = &mut self.ui {
//             let _ = ui.render(render_pass);
//         };
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
//                     view_dimension: wgpu::TextureViewDimension::D3, // 2D texture
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
