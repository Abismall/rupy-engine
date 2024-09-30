use std::collections::HashMap;
use std::sync::Arc;
use wgpu::Buffer;

use crate::log_debug;
#[derive(Debug)]
pub struct BufferManager {
    buffers: HashMap<u64, Arc<Buffer>>,
}

impl BufferManager {
    pub fn new() -> Self {
        let instance = BufferManager {
            buffers: HashMap::new(),
        };
        log_debug!("{:?}", instance);
        instance
    }

    pub fn create_or_get_buffer(
        &mut self,
        key: u64,
        device: &wgpu::Device,
        buffer_size: wgpu::BufferAddress,
    ) -> Arc<Buffer> {
        if let Some(buffer) = self.buffers.get(&key) {
            return Arc::clone(buffer);
        }

        let buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));

        self.buffers.insert(key, Arc::clone(&buffer));
        buffer
    }

    pub fn get_buffer(&self, key: u64) -> Option<Arc<Buffer>> {
        self.buffers.get(&key).cloned()
    }
}
