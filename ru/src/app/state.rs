use super::flags::BitFlags;
use super::DebugMode;
use crate::camera::frustum::Frustum;
use crate::camera::handler::{create_camera_handler, CameraHandler};
use crate::core::files::FileSystem;
use crate::ecs::components::instance::manager::InstanceManager;
use crate::ecs::components::instance::model::Instance;
use crate::ecs::components::material::manager::MaterialManager;
use crate::ecs::components::mesh::manager::MeshManager;
use crate::ecs::components::model::manager::ModelManager;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::ResourceContext;
use crate::ecs::systems::render::{BufferFactory, BufferManager, RenderInfo, Renderer3D};
use crate::graphics::binding::{
    initialize_common_bind_groups, setup_bind_group_layouts, BindGroupManager,
};
use crate::graphics::context::GpuResourceCache;
use crate::graphics::glyphon::GlyphonRender;
use crate::graphics::pipelines::common::PipelineBase;
use crate::graphics::pipelines::hdr::{self, HdrLoader};
use crate::graphics::pipelines::manager::PipelineManager;
use crate::graphics::shaders::manager::ShaderManager;
use crate::graphics::textures::depth_texture::DepthTexture;
use crate::graphics::textures::manager::TextureManager;
use crate::graphics::textures::Texture;
use crate::graphics::uniform::camera::CameraUniform;
use crate::graphics::uniform::lighting::LightUniform;
use crate::graphics::uniform::Uniforms;
use crate::graphics::PrimitiveTopology;
use crate::prelude::cache::CacheKey;
use crate::{
    core::{error::AppError, surface::RenderSurface},
    ecs::world::World,
    prelude::metrics::FrameMetrics,
};
use crate::{log_error, log_warning};
use bytemuck::cast_slice;
use cgmath::{Quaternion, Vector3};

use std::sync::Arc;

use cgmath::Rotation3;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

const NUM_INSTANCES_PER_ROW: u32 = 10;
const SPACE_BETWEEN: f32 = 2.0;

impl State {
    pub async fn new(
        gpu: GpuResourceCache,
        bit_flags: BitFlags,
        window: std::sync::Arc<winit::window::Window>,
    ) -> Result<Self, AppError> {
        let device = gpu.device();
        let queue = gpu.queue();

        let inner_size = window.inner_size();

        let target_surface = gpu.instance().create_surface(window.clone())?;
        let surface = RenderSurface::new(target_surface, inner_size, &gpu.adapter(), &device);

        let bind_group_layouts = setup_bind_group_layouts(&device, surface.config.format);

        let preload_paths = vec![
            "effects/tone_mapping.wgsl",
            "compute/equirectangular.wgsl",
            "core/lighting.wgsl",
            "objects/skybox.wgsl",
            "core/normal.wgsl",
        ];
        let mut shader_manager = ShaderManager::new(&device, preload_paths)?;
        let hdr = hdr::HdrPipeline::new(
            &device,
            surface.config.format,
            inner_size.width,
            inner_size.height,
            &mut shader_manager,
            &bind_group_layouts,
        );
        let pipeline_manager = match PipelineManager::setup(
            &device,
            vec![
                PrimitiveTopology::PointList,
                PrimitiveTopology::LineList,
                PrimitiveTopology::LineStrip,
                PrimitiveTopology::TriangleList,
                PrimitiveTopology::TriangleStrip,
            ],
            Some(Texture::DEPTH_FORMAT),
            &bind_group_layouts,
            hdr.format(),
            &mut shader_manager,
        ) {
            Ok(pipelines) => pipelines,
            Err(e) => {
                return Err(e);
            }
        };
        let mut buffer_manager = BufferManager::new();
        let mut bind_group_manager = BindGroupManager::new(bind_group_layouts);
        let mut instance_manager = InstanceManager::new();

        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                    let scale = Vector3::new(1.0, 1.0, 1.0);

                    let transform = Transform {
                        position: Vector3 { x, y: 1.0, z },
                        rotation: Quaternion::from_axis_angle(Vector3::unit_z(), cgmath::Deg(0.0)),
                        scale,
                    };

                    Instance { transform }
                })
            })
            .collect::<Vec<_>>();

        let mesh_manager = MeshManager::new();
        let material_manager = MaterialManager::new();
        let texture_manager = TextureManager::new();

        let model_manager = ModelManager::new();
        let light_bg_cache_key = CacheKey::from("bind:group:light");
        let camera_bg_cache_key = CacheKey::from("bind:group:camera");
        let environment_bg_cache_key = CacheKey::from("bind:group:environment");
        let skybox_cache_key = CacheKey::from("skybox");
        let sky_texture = HdrLoader::new(&device).from_equirectangular_bytes(
            &device,
            &queue,
            &FileSystem::load_binary("pure-sky.hdr")?,
            1080,
            Some("Sky Texture"),
        )?;
        let uniforms = Uniforms {
            camera: CameraUniform::new(),
            lighting: LightUniform::new([1.0, 5.0, 1.0], [1.0, 1.0, 1.0]),
        };
        let camera_position = Vector3::new(0.0, 0.0, -10.0);

        let camera_handler = create_camera_handler(inner_size, camera_position.into());
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.compute(&camera_handler.view, &camera_handler.projection);
        let frustum = Frustum::from_camera_handler(&camera_handler);

        let _ = initialize_common_bind_groups(
            &device,
            &uniforms,
            sky_texture,
            environment_bg_cache_key,
            light_bg_cache_key,
            camera_bg_cache_key,
            &mut buffer_manager,
            &mut bind_group_manager,
        );

        let depth_stencil = wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        let glyphon = GlyphonRender::new(&device, &queue, surface.config.format, &depth_stencil);
        let ctx = RenderInfo::new(
            FrameMetrics::new(),
            DebugMode::None,
            PrimitiveTopology::TriangleList,
        );
        let depth_texture = Texture::create_depth_texture(&device, inner_size, "texture:depth");
        let depth_buffer = DepthTexture::new(depth_texture, depth_stencil);

        let mut world = World::new();

        let entity = world.create_entity();
        for instance in instances {
            let _ = instance_manager.add_instance(&entity, instance);
        }
        let mut resources = ResourceContext {
            bind_group_manager,
            buffer_manager,
            material_manager,
            texture_manager,
            model_manager,
            pipeline_manager,
            mesh_manager,
            shader_manager,
            instance_manager,
        };
        let model =
            ModelManager::load_model_from_file("cube.obj", &device, &queue, &mut resources).await?;

        if let Err(e) = world.add_component(entity, model) {
            log_error!("Error from world: {:?}", e);
        };

        let renderer = Renderer3D::new(
            ctx,
            hdr,
            depth_buffer,
            glyphon,
            camera_bg_cache_key,
            environment_bg_cache_key,
            light_bg_cache_key,
            skybox_cache_key,
        );

        Ok(Self {
            gpu,
            bit_flags,
            resources,
            camera_handler,
            uniforms,
            world,
            target: surface,
            window,
            renderer,
            frustum,
        })
    }
}
pub struct State {
    pub gpu: GpuResourceCache,
    pub bit_flags: BitFlags,
    pub camera_handler: CameraHandler,
    pub resources: ResourceContext,
    pub renderer: Renderer3D,
    pub uniforms: Uniforms,
    pub frustum: Frustum,

    pub world: World,
    pub target: RenderSurface<'static>,
    pub window: Arc<winit::window::Window>,
}

impl State {
    pub fn gpu(&mut self) -> &GpuResourceCache {
        &self.gpu
    }
    pub fn update(&mut self) {
        self.transform_entity_instances();
        self.update_lighting();
        self.update_camera();
        self.update_metrics();
    }
    pub fn update_camera(&mut self) {
        let device = &self.gpu().device();
        let queue = &self.gpu().queue();
        let view_projection = self.camera_handler.view_projection_matrix();
        self.frustum.update_planes(view_projection);
        self.camera_handler
            .update_buffer(device, queue, &mut self.uniforms, &mut self.resources);
    }

    pub fn update_metrics(&mut self) {
        self.renderer.glyphon.update_debug_buffer(
            &self.renderer.ctx.debug_mode(),
            &self.renderer.ctx.frame_metrics(),
        );
    }

    pub fn compute_metrics(&mut self) {
        self.renderer.ctx.compute_metrics();
    }
    pub fn update_lighting(&mut self) {
        let device = &self.gpu().device();
        let queue = &self.gpu().queue();
        let resources = &mut self.resources;
        let cache_id = CacheKey::from("bind:group:light");
        let old_position: cgmath::Vector3<_> = self.uniforms.lighting.position.into();
        self.uniforms.lighting.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
                * old_position)
                .into();

        if let Ok(buffer) = resources
            .buffer_manager
            .get_or_create_buffer(cache_id, || Ok(BufferFactory::create_light_buffer(device)))
        {
            queue.write_buffer(&buffer, 0, cast_slice(&[self.uniforms.lighting]));
        }
    }
    pub fn transform_entity_instances(&mut self) {
        for entity in self.world.get_entities() {
            self.resources
                .instance_manager
                .update_instance_transform(entity);
        }
    }
    pub fn resize<P: winit::dpi::Pixel>(&mut self, size: PhysicalSize<P>) {
        if self.target.update_config_size(size) {
            let device = self.gpu.device();
            self.target.surface.configure(&device, &self.target.config);
            self.renderer
                .resize_textures(&device, size, &self.resources);
            self.camera_handler.set_aspect_ratio_from_size(size);
        };
    }
    pub fn render(&mut self) {
        if self.bit_flags.is_running() {
            let device = self.gpu.device();
            let queue = self.gpu.queue();
            match self.renderer.render(
                &device,
                &queue,
                &self.target,
                &mut self.world,
                &mut self.resources,
                &self.camera_handler,
                &self.frustum,
                &self.uniforms,
            ) {
                Ok(_) => {
                    self.compute_metrics();
                }
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    self.resize(self.window.inner_size());
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log_warning!("Out of memory");
                }
                Err(wgpu::SurfaceError::Timeout) => {
                    log_warning!("Surface timeout");
                }
            }
        }
    }
}
impl State {
    pub fn input(&mut self, event: &WindowEvent, delta_time: f32) {
        let camera_handler = &mut self.camera_handler;
        camera_handler
            .controller
            .process_movement(event, &mut camera_handler.view, delta_time);
    }
}
