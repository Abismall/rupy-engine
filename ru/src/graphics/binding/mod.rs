pub mod camera;
pub mod equirect;
pub mod hdr;
pub mod light;
pub mod material;
pub mod skybox;
pub mod texture;
use camera::{create_camera_bind_group, create_camera_bind_group_layout};
use equirect::create_equirect_bind_group_layout;
use hdr::create_hdr_pipeline_bind_group_layout;
use light::{create_light_bind_group, create_light_bind_group_layout};
use skybox::{create_skybox_bind_group, create_skybox_bind_group_layout};

use texture::create_texture_bind_group_layout;
use wgpu::{BindGroup, BindGroupLayout, Device, TextureFormat};

use crate::{
    core::error::AppError,
    ecs::{
        systems::render::{BufferFactory, BufferManager},
        traits::Cache,
    },
    prelude::cache::{CacheKey, HashCache},
};

use super::{textures::cube_texture::CubeTexture, uniform::Uniforms};

pub const INDEX_LIGHT_BIND_GROUP: isize = 0;
pub const INDEX_CAMERA_BIND_GROUP: isize = 1;
pub const INDEX_ENVIRONMENT_BIND_GROUP: isize = 2;

#[derive(Debug)]
pub struct BindGroupManager {
    pub bind_groups: HashCache<BindGroup>,
    pub bind_group_layouts: BindGroupLayouts,
}
pub fn initialize_common_bind_groups(
    device: &wgpu::Device,
    uniforms: &Uniforms,
    sky_texture: CubeTexture,
    environment_bg_cache_key: CacheKey,
    light_bg_cache_key: CacheKey,
    camera_bg_cache_key: CacheKey,
    buffer_manager: &mut BufferManager,
    bind_group_manager: &mut BindGroupManager,
) -> Result<(), AppError> {
    bind_group_manager
        .bind_groups
        .get_or_create(light_bg_cache_key, || {
            Ok(create_light_bind_group(
                device,
                buffer_manager.get_or_create_buffer(light_bg_cache_key, || {
                    Ok(BufferFactory::create_light_buffer(device))
                })?,
            ))
        })?;
    bind_group_manager
        .bind_groups
        .get_or_create(camera_bg_cache_key, || {
            Ok(create_camera_bind_group(
                device,
                buffer_manager.get_or_create_buffer(camera_bg_cache_key, || {
                    Ok(BufferFactory::create_camera_uniform_buffer(
                        device,
                        uniforms.camera,
                    ))
                })?,
            ))
        })?;

    bind_group_manager
        .bind_groups
        .get_or_create(environment_bg_cache_key, || {
            Ok(create_skybox_bind_group(device, &sky_texture))
        })?;

    Ok(())
}
impl BindGroupManager {
    pub fn new(bind_group_layouts: BindGroupLayouts) -> Self {
        Self {
            bind_groups: HashCache::new(),
            bind_group_layouts,
        }
    }
    pub fn layouts(&self) -> &BindGroupLayouts {
        &self.bind_group_layouts
    }
}

#[derive(Debug)]
pub struct BindGroupLayouts {
    pub texture_bind_group_layout: BindGroupLayout,
    pub camera_bind_group_layout: BindGroupLayout,
    pub light_bind_group_layout: BindGroupLayout,
    pub skybox_bind_group_layout: BindGroupLayout,
    pub equirect_bind_group_layout: BindGroupLayout,
    pub hdr_pipeline_bind_group_layout: BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(
        texture_bind_group_layout: BindGroupLayout,
        camera_bind_group_layout: BindGroupLayout,
        light_bind_group_layout: BindGroupLayout,
        skybox_bind_group_layout: BindGroupLayout,
        equirect_bind_group_layout: BindGroupLayout,
        hdr_pipeline_bind_group_layout: BindGroupLayout,
    ) -> Self {
        Self {
            texture_bind_group_layout,
            camera_bind_group_layout,
            light_bind_group_layout,
            skybox_bind_group_layout,
            equirect_bind_group_layout,
            hdr_pipeline_bind_group_layout,
        }
    }
}

pub fn setup_bind_group_layouts(
    device: &Device,
    equirect_texture_format: TextureFormat,
) -> BindGroupLayouts {
    let texture_bind_group_layout = create_texture_bind_group_layout(device, true);
    let camera_bind_group_layout = create_camera_bind_group_layout(device);
    let light_bind_group_layout = create_light_bind_group_layout(device);
    let skybox_bind_group_layout = create_skybox_bind_group_layout(device);
    let hdr_pipeline_bind_group_layout = create_hdr_pipeline_bind_group_layout(device);
    let equirect_bind_group_layout =
        create_equirect_bind_group_layout(device, equirect_texture_format);

    BindGroupLayouts::new(
        texture_bind_group_layout,
        camera_bind_group_layout,
        light_bind_group_layout,
        skybox_bind_group_layout,
        equirect_bind_group_layout,
        hdr_pipeline_bind_group_layout,
    )
}
