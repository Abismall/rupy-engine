// pub struct Mesh {
//     pub name: String,
//     pub vertex_buffer: wgpu::Buffer,
//     pub index_buffer: wgpu::Buffer,
//     pub num_elements: u32,
//     pub material: usize,
// }

use crate::{core::cache::ComponentCacheKey, ecs::components::IntoComponentCacheKey};

#[derive(Debug, Clone, Copy)]
pub struct Mesh {
    pub num_elements: u32,
    pub material: usize,
}

impl IntoComponentCacheKey for Mesh {
    fn into_cache_key(&self) -> ComponentCacheKey {
        let combined_key = ((self.material as u64) << 32) | (self.num_elements as u64);
        ComponentCacheKey::from(combined_key)
    }
}
