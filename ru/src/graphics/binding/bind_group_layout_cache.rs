use std::{collections::HashMap, sync::Arc};

pub struct BindGroupLayoutCache {
    layouts: HashMap<String, Arc<wgpu::BindGroupLayout>>,
}
impl Default for BindGroupLayoutCache {
    fn default() -> Self {
        Self {
            layouts: Default::default(),
        }
    }
}
impl BindGroupLayoutCache {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        key: &str,
        descriptor: &wgpu::BindGroupLayoutDescriptor,
    ) -> Arc<wgpu::BindGroupLayout> {
        if let Some(layout) = self.layouts.get(key) {
            Arc::clone(layout)
        } else {
            let layout = Arc::new(device.create_bind_group_layout(descriptor));
            self.layouts.insert(key.to_string(), Arc::clone(&layout));
            layout
        }
    }
}
