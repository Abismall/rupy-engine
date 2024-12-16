use crate::{
    app::DebugMode,
    camera::{
        frustum::{BoundingVolume, Frustum},
        handler::CameraHandler,
    },
    core::{cache::HashCache, error::AppError, surface::RenderSurface},
    ecs::{
        components::{
            instance::model::InstanceRaw,
            mesh::{manager::MeshManager, model::Mesh},
            model::model::Model,
            ResourceContext,
        },
        traits::{BufferCreator, Cache, RenderPassDraw},
        world::World,
    },
    graphics::{
        binding::camera::create_camera_bind_group,
        glyphon::GlyphonRender,
        pipelines::{common::PipelineBase, hdr},
        textures::depth_texture::DepthTexture,
        uniform::{camera::CameraUniform, lighting::LightUniform, Uniforms},
        vertex::VertexType,
        PrimitiveTopology,
    },
    log_error, log_info, log_warning,
    prelude::{cache::CacheKey, metrics::FrameMetrics},
};
use bytemuck::{Pod, Zeroable};
use cgmath::{SquareMatrix, Vector3, Vector4};
use std::ops::Range;
use wgpu::{util::DeviceExt, Buffer, BufferUsages};
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
struct DebugLineVertex {
    position: [f32; 3],
    color: [f32; 3],
}
pub struct Renderer3D {
    pub ctx: RenderInfo,
    pub hdr: hdr::HdrPipeline,
    pub depth_texture: DepthTexture,
    pub glyphon: GlyphonRender,
    pub camera_bg_cache_key: CacheKey,
    environment_bg_cache_key: CacheKey,
    light_bg_cache_key: CacheKey,
    skybox_cache_key: CacheKey,
}

impl Renderer3D {
    pub fn new(
        ctx: RenderInfo,
        hdr: hdr::HdrPipeline,
        depth_texture: DepthTexture,
        glyphon: GlyphonRender,
        camera_bg_cache_key: CacheKey,
        environment_bg_cache_key: CacheKey,
        light_bg_cache_key: CacheKey,
        skybox_cache_key: CacheKey,
    ) -> Self {
        Self {
            ctx,
            hdr,
            depth_texture,
            glyphon,
            camera_bg_cache_key,
            environment_bg_cache_key,
            light_bg_cache_key,
            skybox_cache_key,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &RenderSurface,
        world: &mut World,
        resources: &mut ResourceContext,
        camera_handler: &CameraHandler,
        frustum: &Frustum,
        uniforms: &Uniforms,
    ) -> Result<(), wgpu::SurfaceError> {
        let debug_mode = self.ctx.debug_mode();
        let ctx = &mut self.ctx;

        let output = target.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let bind_group_manager = &mut resources.bind_group_manager;

        let environment_bind_group = bind_group_manager
            .bind_groups
            .get(&self.environment_bg_cache_key)
            .expect("Environment bind group not found");
        let light_bind_group = bind_group_manager
            .bind_groups
            .get(&self.light_bg_cache_key)
            .expect("Light bind group not found");
        let texture_bind_group = bind_group_manager
            .bind_groups
            .get(&CacheKey::from("Material.001"))
            .expect("Texture bind group not found");

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.hdr.view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],

                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.current.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let uniform_buffer =
                BufferFactory::create_camera_uniform_buffer(&device, uniforms.camera);

            let camera_bind_group = create_camera_bind_group(&device, &uniform_buffer);
            let pipeline_label = ctx.render_mode().label();

            let light_pipeline = resources
                .pipeline_manager
                .pipelines
                .get(&CacheKey::from(
                    format!("{}_light_pipeline", pipeline_label).as_str(),
                ))
                .expect("Light pipeline not found");

            let normal_pipeline = resources
                .pipeline_manager
                .pipelines
                .get(&CacheKey::from(
                    format!("{}_pipeline", pipeline_label).as_str(),
                ))
                .expect("Light pipeline not found");

            let _ = world.query::<Model>(|entity, model| {
                let cache_id = CacheKey::from(entity);
                render_pass.set_pipeline(&light_pipeline);
                render_pass.draw_model(
                    &model,
                    &[&camera_bind_group, &light_bind_group],
                    &Some(0..1),
                    &resources.buffer_manager,
                    &resources.mesh_manager,
                );

                if let Some(instances) = resources
                    .instance_manager
                    .instances
                    .get(&CacheKey::from(entity))
                {
                    let total_instances = instances.len() as u32;
                    let mut culled_instances = 0u32;

                    let instance_raw_data: Vec<_> = instances
                        .iter()
                        .filter_map(|instance| {
                            let center = instance.transform.position - camera_handler.position();
                            let radius =
                                Frustum::calculate_instance_radius(instance.transform.scale);
                            if frustum.contains(&BoundingVolume::Sphere { center, radius }) {
                                Some(instance.to_raw([1.0, 1.0, 1.0, 1.0]))
                            } else {
                                culled_instances += 1;
                                if debug_mode == DebugMode::Verbose
                                    || debug_mode == DebugMode::Minimal
                                {
                                    Some(instance.to_raw([1.0, 1.0, 1.0, 0.1]))
                                } else {
                                    None
                                }
                            }
                        })
                        .collect();

                    ctx.frame_metrics
                        .update_instance_stats(total_instances, culled_instances);

                    resources.buffer_manager.create_instance_buffer(
                        &device,
                        &instance_raw_data,
                        cache_id,
                    );

                    render_pass.set_vertex_buffer(
                        1,
                        resources
                            .buffer_manager
                            .get_instance_buffer(cache_id)
                            .unwrap()
                            .slice(..),
                    );

                    render_pass.set_pipeline(&normal_pipeline);

                    render_pass.draw_model(
                        &model,
                        &[
                            texture_bind_group,
                            &camera_bind_group,
                            light_bind_group,
                            environment_bind_group,
                        ],
                        &Some(0..instance_raw_data.len() as u32),
                        &resources.buffer_manager,
                        &resources.mesh_manager,
                    );
                }
            });

            render_pass.set_pipeline(
                &resources
                    .pipeline_manager
                    .pipelines
                    .get(&self.skybox_cache_key)
                    .expect("Sky pipeline not found"),
            );
            render_pass.draw_vertices(
                &[&camera_bind_group, environment_bind_group],
                0..3,
                Some(0..1),
            );

            self.glyphon.render(
                &mut render_pass,
                &device,
                &queue,
                &target.config,
                self.depth_texture.stencil_state.depth_write_enabled,
            );

            if self.ctx.debug_mode() == DebugMode::Verbose {
                Self::debug_render_pass(
                    &device,
                    &Some(self.depth_texture.current.texture.format()),
                    &mut render_pass,
                    camera_handler,
                );
            }
        }

        self.hdr.process(&mut encoder, &view);

        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.glyphon.clear_buffer();

        Ok(())
    }
    pub fn debug_render_pass(
        device: &wgpu::Device,
        depth_format: &Option<wgpu::TextureFormat>,
        render_pass: &mut wgpu::RenderPass,
        camera: &CameraHandler,
    ) {
        let near_plane = camera.projection.znear;
        let frustum_corners = compute_frustum_corners(camera, near_plane);
        let debug_line_vertices = generate_frustum_edges(&frustum_corners);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Frustum Debug Vertex Buffer"),
            contents: bytemuck::cast_slice(&debug_line_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let shader_module = device.create_shader_module(wgpu::include_wgsl!(
            "../../assets/shaders/core/minimal.wgsl"
        ));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Debug Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let debug_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Debug Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<DebugLineVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 12,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Greater,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: Default::default(),
        });

        render_pass.set_pipeline(&debug_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..debug_line_vertices.len() as u32, 0..1);
    }
    pub fn resize_textures<P: winit::dpi::Pixel>(
        &mut self,
        device: &wgpu::Device,
        size: PhysicalSize<P>,
        resources: &ResourceContext,
    ) {
        self.hdr.resize(
            device,
            size,
            &resources.bind_group_manager.bind_group_layouts,
        );
        self.depth_texture.resize(device, size);
    }
}

fn compute_frustum_corners(camera: &CameraHandler, near_plane: f32) -> [Vector3<f32>; 8] {
    let vp = camera.view_projection_matrix();
    let inv_vp = vp.invert().unwrap();

    let ndc_corners = [
        Vector3::new(-1.0, -1.0, -1.0), // Near bottom-left
        Vector3::new(1.0, -1.0, -1.0),  // Near bottom-right
        Vector3::new(1.0, 1.0, -1.0),   // Near top-right
        Vector3::new(-1.0, 1.0, -1.0),  // Near top-left
        Vector3::new(-1.0, -1.0, 1.0),  // Far bottom-left
        Vector3::new(1.0, -1.0, 1.0),   // Far bottom-right
        Vector3::new(1.0, 1.0, 1.0),    // Far top-right
        Vector3::new(-1.0, 1.0, 1.0),   // Far top-left
    ];

    let forward = camera.view.calculate_vectors().0;
    let offset_vector = forward * near_plane;

    ndc_corners.map(|corner| {
        let world = inv_vp * Vector4::new(corner.x, corner.y, corner.z, 1.0);
        let position = (world / world.w).truncate();
        position + offset_vector
    })
}
fn generate_frustum_edges(corners: &[Vector3<f32>; 8]) -> Vec<DebugLineVertex> {
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // Near plane
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // Far plane
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // Connecting edges
    ];

    edges
        .iter()
        .flat_map(|&(start, end)| {
            [
                DebugLineVertex {
                    position: corners[start].into(),
                    color: [1.0, 0.0, 0.0],
                },
                DebugLineVertex {
                    position: corners[end].into(),
                    color: [1.0, 0.0, 0.0],
                },
            ]
        })
        .collect()
}

pub struct RenderInfo {
    frame_metrics: FrameMetrics,
    debug_mode: DebugMode,
    primitive_topology: PrimitiveTopology,
}

impl RenderInfo {
    pub fn new(
        frame_metrics: FrameMetrics,
        debug_mode: DebugMode,
        primitive_topology: PrimitiveTopology,
    ) -> Self {
        Self {
            frame_metrics,
            debug_mode,
            primitive_topology,
        }
    }
    pub fn compute_metrics(&mut self) {
        self.frame_metrics.compute();
    }
    pub fn set_next_debug_mode(&mut self) {
        self.debug_mode = self.debug_mode.next();
        log_info!("Debug Mode: {:?}", self.debug_mode);
    }

    pub fn set_next_topology(&mut self) {
        self.primitive_topology = self.primitive_topology.next();
        log_info!("Render Mode: {:?}", self.primitive_topology);
    }

    pub fn frame_metrics(&self) -> &FrameMetrics {
        &self.frame_metrics
    }

    pub fn set_frame_metrics(&mut self, frame_metrics: FrameMetrics) {
        self.frame_metrics = frame_metrics;
    }

    pub fn debug_mode(&self) -> DebugMode {
        self.debug_mode
    }

    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        self.debug_mode = debug_mode;
    }

    pub fn render_mode(&self) -> PrimitiveTopology {
        self.primitive_topology
    }

    pub fn set_render_mode(&mut self, primitive_topology: PrimitiveTopology) {
        self.primitive_topology = primitive_topology;
    }
}
#[derive(Debug)]
pub struct BufferManager {
    pub buffers: HashCache<wgpu::Buffer>,
}

impl BufferManager {
    pub const DEFAULT_CAPACITY: usize = 100;

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffers: HashCache::new(),
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    fn create_buffer<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage,
        })
    }
    pub fn contains_buffer(&self, cache_id: &CacheKey) -> bool {
        self.buffers.contains(cache_id)
    }
    pub fn create_instance_buffer(
        &mut self,
        device: &wgpu::Device,
        instances: &[InstanceRaw],
        cache_id: CacheKey,
    ) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Instance buffer: {:?}", cache_id)),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let _ = self.buffers.put(cache_id, buffer);
    }

    pub fn update_instance_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instance_raw: &[InstanceRaw],
        id: CacheKey,
    ) {
        match self.buffers.get_or_create(id, || {
            Ok(BufferFactory::create_instance_buffer(device, &instance_raw))
        }) {
            Ok(buffer) => {
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&instance_raw));
            }
            Err(e) => {
                log_error!("{}", e);
            }
        }
    }
    pub fn get_instance_buffer(&self, id: CacheKey) -> Option<&wgpu::Buffer> {
        self.buffers.get(&id)
    }
    pub fn contains_instance_buffer(&self, id: &CacheKey) -> bool {
        self.buffers.contains(id)
    }
    pub fn create_vertex_buffer(
        &mut self,
        device: &wgpu::Device,
        vertices: &[VertexType],
        cache_id: CacheKey,
    ) {
        log_info!("Creating vertex buffer with id: {:?}", cache_id);
        let flat_data: Vec<u8> = vertices.iter().flat_map(|vertex| vertex.as_pod()).collect();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("vertex_buffer")),
            contents: &flat_data,
            usage: wgpu::BufferUsages::VERTEX,
        });

        let _ = self.buffers.put(cache_id, buffer);
    }

    pub fn create_index_buffer<T: bytemuck::Pod>(
        &mut self,
        device: &wgpu::Device,
        indices: &[T],
        cache_id: CacheKey,
    ) {
        log_info!("Creating index buffer with id: {:?}", cache_id);
        let _ = self.buffers.put(
            cache_id,
            Self::create_buffer(
                device,
                indices,
                wgpu::BufferUsages::INDEX,
                &format!("Index buffer: {:?}", cache_id),
            ),
        );
    }

    pub fn get_vertex_buffer(&self, id: &CacheKey) -> Result<&wgpu::Buffer, AppError> {
        self.buffers.get(id).ok_or_else(|| {
            AppError::ResourceNotFound(format!("Vertex buffer with ID {:?} not found", id))
        })
    }

    pub fn get_index_buffer(&self, id: &CacheKey) -> Result<&wgpu::Buffer, AppError> {
        self.buffers.get(id).ok_or_else(|| {
            AppError::ResourceNotFound(format!("Index buffer with ID {:?} not found", id))
        })
    }
    pub fn get_or_create_buffer(
        &mut self,
        cache_id: CacheKey,
        create_fn: impl FnOnce() -> Result<wgpu::Buffer, AppError>,
    ) -> std::result::Result<&mut wgpu::Buffer, AppError> {
        self.buffers.get_or_create(cache_id, create_fn)
    }
}
pub struct DebugLine {
    start: [f32; 3], // Line start position
    end: [f32; 3],   // Line end position
    color: [f32; 3], // Line color
}

pub struct BufferFactory;

impl BufferFactory {
    pub fn create_index_buffer<T: Pod>(device: &wgpu::Device, indices: &[T]) -> Buffer {
        Self::create_buffer(
            device,
            indices,
            BufferUsages::INDEX,
            "IndexBufferInitDescriptor",
        )
    }
    pub fn create_instance_buffer(device: &wgpu::Device, instance_data: &[InstanceRaw]) -> Buffer {
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        instance_buffer
    }
    pub fn create_camera_uniform_buffer(
        device: &wgpu::Device,
        uniform: CameraUniform,
    ) -> wgpu::Buffer {
        let uniform_data = &[uniform];
        Self::create_buffer(
            device,
            uniform_data,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            "CameraUniformBuffer",
        )
    }
    pub fn create_light_buffer(device: &wgpu::Device) -> Buffer {
        let data = &[LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0])];
        Self::create_buffer(
            device,
            data,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            "LightVertexBuffer",
        )
    }
}
impl BufferCreator for BufferFactory {
    fn create_buffer<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage,
        })
    }
}

fn set_bind_groups<'a>(render_pass: &mut wgpu::RenderPass<'a>, bind_groups: &[&wgpu::BindGroup]) {
    for (index, bind_group) in bind_groups.iter().enumerate() {
        render_pass.set_bind_group(index as u32, bind_group, &[]);
    }
}

impl<'a, 'b> RenderPassDraw<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model(
        &mut self,
        model: &Model,
        bind_groups: &[&'b wgpu::BindGroup],
        instances: &Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    ) {
        set_bind_groups(self, bind_groups);
        for mesh_id in &model.mesh_ids {
            if let Some(mesh) = mesh_manager.meshes.get(&mesh_id) {
                if let (Ok(vertex_buffer), Ok(index_buffer)) = (
                    buffer_manager.get_vertex_buffer(&mesh.vertex_buffer_key),
                    buffer_manager.get_index_buffer(&mesh.index_buffer_key),
                ) {
                    self.draw_mesh(
                        vertex_buffer,
                        index_buffer,
                        mesh,
                        bind_groups,
                        instances.clone(),
                    );
                } else {
                    log_warning!(
                        "Mesh with CacheId {:?} is missing vertex or index buffer.",
                        mesh_id
                    );
                }
            } else {
                log_warning!(
                    "Mesh with CacheId {:?} not found in the mesh manager.",
                    mesh_id
                );
            }
        }
    }
    fn draw_models(
        &mut self,
        models: &Vec<Model>,
        bind_groups: &[&'b wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    ) {
        set_bind_groups(self, bind_groups);

        for model in models {
            self.draw_model(model, bind_groups, &instances, buffer_manager, mesh_manager);
        }
    }
    fn draw_mesh(
        &mut self,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        mesh: &Mesh,
        bind_groups: &[&wgpu::BindGroup],
        instances: Option<Range<u32>>,
    ) {
        set_bind_groups(self, bind_groups);
        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        if let Some(instances) = instances {
            self.draw_indexed(0..mesh.num_elements, 0, instances);
        } else {
            self.draw_indexed(0..mesh.num_elements, 0, 0..1);
        }
    }

    fn draw_vertices(
        &mut self,
        bind_groups: &[&'b wgpu::BindGroup],
        vertices: Range<u32>,
        instances: Option<Range<u32>>,
    ) {
        set_bind_groups(self, bind_groups);
        if let Some(instances) = instances {
            self.draw(vertices, instances);
        } else {
            self.draw(vertices, 0..1);
        }
    }
}
