use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Quaternion, Vector3};

use crate::{
    core::{cache::HashCache, error::AppError},
    ecs::traits::Cache,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: Quaternion<f32>, // Use Quaternion for rotation
    pub scale: [f32; 3],
}

impl Transform {
    pub fn to_model_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(Vector3::new(
            self.position[0],
            self.position[1],
            self.position[2],
        ));

        let rotation = Matrix4::from(self.rotation);

        let scale = Matrix4::from_nonuniform_scale(self.scale[0], self.scale[1], self.scale[2]);

        translation * rotation * scale
    }
}
#[derive(Debug, Clone)]
pub struct TransformManager {
    cache: HashCache<Transform>,
}

impl TransformManager {
    pub fn new() -> Self {
        TransformManager {
            cache: HashCache::new(),
        }
    }

    pub fn insert(&mut self, id: u64, transform: Transform) -> Result<(), AppError> {
        self.cache.put(id, transform)
    }

    pub fn get(&self, id: u64) -> Option<&Transform> {
        self.cache.get(id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Transform> {
        self.cache.get_mut(id)
    }

    pub fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut Transform, AppError>
    where
        F: FnOnce() -> Result<Transform, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    pub fn remove(&mut self, id: u64) {
        self.cache.remove(id);
    }

    pub fn contains(&self, id: u64) -> bool {
        self.cache.contains(id)
    }
}
