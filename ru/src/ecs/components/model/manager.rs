use crate::{core::cache::HashCache, core::error::AppError, ecs::components::ResourceContext};

use super::model::{load_model, Model};

pub struct ModelManager {
    pub models: HashCache<Model>,
}
impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: HashCache::new(),
        }
    }
    pub async fn load_model_from_file(
        file_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resources: &mut ResourceContext,
    ) -> Result<Model, AppError> {
        load_model(file_name, device, queue, resources).await
    }
}
