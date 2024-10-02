use crate::{
    log_debug,
    math::{
        mat4_id,
        spatial::{new_nonuniform_scaling, new_rotation},
        vector::vec3_to_mat4_translation,
        Mat4,
    },
    pipeline::cache::PipelineCache,
    render::Renderable,
};

use vecmath::col_mat4_mul;
use wgpu::{Buffer, ShaderModule};
pub struct Object {
    pub model_matrix: Mat4, // Each object instance has its own transformation
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub mesh: Box<dyn Renderable>, // Use a trait object for dynamic dispatch
    pub pipeline_cache_key: u64,
}

impl Object {
    pub fn new(
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        mesh: Box<dyn Renderable>, // Dynamic vertex data
        pipeline_cache_key: u64,
    ) -> Self {
        Object {
            model_matrix: mat4_id(),
            position,
            rotation,
            scale,
            mesh,
            pipeline_cache_key,
        }
    }

    pub fn update_model_matrix(&mut self) {
        let translation = vec3_to_mat4_translation(self.position);
        let rotation = new_rotation(self.rotation);
        let scaling = new_nonuniform_scaling(self.scale);
        self.model_matrix = col_mat4_mul(col_mat4_mul(translation, rotation), scaling);
    }
}

impl Renderable for Object {
    fn render(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        swapchain_format: wgpu::TextureFormat,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        global_bind_group: &wgpu::BindGroup,
    ) {
        log_debug!("Object render called");

        // Ensure buffers are created
        self.mesh.create_buffers(device);

        // Create or get cached pipeline and layout
        let bind_group_layout = pipeline_cache.get_or_create_bind_group_layout(
            device,
            self.pipeline_cache_key,
            &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        );

        let pipeline = pipeline_cache.get_or_create_pipeline(
            device,
            self.pipeline_cache_key,
            vertex_shader_src,
            fragment_shader_src,
            &bind_group_layout,
            swapchain_format,
        );

        // Start the render pass
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
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
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Bind global uniform data
        render_pass.set_bind_group(0, global_bind_group, &[]);

        // Bind the vertex and index buffers
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer().slice(..));
        render_pass.set_index_buffer(
            self.mesh.index_buffer().slice(..),
            wgpu::IndexFormat::Uint32,
        );

        // Bind the pipeline
        render_pass.set_pipeline(&pipeline);

        // Issue the draw command
        render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..1);
    }

    fn create_buffers(&mut self, device: &wgpu::Device) {
        todo!()
    }

    fn vertex_buffer(&self) -> &wgpu::Buffer {
        todo!()
    }

    fn index_buffer(&self) -> &wgpu::Buffer {
        todo!()
    }

    fn num_indices(&self) -> u32 {
        todo!()
    }

    fn is_textured(&self) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    // Implement other required methods...
}

#[derive(Default)]
pub struct ObjectManager {
    objects: Vec<Object>, // No more generics
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, index: usize) {
        if index < self.objects.len() {
            log_debug!("Removing: {}", index);
            self.objects.remove(index);
        }
    }

    pub fn update_object_model_matrices(&mut self) {
        self.objects
            .iter_mut()
            .for_each(|f| f.update_model_matrix());
        log_debug!("Updated object model matrices.");
    }

    pub fn get_render_data(&self) -> &[Object] {
        &self.objects
    }

    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }

    pub fn get_object(&self, index: usize) -> Option<&Object> {
        self.objects.get(index)
    }

    pub fn object_count(&self) -> usize {
        let count = self.objects.len();
        log_debug!("Object count: {}", count);
        count
    }
}
pub struct ObjectDescription {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub mesh: Box<dyn Renderable>,
    pub pipeline_cache_key: u64,
}
