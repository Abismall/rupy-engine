use super::flags::BitFlags;
use super::DebugMode;
use crate::camera::CameraHandler;
use crate::ecs::systems::render::{BufferFactory, BufferManager, RenderInfo, Renderer3D};
use crate::graphics::binding::camera::create_camera_bind_group;
use crate::graphics::binding::environment::create_environment_bind_group;
use crate::graphics::binding::light::create_light_bind_group;
use crate::graphics::context::GpuResourceCache;
use crate::graphics::depth::DepthBuffer;
use crate::graphics::glyphon::GlyphonRender;
use crate::graphics::model::CameraUniform;
use crate::graphics::pipelines::hdr::{self, HdrLoader};
use crate::graphics::textures::Texture;
use crate::graphics::ResourceManager;
use crate::{
    core::{error::AppError, files::FileSystem, surface::RenderSurface},
    ecs::world::World,
    graphics::{binding::BindGroupLayouts, model::LightUniform, PrimitiveTopology},
    prelude::frame::FrameMetrics,
};
use crate::{log_error, log_warning};

use std::sync::Arc;

use cgmath::{Rotation3, Vector3};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

impl<'a> State<'a> {
    pub async fn new(
        gpu: GpuResourceCache,
        bit_flags: BitFlags,
        window: winit::window::Window,
    ) -> Result<Self, AppError> {
        let device = gpu.device();
        let queue = gpu.queue();

        let window = Arc::new(window);
        let inner_size = window.inner_size();

        let target_surface = gpu.instance().create_surface(window.clone())?;
        let surface = RenderSurface::new(target_surface, inner_size, gpu.adapter());

        let hdr = hdr::HdrPipeline::new(gpu.device(), &surface.config);
        let bind_group_layouts = BindGroupLayouts::new(device);

        let resources = ResourceManager::new(device, &bind_group_layouts, hdr.format())?;

        let camera_handler = CameraHandler::new(
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
            inner_size,
        );
        let hdr_loader = HdrLoader::new(gpu.device());
        let sky_texture = hdr_loader.from_equirectangular_bytes(
            gpu.device(),
            gpu.queue(),
            &FileSystem::load_binary("pure-sky.hdr")?,
            1080,
            Some("Sky Texture"),
        )?;
        let bind_groups =
            Self::initialize_bind_groups(gpu.device(), &sky_texture, &resources.buffer_manager)?;

        let depth_stencil = wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        let glyphon = GlyphonRender::new(&device, queue, 3, surface.config.format, &depth_stencil);
        let ctx = RenderInfo::new(
            FrameMetrics::new(),
            DebugMode::None,
            PrimitiveTopology::TriangleList,
        );
        let depth_buffer = DepthBuffer::new(
            device,
            inner_size,
            wgpu::CompareFunction::Less,
            true,
            "depth_buffer",
        );

        let world = World::new();
        let renderer = Renderer3D::new(
            ctx,
            hdr,
            depth_buffer,
            glyphon,
            bind_group_layouts,
            bind_groups,
        );

        Ok(Self {
            gpu,
            bit_flags,
            resources,
            camera_handler,
            light_uniform: LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0]),
            world,
            target: surface,
            window,
            renderer,
        })
    }
    fn initialize_bind_groups(
        device: &wgpu::Device,
        sky_texture: &Texture,
        buffer_manager: &BufferManager,
    ) -> Result<Vec<wgpu::BindGroup>, AppError> {
        let mut bind_groups: Vec<wgpu::BindGroup> = Vec::with_capacity(3);

        let light_bind_group = if let Some(buffer) = buffer_manager.get_light_buffer() {
            create_light_bind_group(device, buffer)
        } else {
            let buffer = BufferFactory::create_light_buffer(device);
            create_light_bind_group(device, &buffer)
        };
        bind_groups.push(light_bind_group);

        let camera_buffer = if let Some(buffer) = buffer_manager.get_camera_buffer() {
            buffer
        } else {
            &BufferFactory::create_camera_uniform_buffer(device)
        };
        let camera_bind_group = create_camera_bind_group(device, &camera_buffer);
        bind_groups.push(camera_bind_group);

        let environment_bind_group = create_environment_bind_group(device, sky_texture);
        bind_groups.push(environment_bind_group);

        Ok(bind_groups)
    }
}
pub struct State<'a> {
    pub gpu: GpuResourceCache,
    pub bit_flags: BitFlags,
    pub camera_handler: CameraHandler,
    pub resources: ResourceManager,
    pub renderer: Renderer3D,
    pub light_uniform: LightUniform,

    pub world: World,
    pub target: RenderSurface<'a>,
    pub window: Arc<winit::window::Window>,
}

impl<'a> State<'a> {
    pub fn gpu(&mut self) -> &GpuResourceCache {
        &self.gpu
    }
    pub fn update_camera(&mut self) {
        self.camera_handler.update_view_position();
        self.camera_handler.update_view_projection();
    }
    pub fn update_camera_uniform_buffer(&mut self) {
        if !self.resources.buffer_manager.contains_camera_buffer() {
            let buffer = BufferFactory::create_camera_uniform_buffer(self.gpu.device());
            self.resources.buffer_manager.set_camera_buffer(buffer);
        }
        if let Some(buffer) = self.resources.buffer_manager.get_camera_buffer() {
            self.gpu.write_to_buffer::<CameraUniform>(
                buffer,
                0,
                bytemuck::cast_slice(&[self.camera_handler.uniform]),
            );
        }
    }
    pub fn update_debug_buffer(&mut self) {
        self.renderer.glyphon.update_debug_buffer(
            &self.renderer.ctx.debug_mode(),
            &self.renderer.ctx.frame_metrics(),
        );
    }
    pub fn compute(&mut self) {
        self.renderer.ctx.compute();
        self.update_debug_buffer();
    }
    pub fn update(&mut self) {
        self.update_camera();
        self.update_camera_uniform_buffer();

        self.update_light_position();
        self.update_light_uniform_buffer();
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if self.target.resize(size) {
            let device = self.gpu.device();
            self.camera_handler.resize(size);
            self.renderer.resize(&device, &self.target.config, size);
            self.target.surface.configure(&device, &self.target.config);
        };
    }
    pub fn render(&mut self) {
        match self.renderer.render(
            &self.gpu,
            &self.target,
            &mut self.world,
            &mut self.resources,
            &self.camera_handler,
        ) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.target.resize(self.window.inner_size());
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log_error!("OutOfMemory");
            }
            Err(wgpu::SurfaceError::Timeout) => {
                log_warning!("Surface timeout");
            }
        }
    }
}
impl<'a> State<'a> {
    fn light_position(&self) -> [f32; 3] {
        self.light_uniform.position
    }
    fn compute_new_light_position(&self, old_position: Vector3<f32>) -> Vector3<f32> {
        cgmath::Quaternion::from_axis_angle(
            (0.0, 1.0, 0.0).into(),
            cgmath::Deg(60.0 * self.renderer.ctx.frame_metrics().delta_time),
        ) * old_position
    }
    pub fn update_light_position(&mut self) {
        let old_position: cgmath::Vector3<_> = self.light_position().into();
        self.light_uniform.position = self.compute_new_light_position(old_position).into();
    }
    fn update_light_uniform_buffer(&mut self) {
        if !self.resources.buffer_manager.contains_light_buffer() {
            let device = &self.gpu.device();
            let light_buffer = BufferFactory::create_light_buffer(device);
            self.resources.buffer_manager.set_light_buffer(light_buffer);
        }
        if let Some(light_buffer) = self.resources.buffer_manager.get_light_buffer() {
            self.gpu
                .write_to_buffer(&light_buffer, 0, &[self.light_uniform]);
        }
    }

    pub fn input(&mut self, event: &WindowEvent, delta_time: f32) {
        if self.camera_handler.controller.process_events(event) == true {
            self.camera_handler.update_movement(delta_time);
        }
    }
}
