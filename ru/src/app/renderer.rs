use super::DebugMode;
use crate::{
    camera::Camera,
    core::{
        error::AppError,
        surface::{surface_configuration, RenderSurface},
    },
    ecs::{
        buffer::BufferManager,
        component::{material::Material, mesh::Mesh, transform::Transform},
        world::World,
    },
    graphics::{
        bind_group::{instance_data_bind_group, uniform_bind_group, BindGroupLayouts},
        context::GpuContext,
        data::{InstanceData, Uniforms},
        default_depth_stencil_state,
        render::state::RenderState,
        RenderMode,
    },
    input::InputListener,
    log_debug, log_error,
    pipeline::{get_pipeline_label, manager::PipelineManager, render_pipeline},
    prelude::{frame::FrameTime, helpers::string_to_u64},
    shader::manager::ShaderManager,
    texture::{create_depth_texture_with_view, manager::TextureManager},
};

use std::sync::Arc;

use nalgebra::Matrix4;
use wgpu::{
    BindGroup, CompositeAlphaMode, DepthStencilState, PresentMode, TextureUsages, TextureView,
};
use winit::dpi::PhysicalSize;

pub struct Renderer {
    pub gpu: GpuContext,
    pub state: RenderState,
    pub depth_stencil_state: DepthStencilState,
    pub depth_texture_view: TextureView,
    pub pipelines: PipelineManager,
    pub shaders: ShaderManager,
    pub buffers: BufferManager,
    pub textures: TextureManager,
    pub bind_group_layouts: BindGroupLayouts,
    pub uniform_bind_group: BindGroup,
    pub instance_bind_group: Option<wgpu::BindGroup>,
    pub world: World,
    pub target: RenderSurface,
}

impl Renderer {
    pub fn new(gpu: GpuContext, window: Arc<winit::window::Window>) -> Result<Self, AppError> {
        let adapter = gpu.adapter();
        let device = gpu.device();
        let depth_stencil_state = default_depth_stencil_state(None);
        let uniform_buffer_size = std::mem::size_of::<Uniforms>() as u64;
        let bind_group_layouts = BindGroupLayouts::new(device, false, uniform_buffer_size);
        let shaders = ShaderManager::new();
        let pipelines = PipelineManager::new();
        let mut buffers = BufferManager::new(device);
        let textures = TextureManager::new();

        let uniform_bind_group = uniform_bind_group(
            device,
            &buffers.get_uniform_buffer(),
            &bind_group_layouts.uniform_layout,
        );

        let uniform_buffer = buffers.create_uniform_buffer(device, uniform_buffer_size);
        buffers.set_uniform_buffer(uniform_buffer);
        let world = World::new();

        let state = RenderState::new(
            Camera::default(),
            FrameTime::new(),
            DebugMode::None,
            RenderMode::TriangleTextureWithDepth,
        );
        let inner_size = window.inner_size();
        let surface = gpu.instance().create_surface(window.clone())?;
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities.formats[0];
        let present_mode = PresentMode::Fifo;
        let alpha_mode = CompositeAlphaMode::Opaque;
        let texture_usages = TextureUsages::all();
        let desired_frame_latency = 1;
        let surface_config = surface_configuration(
            format,
            inner_size.width,
            inner_size.height,
            present_mode,
            alpha_mode,
            texture_usages,
            vec![],
            desired_frame_latency,
        );
        let (.., depth_texture_view) =
            create_depth_texture_with_view(gpu.device(), &surface_config);
        let surface = RenderSurface::new(surface, surface_config);
        Ok(Renderer {
            gpu,
            state,
            depth_stencil_state,
            pipelines,
            shaders,
            buffers,
            textures,
            depth_texture_view,
            bind_group_layouts,
            uniform_bind_group,
            instance_bind_group: None,
            world,
            target: surface,
        })
    }
    pub fn state(&mut self) -> &RenderState {
        &self.state
    }
    pub fn state_mut(&mut self) -> &mut RenderState {
        &mut self.state
    }
    pub fn gpu(&mut self) -> &GpuContext {
        &self.gpu
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let device = self.gpu.device();
        self.target.surface_config.width = size.width;
        self.target.surface_config.height = size.height;

        self.target.resize(size.width, size.height, &device);
        log_debug!(
            "Resized to: {:?} {:?}",
            self.target.surface_config.height,
            self.target.surface_config.width
        );
        self.depth_texture_view =
            create_depth_texture_with_view(&device, &self.target.surface_config).1;
    }

    pub fn handle_input<T: 'static>(&mut self, event: &winit::event::Event<T>, delta_time: f32) {
        match event {
            winit::event::Event::DeviceEvent { event, .. } => {
                if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                    self.state.camera_mut().on_mouse_motion(*delta);
                }
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    self.state.camera_mut().on_key_event(event, delta_time);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    self.state.camera_mut().on_scroll(*delta);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    self.state.camera_mut().on_mouse_button(*button, *state);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
impl Renderer {
    pub fn render(&mut self) -> Result<(), AppError> {
        match self.target.surface.get_current_texture() {
            Ok(frame) => {
                let current_view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                log_debug!("Received frame!");
                let use_depth = matches!(
                    self.state.render_mode(),
                    RenderMode::LineColorWithDepth
                        | RenderMode::LineTextureWithDepth
                        | RenderMode::TriangleColorWithDepth
                        | RenderMode::TriangleTextureWithDepth
                );

                let depth_stencil_attachment = if use_depth {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: Default::default(),
                    })
                } else {
                    None
                };
                let (primitive, shading, depth) = self.state.render_mode().to_pipeline_config();
                let pipeline_label = get_pipeline_label(&primitive, &shading, &depth);
                let desc = wgpu::RenderPassDescriptor {
                    label: Some(&pipeline_label),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &current_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.05,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],

                    timestamp_writes: None,
                    occlusion_query_set: None,
                    depth_stencil_attachment,
                };
                log_debug!("Received label {:?}!", pipeline_label);
                let mut encoder =
                    self.gpu
                        .device()
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    let mut pass = encoder.begin_render_pass(&desc);

                    let _ = self.world.render(
                        &self.gpu.queue(),
                        self.state.camera(),
                        &mut pass,
                        pipeline_label,
                        &self.pipelines,
                        &self.buffers,
                        &self.textures,
                        &self.uniform_bind_group,
                    );

                    // if self.state.debug_mode() != DebugMode::None {
                    //     self.text_rendering_system.render(
                    //         &mut pass,
                    //         use_depth,
                    //         &self.device,
                    //         &self.queue,
                    //         &self.render_surface.surface_config,
                    //     );
                    // }
                }

                self.gpu.queue().submit(Some(encoder.finish()));
                frame.present();
            }

            Err(e) => {
                log_error!("Error rendering world: {:?}", e);
            }
        }

        Ok(())
    }

    pub async fn initialize_caches(&mut self) -> Result<(), AppError> {
        let device = self.gpu.device();
        let queue = self.gpu.queue();
        let uniform_layout = &self.bind_group_layouts.uniform_layout;
        let texture_layout = &self.bind_group_layouts.sampled_texture_layout;
        let instance_layout = &self.bind_group_layouts.instance_data_layout;
        self.world
            .query_current_scene::<Transform, Material, Mesh>(|_, _, material, mesh| {
                self.buffers
                    .ensure_buffers(&mesh.id, &mesh.vertices, &mesh.indices, device);
                log_debug!("Processing material: {:?}", material);
                if let Some(texture_name) = &material.texture_name {
                    let texture_cache_key = string_to_u64(&texture_name);

                    if !self
                        .textures
                        .contains_texture(&string_to_u64(&texture_name))
                    {
                        match self.textures.load_texture_file(
                            device,
                            &texture_name,
                            &self.target.surface_config,
                        ) {
                            Ok(texture) => {
                                self.textures.insert_texture(texture_cache_key, texture);
                            }
                            Err(e) => {
                                log_error!(
                                    "Using cache key {:?} using texture name {:?}",
                                    texture_cache_key,
                                    texture_name
                                );
                                log_error!("Failed to cache texture: {:?}", e);
                            }
                        }
                    }

                    if !self.textures.contains_bind_group(&texture_cache_key) {
                        if let Err(e) = self.textures.ensure_bind_group(device, texture_cache_key) {
                            log_error!(
                                "Failed to create and cache bind group for texture: {:?}",
                                e
                            );
                        }
                    }

                    if !self.pipelines.contains(mesh.id) {
                        match render_pipeline(
                            &device,
                            &self.target.surface_config.format,
                            uniform_layout,
                            texture_layout,
                            &descriptor,
                        ) {
                            Ok(pipeline) => {
                                if let Err(e) = self.pipelines.put(mesh.id, pipeline) {
                                    log_error!(
                                        "Failed to cache render pipeline for material: {:?}",
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                log_error!("Failed to create render pipeline: {:?}", e);
                            }
                        }
                    }
                }
            })?;
        self.textures.write_textures(queue).await?;
        Ok(())
    }
}
impl Renderer {
    pub fn prepare_instance_data(&mut self) -> Result<(), AppError> {
        let instance_data = self
            .world
            .generate_instance_data_for_current_scene()
            .map_err(|e| {
                log_error!("Failed to generate instance data: {:?}", e);
                e
            })?;
        self.set_instance_data(&instance_data);

        Ok(())
    }
    pub fn update_uniforms(&mut self) {
        let view_proj_matrix = self.state.camera().view_projection_matrix();

        let uniforms = Uniforms::new(
            view_proj_matrix.into(),
            Matrix4::identity().into(),
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [10.0, 10.0, 10.0, 1.0],
        );

        self.gpu.queue().write_buffer(
            &self.buffers.get_uniform_buffer(),
            0,
            bytemuck::cast_slice(&[uniforms]),
        );
    }
    pub fn update_scene(&mut self) {
        // if let Err(e) = self
        //     .world
        //     .update_scene_components::<Transform>("test_scene", |transform| {
        //         transform.rotate();
        //     })
        // {
        //     log_error!("Failed to update scene: {:?}", e);
        // };
    }
    pub fn set_instance_data(&mut self, instance_data: &[InstanceData]) {
        let buffer = self
            .buffers
            .create_instance_buffer(self.gpu.device(), instance_data);
        let instance_bind_group = instance_data_bind_group(
            self.gpu.device(),
            &buffer,
            &self.bind_group_layouts.instance_data_layout,
        );
        self.buffers.set_instance_buffer(buffer);
        self.instance_bind_group = Some(instance_bind_group);
    }
}
