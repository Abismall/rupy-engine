use std::{collections::HashMap, ops::Range};

use wgpu::{util::DeviceExt, Buffer, BufferUsages, DepthStencilState};

use crate::{
    app::DebugMode,
    camera::{projection::Projection, Camera},
    core::{cache::CacheId, error::AppError, surface::RenderSurface},
    ecs::{
        components::{
            instance::model::{Instance, InstanceRaw},
            mesh::{manager::MeshManager, model::Mesh},
            model::{model::Model, prepare_model_bind_groups},
        },
        resources::ResourceManager,
        traits::{BufferCreator, Cache, RenderPassDraw},
        world::World,
    },
    graphics::{
        binding::{
            BindGroupLayouts, SharedBindGroups, INDEX_CAMERA_BIND_GROUP,
            INDEX_ENVIRONMENT_BIND_GROUP, INDEX_LIGHT_BIND_GROUP,
        },
        context::GpuContext,
        glyphon::GlyphonRender,
        model::{CameraUniform, LightUniform, VertexType},
        pipelines::hdr,
        textures::Texture,
        PrimitiveTopology,
    },
    log_info, log_warning,
    prelude::frame::FrameTime,
};
pub struct Renderer3D {
    pub ctx: RenderInfo,
    pub hdr: hdr::HdrPipeline,
    pub depth_texture: Texture,
    pub depth_stencil: DepthStencilState,
    pub glyphon: GlyphonRender,
    pub bind_group_layouts: BindGroupLayouts,
    pub shared_bind_groups: SharedBindGroups,
}

impl Renderer3D {
    pub fn new(
        ctx: RenderInfo,
        hdr: hdr::HdrPipeline,
        depth_texture: Texture,
        depth_stencil: DepthStencilState,
        glyphon: GlyphonRender,
        bind_group_layouts: BindGroupLayouts,
        shared_bind_groups: SharedBindGroups,
    ) -> Self {
        Self {
            ctx,
            hdr,
            depth_texture,
            depth_stencil,
            glyphon,
            bind_group_layouts,
            shared_bind_groups,
        }
    }

    pub fn render(
        &mut self,
        gpu: &GpuContext,
        target: &RenderSurface,
        world: &mut World,
        resources: &mut ResourceManager,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = target.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

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
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let pipeline_label = self.ctx.render_mode().label();
            let sky_pipeline = resources
                .pipeline_manager
                .get(CacheId::from("sky_pipeline").value())
                .expect("Sky pipeline not found");
            let render_pipeline = resources
                .pipeline_manager
                .get(CacheId::from(format!("{}_pipeline", pipeline_label).as_str()).value())
                .expect("Render pipeline not found for topology");
            let light_pipeline = resources
                .pipeline_manager
                .get(CacheId::from(format!("{}_light_pipeline", pipeline_label).as_str()).value())
                .expect("Light pipeline not found");

            render_pass.set_pipeline(&sky_pipeline);
            if let (Some(camera_bind_group), Some(environment_bind_group), Some(light_bind_group)) = (
                self.shared_bind_groups
                    .get(INDEX_CAMERA_BIND_GROUP.try_into().unwrap()),
                self.shared_bind_groups
                    .get(INDEX_ENVIRONMENT_BIND_GROUP.try_into().unwrap()),
                self.shared_bind_groups
                    .get(INDEX_LIGHT_BIND_GROUP.try_into().unwrap()),
            ) {
                render_pass.draw_vertices(
                    &[camera_bind_group, environment_bind_group],
                    0..3,
                    Some(0..1),
                );

                world.query::<Vec<Instance>>(|entity, instances| {
                    let cache_id = CacheId::from(entity);
                    if let Some(model) = resources.model_manager.get(cache_id.value()) {
                        render_pass.set_pipeline(&light_pipeline);
                        render_pass.draw_model(
                            cache_id.into(),
                            model,
                            &[camera_bind_group, light_bind_group],
                            None,
                            &resources.buffer_manager,
                            &resources.mesh_manager,
                        );

                        let model_bind_group_vec = prepare_model_bind_groups(
                            &model.material_ids,
                            &[camera_bind_group, light_bind_group],
                            &resources.material_manager,
                        );

                        if let Ok(cube_instance_buffer) = resources
                            .buffer_manager
                            .get_instance_buffer(cache_id.value())
                            .map_err(|_| wgpu::SurfaceError::Outdated)
                        {
                            render_pass.set_pipeline(&render_pipeline);
                            render_pass.set_vertex_buffer(1, cube_instance_buffer.slice(..));
                            render_pass.draw_model(
                                cache_id.into(),
                                model,
                                &model_bind_group_vec,
                                Some(0..instances.len() as u32),
                                &resources.buffer_manager,
                                &resources.mesh_manager,
                            );
                        }
                    }
                });
            }

            if self.ctx.frame_time().frame_count % 3 == 0
                && self.ctx.debug_mode() != DebugMode::None
            {
                self.glyphon.render(
                    &mut render_pass,
                    &gpu.device(),
                    &gpu.queue(),
                    &target.config,
                    self.depth_stencil.depth_write_enabled,
                );
            }
        }

        self.hdr.process(&mut encoder, &view);
        gpu.queue().submit(std::iter::once(encoder.finish()));
        output.present();
        self.glyphon.clear_buffer();

        Ok(())
    }
}

pub struct RenderInfo {
    frame: FrameTime,
    debug: DebugMode,
    primitive_topology: PrimitiveTopology,
    draws_per_frame: u32,
    vertices_per_frame: u32,
}

impl RenderInfo {
    pub fn new(frame: FrameTime, debug: DebugMode, primitive_topology: PrimitiveTopology) -> Self {
        Self {
            frame,
            debug,
            primitive_topology,
            draws_per_frame: 0,
            vertices_per_frame: 0,
        }
    }
    pub fn reset_frame_stats(&mut self) {
        self.draws_per_frame = 0;
        self.vertices_per_frame = 0;
    }

    pub fn add_draw_call(&mut self, count: u32) {
        self.draws_per_frame += count;
    }

    pub fn add_vertices(&mut self, count: u32) {
        self.vertices_per_frame += count;
    }

    pub fn frame_stats(&self) -> (u32, u32) {
        (self.draws_per_frame, self.vertices_per_frame)
    }
    pub fn update(&mut self) {
        self.frame.compute();
    }
    pub fn set_next_debug_mode(&mut self) {
        self.debug = self.debug.next();
        log_info!("Debug Mode: {:?}", self.debug);
    }

    pub fn set_next_topology(&mut self) {
        self.primitive_topology = self.primitive_topology.next();
        log_info!("Render Mode: {:?}", self.primitive_topology);
    }

    pub fn frame_time(&self) -> &FrameTime {
        &self.frame
    }

    pub fn set_frame_time(&mut self, frame: FrameTime) {
        self.frame = frame;
    }

    pub fn debug_mode(&self) -> DebugMode {
        self.debug
    }

    pub fn set_debug_mode(&mut self, debug: DebugMode) {
        self.debug = debug;
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
    vertex_buffers: HashMap<u64, wgpu::Buffer>,
    index_buffers: HashMap<u64, wgpu::Buffer>,
    instance_buffers: HashMap<u64, wgpu::Buffer>,

    camera_buffer: Option<wgpu::Buffer>,
    light_buffer: Option<wgpu::Buffer>,
}

impl BufferManager {
    pub const DEFAULT_CAPACITY: usize = 100;

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertex_buffers: HashMap::with_capacity(capacity),
            index_buffers: HashMap::with_capacity(capacity),
            instance_buffers: HashMap::with_capacity(capacity),
            camera_buffer: None,
            light_buffer: None,
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
    pub fn create_instance_buffer(
        &mut self,
        device: &wgpu::Device,
        instances: &[Instance],
        cache_id: u64,
    ) {
        let raw_instances: Vec<InstanceRaw> = instances.iter().map(|inst| inst.to_raw()).collect();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Instance buffer: {:?}", cache_id)),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        self.instance_buffers.insert(cache_id, buffer);
    }

    pub fn update_instance_buffer(
        &self,
        queue: &wgpu::Queue,
        instances: &[Instance],
        id: u64,
    ) -> Result<(), AppError> {
        if let Some(buffer) = self.instance_buffers.get(&id) {
            let raw_instances: Vec<InstanceRaw> =
                instances.iter().map(|inst| inst.to_raw()).collect();
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&raw_instances));
            Ok(())
        } else {
            Err(AppError::BufferNotFoundError(format!(
                "Instance buffer with ID {:?} not found",
                id
            )))
        }
    }

    pub fn get_instance_buffer(&self, id: u64) -> Result<&wgpu::Buffer, AppError> {
        self.instance_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Instance buffer with ID {:?} not found", id))
        })
    }
    pub fn create_vertex_buffer(
        &mut self,
        device: &wgpu::Device,
        vertices: &[VertexType],
        cache_id: u64,
    ) {
        let flat_data: Vec<u8> = vertices.iter().flat_map(|vertex| vertex.as_pod()).collect();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Vertex buffer: {:?}", cache_id)),
            contents: &flat_data,
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_buffers.insert(cache_id, buffer);
    }

    pub fn create_index_buffer(&mut self, device: &wgpu::Device, indices: &[u16], cache_id: u64) {
        let buffer = Self::create_buffer(
            device,
            indices,
            wgpu::BufferUsages::INDEX,
            &format!("Index buffer: {:?}", cache_id),
        );
        self.index_buffers.insert(cache_id, buffer);
    }

    pub fn get_vertex_buffer(&self, id: u64) -> Result<&wgpu::Buffer, AppError> {
        self.vertex_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Vertex buffer with ID {:?} not found", id))
        })
    }

    pub fn get_index_buffer(&self, id: u64) -> Result<&wgpu::Buffer, AppError> {
        self.index_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Index buffer with ID {:?} not found", id))
        })
    }

    pub fn get_camera_buffer(&self) -> &Option<Buffer> {
        &self.camera_buffer
    }

    pub fn get_light_buffer(&self) -> &Option<Buffer> {
        &self.light_buffer
    }
    pub fn set_camera_buffer(&mut self, buffer: wgpu::Buffer) {
        self.camera_buffer = Some(buffer);
    }

    pub fn set_light_buffer(&mut self, buffer: wgpu::Buffer) {
        self.light_buffer = Some(buffer)
    }
    pub fn list_buffers(&self) -> Vec<String> {
        self.vertex_buffers
            .keys()
            .chain(self.index_buffers.keys())
            .chain(self.instance_buffers.keys())
            .map(|id| format!("{:?}", id))
            .collect()
    }
}

pub struct BufferFactory;

impl BufferFactory {
    pub fn index_buffer(device: &wgpu::Device, indices: &[u16]) -> Buffer {
        Self::create_buffer(
            device,
            indices,
            BufferUsages::INDEX,
            "IndexBufferInitDescriptor",
        )
    }
    pub fn instance_buffer(device: &wgpu::Device, instance_data: &Vec<Instance>) -> Buffer {
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(
                &instance_data
                    .iter()
                    .map(Instance::to_raw)
                    .collect::<Vec<_>>(),
            ),
            usage: wgpu::BufferUsages::VERTEX,
        });
        instance_buffer
    }
    pub fn camera_buffer(
        device: &wgpu::Device,
        camera: &Camera,
        projection: &Projection,
    ) -> Buffer {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
        Self::create_buffer(
            device,
            &[camera_uniform],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            "CameraUniformBuffer",
        )
    }
    pub fn light_buffer(device: &wgpu::Device) -> Buffer {
        Self::create_buffer(
            device,
            &[LightUniform {
                position: [2.0, 2.0, 2.0],
                _padding: 0,
                color: [1.0, 1.0, 1.0],
                _padding2: 0,
            }],
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
        cache_id: u64,
        model: &Model,
        bind_groups: &[&'b wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    ) {
        set_bind_groups(self, bind_groups);

        for mesh_id in &model.mesh_ids {
            if let Some(mesh) = mesh_manager.get(mesh_id.value()) {
                if let (Ok(vertex_buffer), Ok(index_buffer)) = (
                    buffer_manager.get_vertex_buffer(cache_id),
                    buffer_manager.get_index_buffer(cache_id),
                ) {
                    self.set_vertex_buffer(0, vertex_buffer.slice(..));
                    self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                    if let Some(instances) = &instances {
                        self.draw_indexed(0..mesh.num_elements, 0, instances.clone());
                    } else {
                        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
                    }
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

    fn draw_mesh(
        &mut self,
        cache_id: u64,
        mesh: &'b Mesh,
        bind_groups: &[&'b wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: BufferManager,
    ) {
        if let (Ok(vertex_buffer), Ok(index_buffer)) = (
            buffer_manager.get_vertex_buffer(cache_id),
            buffer_manager.get_index_buffer(cache_id),
        ) {
            self.set_vertex_buffer(0, vertex_buffer.slice(..));
            self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            set_bind_groups(self, bind_groups);
            if let Some(instances) = instances {
                self.draw_indexed(0..mesh.num_elements, 0, instances);
            } else {
                self.draw_indexed(0..mesh.num_elements, 0, 0..1);
            }
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
