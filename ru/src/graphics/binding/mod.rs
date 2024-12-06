pub mod camera;
pub mod environment;
pub mod equirect;
pub mod light;
pub mod texture;

use camera::create_camera_bind_group_layout;
use environment::create_environment_bind_group_layout;
use light::create_light_bind_group_layout;
use texture::create_texture_bind_group_layout;
use wgpu::{BindGroupLayout, Device};

pub const INDEX_LIGHT_BIND_GROUP: isize = 0;
pub const INDEX_CAMERA_BIND_GROUP: isize = 1;
pub const INDEX_ENVIRONMENT_BIND_GROUP: isize = 2;

#[derive(Debug)]
pub enum CommonBindGroupIndex {
    Light = INDEX_LIGHT_BIND_GROUP,
    Camera = INDEX_CAMERA_BIND_GROUP,
    Environment = INDEX_ENVIRONMENT_BIND_GROUP,
}

impl CommonBindGroupIndex {
    pub fn as_index(&self) -> u32 {
        match self {
            CommonBindGroupIndex::Light => INDEX_LIGHT_BIND_GROUP.try_into().unwrap(),
            CommonBindGroupIndex::Camera => INDEX_CAMERA_BIND_GROUP.try_into().unwrap(),
            CommonBindGroupIndex::Environment => INDEX_ENVIRONMENT_BIND_GROUP.try_into().unwrap(),
        }
    }
}

pub struct SharedBindGroups {
    bind_groups: Vec<wgpu::BindGroup>,
}

impl SharedBindGroups {
    pub fn new() -> Self {
        Self {
            bind_groups: Vec::new(),
        }
    }

    pub fn insert(&mut self, bind_group_index: CommonBindGroupIndex, bind_group: wgpu::BindGroup) {
        self.bind_groups.insert(
            match bind_group_index {
                CommonBindGroupIndex::Light => INDEX_LIGHT_BIND_GROUP as usize,
                CommonBindGroupIndex::Camera => INDEX_CAMERA_BIND_GROUP as usize,
                CommonBindGroupIndex::Environment => INDEX_ENVIRONMENT_BIND_GROUP as usize,
            },
            bind_group,
        );
    }

    pub fn get(&self, index: usize) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(index)
    }

    pub fn len(&self) -> u32 {
        self.bind_groups.len() as u32
    }
}

pub struct BindGroupLayouts {
    pub texture_bind_group_layout: BindGroupLayout,
    pub camera_bind_group_layout: BindGroupLayout,
    pub light_bind_group_layout: BindGroupLayout,
    pub environment_bind_group_layout: BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &Device) -> Self {
        let texture_bind_group_layout = create_texture_bind_group_layout(device);
        let camera_bind_group_layout = create_camera_bind_group_layout(device);
        let light_bind_group_layout = create_light_bind_group_layout(device);
        let environment_bind_group_layout = create_environment_bind_group_layout(device);
        Self {
            texture_bind_group_layout,
            camera_bind_group_layout,
            light_bind_group_layout,
            environment_bind_group_layout,
        }
    }
}
