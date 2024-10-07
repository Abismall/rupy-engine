use crate::{ecs::vertex::Vertex, prelude::AppError};

pub struct RenderPipelineBuilder<'a> {
    device: &'a wgpu::Device,
    global_bind_group_layout: Option<&'a wgpu::BindGroupLayout>,
    mesh_bind_group_layout: Option<&'a wgpu::BindGroupLayout>,
    camera_bind_group_layout: Option<&'a wgpu::BindGroupLayout>,
    texture_bind_group_layout: Option<&'a wgpu::BindGroupLayout>,
    use_texture: bool,
    use_lighting: bool,
    vertex_shader: Option<&'a wgpu::ShaderModule>,
    fragment_shader: Option<&'a wgpu::ShaderModule>,
}

impl<'a> RenderPipelineBuilder<'a> {
    // Constructor
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            device,
            global_bind_group_layout: None,
            camera_bind_group_layout: None,
            mesh_bind_group_layout: None,
            texture_bind_group_layout: None,
            use_texture: false,
            use_lighting: false,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    // Set the global bind group layout
    pub fn with_global_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.global_bind_group_layout = Some(layout);
        self
    }
    pub fn with_camera_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.camera_bind_group_layout = Some(layout);
        self
    }
    // Set the mesh bind group layout
    pub fn with_mesh_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.mesh_bind_group_layout = Some(layout);
        self
    }

    // Set the texture bind group layout
    pub fn with_texture_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.texture_bind_group_layout = Some(layout);
        self.use_texture = true; // Enable texture support
        self
    }

    // Set the vertex shader
    pub fn with_vertex_shader(mut self, shader: &'a wgpu::ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    // Set the fragment shader
    pub fn with_fragment_shader(mut self, shader: &'a wgpu::ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }

    // Enable lighting support (could add more bindings for lights)
    pub fn with_lighting(mut self) -> Self {
        self.use_lighting = true;
        self
    }

    // Build the pipeline
    pub fn build(self) -> Result<wgpu::RenderPipeline, AppError> {
        let global_bind_group_layout = self
            .global_bind_group_layout
            .ok_or_else(|| AppError::NoBindGroupEntryError)?;

        let mesh_bind_group_layout = self
            .mesh_bind_group_layout
            .ok_or_else(|| AppError::NoBindGroupEntryError)?;

        let mut bind_group_layouts = vec![global_bind_group_layout, mesh_bind_group_layout];

        if self.use_texture {
            let texture_bind_group_layout = self
                .texture_bind_group_layout
                .ok_or_else(|| AppError::NoBindGroupEntryError)?;
            bind_group_layouts.push(texture_bind_group_layout);
        }

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &[],
            });

        let vertex_shader = self
            .vertex_shader
            .ok_or_else(|| AppError::ShaderSourceFileError("Vertex shader missing".to_string()))?;

        let fragment_shader = self.fragment_shader.ok_or_else(|| {
            AppError::ShaderSourceFileError("Fragment shader missing".to_string())
        })?;

        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: vertex_shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: fragment_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb, // Adjust according to your needs
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: Default::default(),
            });

        Ok(pipeline)
    }
}

pub fn create_textured_pipeline(
    device: &wgpu::Device,
    global_layout: &wgpu::BindGroupLayout,
    mesh_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
) -> Result<wgpu::RenderPipeline, AppError> {
    RenderPipelineBuilder::new(device)
        .with_global_bind_group_layout(global_layout)
        .with_mesh_bind_group_layout(mesh_layout)
        .with_texture_bind_group_layout(texture_layout)
        .with_vertex_shader(vertex_shader)
        .with_fragment_shader(fragment_shader)
        .build()
}

pub fn create_untextured_pipeline(
    device: &wgpu::Device,
    global_layout: &wgpu::BindGroupLayout,
    mesh_layout: &wgpu::BindGroupLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
) -> Result<wgpu::RenderPipeline, AppError> {
    RenderPipelineBuilder::new(device)
        .with_global_bind_group_layout(global_layout)
        .with_mesh_bind_group_layout(mesh_layout)
        .with_vertex_shader(vertex_shader)
        .with_fragment_shader(fragment_shader)
        .build()
}
