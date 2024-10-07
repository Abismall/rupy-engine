use crate::{
    log_debug, pipeline::builder::RenderPipelineBuilder, prelude::AppError,
    scene::texture::texture::Texture,
};
use std::{collections::HashMap, sync::Arc};
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline};

pub struct PipelineCache {
    pipelines: HashMap<u64, Arc<RenderPipeline>>,
    bind_group_layouts: HashMap<u64, Arc<BindGroupLayout>>,
    bind_groups: HashMap<u64, Arc<BindGroup>>,
}

impl PipelineCache {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            bind_groups: HashMap::new(),
        }
    }

    pub fn is_pipeline_cached(&self, pipeline_cache_key: u64) -> bool {
        self.pipelines.contains_key(&pipeline_cache_key)
    }

    pub fn get_or_create_pipeline(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache_key: u64,
        global_bind_group_layout: &wgpu::BindGroupLayout,
        mesh_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: Option<&wgpu::BindGroupLayout>,
        vertex_shader: &wgpu::ShaderModule,
        fragment_shader: &wgpu::ShaderModule,
        swap_chain_format: wgpu::TextureFormat,
    ) -> Result<Arc<RenderPipeline>, AppError> {
        let pipeline_cache = &mut self.pipelines;

        // Use the cache or create a new pipeline
        Ok(get_or_create(pipeline_cache_key, pipeline_cache, || {
            log_debug!("Pipeline not found in cache, creating new render pipeline.");

            let builder = RenderPipelineBuilder::new(device)
                .with_global_bind_group_layout(global_bind_group_layout)
                .with_mesh_bind_group_layout(mesh_bind_group_layout)
                .with_vertex_shader(vertex_shader)
                .with_fragment_shader(fragment_shader);

            let builder = if let Some(texture_layout) = texture_bind_group_layout {
                builder.with_texture_bind_group_layout(texture_layout)
            } else {
                builder
            };
            builder.build().unwrap()
        }))
    }

    pub fn get_or_create_texture_bind_group_layout(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
    ) -> Arc<BindGroupLayout> {
        let bind_group_layout_cache = &mut self.bind_group_layouts;
        get_or_create(bind_group_cache_key, bind_group_layout_cache, || {
            log_debug!("Bind group layout not found in cache, creating new bind group layout.");
            create_texture_bind_group_layout(device)
        })
    }

    pub fn get_or_create_texture_bind_group(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
        texture_bind_group_layout: Arc<BindGroupLayout>,
        texture: std::sync::Arc<Texture>,
    ) -> Result<Arc<BindGroup>, AppError> {
        Ok(get_or_create(
            bind_group_cache_key,
            &mut self.bind_groups,
            || {
                log_debug!("Bind group not found in cache, creating new bind group.");
                create_texture_bind_group(device, texture, texture_bind_group_layout)
            },
        ))
    }

    pub fn get_or_create_bind_group_layout(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
        entries: &[wgpu::BindGroupLayoutEntry],
    ) -> Arc<BindGroupLayout> {
        let bind_group_layout_cache = &mut self.bind_group_layouts;
        get_or_create(bind_group_cache_key, bind_group_layout_cache, || {
            log_debug!("Bind group layout not found in cache, creating new bind group layout.");
            create_bind_group_layout(device, entries)
        })
    }

    pub fn invalidate_pipeline_cache(&mut self, pipeline_cache_key: u64) {
        self.pipelines.remove(&pipeline_cache_key);
    }

    pub fn get_or_create_bind_group(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
        layout: Arc<BindGroupLayout>,
        entries: &[wgpu::BindGroupEntry],
    ) -> Result<Arc<BindGroup>, AppError> {
        if entries.is_empty() {
            log_debug!("Empty bind group entries passed.");
            return Err(AppError::NoBindGroupEntryError);
        }

        let bind_group_cache = &mut self.bind_groups;
        Ok(get_or_create(
            bind_group_cache_key,
            bind_group_cache,
            || {
                log_debug!("Bind group not found in cache, creating new bind group.");
                create_bind_group(device, layout, entries)
            },
        ))
    }
}

fn get_or_create<T, F>(cache_key: u64, cache: &mut HashMap<u64, Arc<T>>, create_fn: F) -> Arc<T>
where
    F: FnOnce() -> T,
{
    if let Some(cached) = cache.get(&cache_key) {
        return Arc::clone(cached);
    }

    let resource = Arc::new(create_fn());
    log_debug!("Storing newly created resource for key: {:?}", cache_key);
    cache.insert(cache_key, Arc::clone(&resource));
    resource
}

fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

fn create_texture_bind_group(
    device: &Device,
    texture: std::sync::Arc<Texture>,
    texture_bind_group_layout: Arc<BindGroupLayout>,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
        label: Some("Texture Bind Group"),
    })
}

fn create_bind_group(
    device: &Device,
    layout: Arc<BindGroupLayout>,
    entries: &[wgpu::BindGroupEntry],
) -> BindGroup {
    log_debug!("Creating new bind group.");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries,
        label: Some("Bind Group"),
    })
}

fn create_bind_group_layout(
    device: &Device,
    entries: &[wgpu::BindGroupLayoutEntry],
) -> BindGroupLayout {
    log_debug!("Creating new bind group layout.");
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Custom Bind Group Layout"),
        entries,
    })
}
