use super::global::{get_adapter, get_device, get_instance, get_queue};
use bytemuck::Pod;
use std::sync::Arc;
use wgpu::{Adapter, Buffer, BufferAddress, CommandBuffer, Device, Queue};

pub struct GpuResourceCache {
    device: Arc<Device>,
    queue: Arc<Queue>,
    adapter: Arc<Adapter>,
    instance: Arc<wgpu::Instance>,
}

impl GpuResourceCache {
    pub async fn new() -> Self {
        let (adapter, device, queue, instance) = GpuResourceCache::get_cached_context().await;
        Self {
            device,
            queue,
            adapter,
            instance,
        }
    }

    async fn get_cached_context() -> (
        Arc<wgpu::Adapter>,
        Arc<wgpu::Device>,
        Arc<wgpu::Queue>,
        Arc<wgpu::Instance>,
    ) {
        let instance = get_instance().expect("Instance");
        let adapter = get_adapter().expect("Adapter");
        let device = get_device().expect("Device");
        let queue = get_queue().expect("Queue");
        (adapter, device, queue, instance)
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn adapter(&self) -> &Arc<Adapter> {
        &self.adapter
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn instance(&self) -> &Arc<wgpu::Instance> {
        &self.instance
    }

    pub fn submit(&self, command_buffer: CommandBuffer) {
        self.queue.submit(Some(command_buffer));
    }

    pub fn submit_multiple<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.queue.submit(command_buffers);
    }

    pub fn write_to_buffer<T: Pod>(&self, buffer: &Buffer, offset: BufferAddress, data: &[T]) {
        self.queue
            .write_buffer(buffer, offset, bytemuck::cast_slice(data));
    }

    pub fn flush(&self) {
        self.device.poll(wgpu::Maintain::Wait);
    }
}
