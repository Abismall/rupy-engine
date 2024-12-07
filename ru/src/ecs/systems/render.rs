use std::{collections::HashMap, ops::Range};

use cgmath::Vector3;
use wgpu::{util::DeviceExt, BindGroup, Buffer, BufferUsages};
use winit::dpi::PhysicalSize;

use crate::{
    app::DebugMode,
    camera::{frustum::Frustum, CameraHandler},
    core::{cache::ComponentCacheKey, error::AppError, surface::RenderSurface},
    ecs::{
        components::{
            instance::model::{Instance, InstanceRaw},
            mesh::{manager::MeshManager, model::Mesh},
            model::model::Model,
            IntoComponentCacheKey,
        },
        traits::{BufferCreator, Cache, RenderPassDraw},
        world::World,
    },
    graphics::{
        binding::{
            BindGroupLayouts, INDEX_CAMERA_BIND_GROUP, INDEX_ENVIRONMENT_BIND_GROUP,
            INDEX_LIGHT_BIND_GROUP,
        },
        context::GpuResourceCache,
        depth::DepthBuffer,
        glyphon::GlyphonRender,
        model::{CameraUniform, LightUniform, VertexType},
        pipelines::hdr,
        PrimitiveTopology, ResourceManager,
    },
    log_debug, log_info, log_warning,
    prelude::frame::FrameMetrics,
};
pub struct Renderer3D {
    pub ctx: RenderInfo,
    pub hdr: hdr::HdrPipeline,
    pub depth_buffer: DepthBuffer,
    pub glyphon: GlyphonRender,
    pub bind_group_layouts: BindGroupLayouts,
    pub bind_groups: Vec<BindGroup>,
}

impl Renderer3D {
    pub fn new(
        ctx: RenderInfo,
        hdr: hdr::HdrPipeline,
        depth_buffer: DepthBuffer,
        glyphon: GlyphonRender,
        bind_group_layouts: BindGroupLayouts,
        bind_groups: Vec<BindGroup>,
    ) -> Self {
        Self {
            ctx,
            hdr,
            depth_buffer,
            glyphon,
            bind_group_layouts,
            bind_groups,
        }
    }

    pub fn render(
        &mut self,
        gpu: &GpuResourceCache,
        target: &RenderSurface,
        world: &mut World,
        resources: &mut ResourceManager,
        camera_handler: &CameraHandler,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = target.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let device = gpu.device();
        let queue = gpu.queue();
        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let view_proj_matrix =
            camera_handler.camera.view_matrix() * camera_handler.projection.calc_matrix();
        let frustum = Frustum::from_view_projection_matrix(view_proj_matrix.into());

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
                    view: &self.depth_buffer.view,
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
            let bind_groups = (
                &self.bind_groups[INDEX_CAMERA_BIND_GROUP as usize],
                &self.bind_groups[INDEX_ENVIRONMENT_BIND_GROUP as usize],
                &self.bind_groups[INDEX_LIGHT_BIND_GROUP as usize],
            );

            render_pass.set_pipeline(
                &resources
                    .pipeline_manager
                    .get(ComponentCacheKey::from("sky_pipeline"))
                    .expect("Sky pipeline not found"),
            );

            render_pass.draw_vertices(&[bind_groups.0, bind_groups.1], 0..3, Some(0..1));
            let light_pipeline = resources
                .pipeline_manager
                .get(ComponentCacheKey::from(
                    format!("{}_light_pipeline", pipeline_label).as_str(),
                ))
                .expect("Light pipeline not found");
            let _ = world.query::<Instance>(|entity, instances| {
                log_debug!("Rendering entity: {:?}", entity);
                let cache_id = ComponentCacheKey::from(entity);
                if let Some(model) = resources.model_manager.get(cache_id) {
                    let instance_positions = Vector3::new(
                        instances.position[0],
                        instances.position[1],
                        instances.position[2],
                    );

                    if frustum.contains_sphere(instance_positions, 1.0) {
                        return;
                    }
                    render_pass.set_pipeline(&light_pipeline);
                    render_pass.draw_model(
                        &model,
                        &[&bind_groups.0, &bind_groups.1, &bind_groups.2],
                        Some(0..1 as u32),
                        &resources.buffer_manager,
                        &resources.mesh_manager,
                    );
                }
            });
            if self.ctx.frame_metrics().frame_count % self.glyphon.interval == 0
                && self.ctx.debug_mode() != DebugMode::None
            {
                self.glyphon.render(
                    &mut render_pass,
                    &device,
                    &queue,
                    &target.config,
                    self.depth_buffer.stencil_state.depth_write_enabled,
                );
            }
        }

        self.hdr.process(&mut encoder, &view);
        gpu.queue().submit(std::iter::once(encoder.finish()));
        output.present();
        self.glyphon.clear_buffer();

        Ok(())
    }
    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        size: PhysicalSize<u32>,
    ) {
        self.hdr.resize(device, size, config.format);
        self.depth_buffer.resize(device, size);
    }
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
    pub fn compute(&mut self) {
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
    vertex_buffers: HashMap<ComponentCacheKey, wgpu::Buffer>,
    index_buffers: HashMap<ComponentCacheKey, wgpu::Buffer>,
    instance_buffers: HashMap<ComponentCacheKey, wgpu::Buffer>,

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
        cache_id: ComponentCacheKey,
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
        id: ComponentCacheKey,
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

    pub fn get_instance_buffer(&self, id: ComponentCacheKey) -> Result<&wgpu::Buffer, AppError> {
        self.instance_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Instance buffer with ID {:?} not found", id))
        })
    }
    pub fn create_vertex_buffer(
        &mut self,
        device: &wgpu::Device,
        vertices: &[VertexType],
        cache_id: ComponentCacheKey,
    ) {
        let flat_data: Vec<u8> = vertices.iter().flat_map(|vertex| vertex.as_pod()).collect();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("vertex_buffer")),
            contents: &flat_data,
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_buffers.insert(cache_id, buffer);
    }

    pub fn create_index_buffer(
        &mut self,
        device: &wgpu::Device,
        indices: &[u16],
        cache_id: ComponentCacheKey,
    ) {
        let buffer = Self::create_buffer(
            device,
            indices,
            wgpu::BufferUsages::INDEX,
            &format!("Index buffer: {:?}", cache_id),
        );
        self.index_buffers.insert(cache_id, buffer);
    }

    pub fn get_vertex_buffer(&self, id: ComponentCacheKey) -> Result<&wgpu::Buffer, AppError> {
        self.vertex_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Vertex buffer with ID {:?} not found", id))
        })
    }

    pub fn get_index_buffer(&self, id: ComponentCacheKey) -> Result<&wgpu::Buffer, AppError> {
        self.index_buffers.get(&id).ok_or_else(|| {
            AppError::BufferNotFoundError(format!("Index buffer with ID {:?} not found", id))
        })
    }

    pub fn get_camera_buffer(&self) -> Option<&wgpu::Buffer> {
        self.camera_buffer.as_ref()
    }
    pub fn contains_camera_buffer(&self) -> bool {
        self.camera_buffer.is_some()
    }
    pub fn set_camera_buffer(&mut self, buffer: wgpu::Buffer) {
        self.camera_buffer = Some(buffer);
    }
    pub fn get_light_buffer(&self) -> Option<&wgpu::Buffer> {
        self.light_buffer.as_ref()
    }
    pub fn contains_light_buffer(&self) -> bool {
        self.light_buffer.is_some()
    }
    pub fn set_light_buffer(&mut self, buffer: wgpu::Buffer) {
        self.light_buffer = Some(buffer);
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
    pub fn create_camera_uniform_buffer(device: &wgpu::Device) -> Buffer {
        let camera_uniform = CameraUniform::new();
        Self::create_buffer(
            device,
            &[camera_uniform],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            "CameraUniformBuffer",
        )
    }
    pub fn create_light_buffer(device: &wgpu::Device) -> Buffer {
        Self::create_buffer(
            device,
            &[LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0])],
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
        instances: Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    ) {
        set_bind_groups(self, bind_groups);
        let model_cache_key = model.into_cache_key();

        for mesh_id in &model.mesh_ids {
            if let Some(mesh) = mesh_manager.get(*mesh_id) {
                if let (Ok(vertex_buffer), Ok(index_buffer)) = (
                    buffer_manager.get_vertex_buffer(model_cache_key),
                    buffer_manager.get_index_buffer(model_cache_key),
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
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        mesh: &Mesh,
        bind_groups: &[&wgpu::BindGroup],
        instances: Option<Range<u32>>,
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
