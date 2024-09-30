use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{Buffer, Sampler, TextureView};

#[derive(Debug)]
pub enum BindGroupLayoutEntryEnum {
    UniformBuffer(Buffer),
    Texture(TextureView),
    Sampler(Sampler),
}

#[derive(Debug)]
pub struct BindGroupLayoutLayers {
    pub layers: Arc<Vec<BindGroupLayoutEntryEnum>>,
}

impl BindGroupLayoutLayers {
    pub fn new(entries: Vec<BindGroupLayoutEntryEnum>) -> Self {
        Self {
            layers: Arc::new(entries),
        }
    }
}

pub struct BindGroupLayoutManager {
    layouts: HashMap<u64, Arc<BindGroupLayoutLayers>>,
}

impl BindGroupLayoutManager {
    pub fn new() -> Self {
        Self {
            layouts: HashMap::new(),
        }
    }

    pub fn get_or_create_layouts(
        &mut self,
        key: u64,
        entries: Vec<BindGroupLayoutEntryEnum>,
    ) -> Arc<BindGroupLayoutLayers> {
        if let Some(resource) = self.layouts.get(&key) {
            return Arc::clone(resource);
        }

        let resource = Arc::new(BindGroupLayoutLayers::new(entries));
        self.layouts.insert(key, Arc::clone(&resource));
        resource
    }

    pub fn get_layouts(&self, key: u64) -> Option<Arc<BindGroupLayoutLayers>> {
        self.layouts.get(&key).cloned()
    }
}
