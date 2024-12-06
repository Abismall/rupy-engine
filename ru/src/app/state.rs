use super::DebugMode;
use crate::ecs::resources::ResourceManager;
use crate::ecs::systems::render::{BufferFactory, RenderInfo, Renderer3D};
use crate::graphics::binding::camera::create_camera_bind_group;
use crate::graphics::binding::environment::create_environment_bind_group;
use crate::graphics::binding::light::create_light_bind_group;
use crate::graphics::binding::{CommonBindGroupIndex, SharedBindGroups};
use crate::graphics::context::GpuContext;
use crate::graphics::glyphon::GlyphonRender;
use crate::graphics::pipelines::hdr::{self, HdrLoader};
use crate::graphics::textures::Texture;
use crate::{
    camera::{controller::CameraController, projection::Projection, Camera},
    core::{error::AppError, files::FileSystem, surface::RenderSurface},
    ecs::world::World,
    graphics::{
        binding::BindGroupLayouts,
        model::{CameraUniform, LightUniform},
        PrimitiveTopology,
    },
    prelude::frame::FrameTime,
};

use std::sync::Arc;

use cgmath::Rotation3;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

pub struct State<'a> {
    pub gpu: GpuContext,
    pub renderer: Renderer3D,
    pub resources: ResourceManager,

    pub camera: Camera,
    pub projection: Projection,
    pub camera_controller: CameraController,
    pub camera_uniform: CameraUniform,

    pub light_uniform: LightUniform,

    pub world: World,
    pub target: RenderSurface<'a>,
    pub window: Arc<winit::window::Window>,
}

impl<'a> State<'a> {
    pub async fn new(gpu: GpuContext, window: winit::window::Window) -> Result<Self, AppError> {
        let window = Arc::new(window);
        let adapter = gpu.adapter();
        let device = gpu.device();
        let queue = gpu.queue();
        let bind_group_layouts = BindGroupLayouts::new(device);
        let inner_size = window.inner_size();
        let target_surface = gpu.instance().create_surface(window.clone())?;
        let surface = RenderSurface::new(target_surface, inner_size, &adapter);
        let hdr = hdr::HdrPipeline::new(&device, &surface.config);
        let mut resources = ResourceManager::new(&device, &bind_group_layouts, hdr.format())?;
        let hdr_loader = HdrLoader::new(&device);
        let sky_bytes = FileSystem::load_binary("pure-sky.hdr")?;

        let sky_texture = hdr_loader.from_equirectangular_bytes(
            &device,
            &queue,
            &sky_bytes,
            1080,
            Some("Sky Texture"),
        )?;

        let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = Projection::new(
            inner_size.width,
            inner_size.height,
            cgmath::Deg(45.0),
            0.1,
            100.0,
        );

        let camera_controller = CameraController::new(4.0, 0.8);

        let ctx = RenderInfo::new(
            FrameTime::new(),
            DebugMode::None,
            PrimitiveTopology::TriangleList,
        );
        let camera_buffer = BufferFactory::camera_buffer(device, &camera, &projection);
        let light_buffer = BufferFactory::light_buffer(device);
        resources.buffer_manager.set_camera_buffer(camera_buffer);
        resources.buffer_manager.set_light_buffer(light_buffer);

        let depth_texture =
            Texture::create_depth_texture(&device, &surface.config, "depth_texture");

        let camera_uniform = CameraUniform::new();
        let depth_stencil = wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        let glyphon = GlyphonRender::new(device, queue, 3, surface.config.format, &depth_stencil);

        let light_uniform = LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };
        let mut shared_bind_groups = SharedBindGroups::new();
        if let Some(buffer) = resources.buffer_manager.get_light_buffer() {
            shared_bind_groups.insert(
                CommonBindGroupIndex::Light,
                create_light_bind_group(device, buffer),
            );
        }

        if let Some(buffer) = resources.buffer_manager.get_camera_buffer() {
            shared_bind_groups.insert(
                CommonBindGroupIndex::Camera,
                create_camera_bind_group(device, buffer),
            );
        }

        shared_bind_groups.insert(
            CommonBindGroupIndex::Environment,
            create_environment_bind_group(device, &sky_texture),
        );
        let mut world = World::new();
        let _ = world
            .initialize_test_scene_components(
                &device,
                &queue,
                &bind_group_layouts.texture_bind_group_layout,
                &mut resources,
            )
            .await?;
        let renderer = Renderer3D::new(
            ctx,
            hdr,
            depth_texture,
            depth_stencil,
            glyphon,
            bind_group_layouts,
            shared_bind_groups,
        );
        Ok(State {
            renderer,
            gpu,
            light_uniform,
            camera,
            projection,
            camera_controller,
            camera_uniform,
            world,
            window: window.into(),
            target: surface,
            resources,
        })
    }
    pub fn context(&mut self) -> &RenderInfo {
        &self.renderer.ctx
    }
    pub fn context_mut(&mut self) -> &mut RenderInfo {
        &mut self.renderer.ctx
    }
    pub fn gpu(&mut self) -> &GpuContext {
        &self.gpu
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let device = self.gpu.device();
        self.projection.resize(size.width, size.height);
        self.target.resize(&device, size);
        self.renderer.depth_texture =
            Texture::create_depth_texture(&device, &self.target.config, "depth_texture");
        self.renderer.hdr.resize(
            &self.gpu.device(),
            size.width,
            size.height,
            self.target.config.format,
        );
    }

    pub fn update(&mut self) {
        self.camera_controller
            .update_camera(&mut self.camera, self.renderer.ctx.frame_time().delta_time);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        if let Some(camera_buffer) = self.resources.buffer_manager.get_camera_buffer() {
            self.gpu.queue().write_buffer(
                &camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        }

        let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        self.light_uniform.position = (cgmath::Quaternion::from_axis_angle(
            (0.0, 1.0, 0.0).into(),
            cgmath::Deg(60.0 * self.renderer.ctx.frame_time().delta_time),
        ) * old_position)
            .into();
        if let Some(light_buffer) = self.resources.buffer_manager.get_light_buffer() {
            self.gpu.queue().write_buffer(
                &light_buffer,
                0,
                bytemuck::cast_slice(&[self.light_uniform]),
            );
        }

        self.renderer.ctx.update();
        self.renderer.glyphon.debug(
            self.renderer.ctx.debug_mode(),
            &self.renderer.ctx.frame_time(),
        );
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }
}
