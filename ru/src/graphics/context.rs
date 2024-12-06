use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue};

use super::global::{get_adapter, get_device, get_instance, get_queue};

pub struct GpuContext {
    device: Arc<Device>,
    queue: Arc<Queue>,
    adapter: Arc<Adapter>,
    instance: Arc<Instance>,
}
impl GpuContext {
    pub async fn new() -> Self {
        let (adapter, device, queue, instance) = GpuContext::get_cached_context().await;
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
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
    pub fn submit(&self, command_buffer: wgpu::CommandBuffer) {
        self.queue.submit(Some(command_buffer));
    }
}
