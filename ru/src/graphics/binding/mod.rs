pub mod camera;
pub mod environment;
pub mod equirect;
pub mod light;
pub mod texture;

use std::collections::HashMap;

use camera::create_camera_bind_group_layout;
use environment::create_environment_bind_group_layout;
use light::create_light_bind_group_layout;
use texture::create_texture_bind_group_layout;
use wgpu::{BindGroup, BindGroupLayout, Device};

use crate::{
    core::{cache::ComponentCacheKey, error::AppError},
    ecs::traits::Cache,
};

pub const INDEX_LIGHT_BIND_GROUP: isize = 0;
pub const INDEX_CAMERA_BIND_GROUP: isize = 1;
pub const INDEX_ENVIRONMENT_BIND_GROUP: isize = 2;
#[derive(Default, Debug)]
pub struct BindGroupManager {
    bind_groups: HashMap<ComponentCacheKey, BindGroup>,
}

impl BindGroupManager {
    pub fn new() -> Self {
        Self {
            bind_groups: HashMap::new(),
        }
    }
}

impl Cache<BindGroup> for BindGroupManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&BindGroup> {
        self.bind_groups.get(&id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.bind_groups.contains_key(&id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut BindGroup> {
        self.bind_groups.get_mut(&id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut BindGroup, AppError>
    where
        F: FnOnce() -> Result<BindGroup, AppError>,
    {
        if !self.bind_groups.contains_key(&id) {
            let bind_group = create_fn()?;
            self.bind_groups.insert(id, bind_group);
        }
        self.bind_groups
            .get_mut(&id)
            .ok_or(AppError::ResourceNotFound)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: BindGroup) -> Result<(), AppError> {
        if self.bind_groups.contains_key(&id) {
            return Err(AppError::DuplicateResource);
        }
        self.bind_groups.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.bind_groups.remove(&id);
    }
}
#[derive(Debug)]
pub enum CommonBindGroupIndex {
    Light = INDEX_LIGHT_BIND_GROUP,
    Camera = INDEX_CAMERA_BIND_GROUP,
    Environment = INDEX_ENVIRONMENT_BIND_GROUP,
}

impl CommonBindGroupIndex {
    pub fn as_index(&self) -> usize {
        match self {
            CommonBindGroupIndex::Light => INDEX_LIGHT_BIND_GROUP.try_into().unwrap(),
            CommonBindGroupIndex::Camera => INDEX_CAMERA_BIND_GROUP.try_into().unwrap(),
            CommonBindGroupIndex::Environment => INDEX_ENVIRONMENT_BIND_GROUP.try_into().unwrap(),
        }
    }
}
#[derive(Debug)]
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
