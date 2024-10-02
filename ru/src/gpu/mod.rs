use crate::{
    config::gpu::GpuConfig,
    utilities::{adapter_request_device, instance_request_adapter},
    AppError,
};

use pollster::FutureExt;
use std::sync::{Arc, RwLock};
use wgpu::{self};

pub struct GPUGlobal {
    _device: Arc<RwLock<wgpu::Device>>,
    _queue: Arc<RwLock<wgpu::Queue>>,
    _instance: Arc<RwLock<wgpu::Instance>>,
    _adapter: Arc<RwLock<wgpu::Adapter>>,
}
impl GPUGlobal {
    pub async fn initialize(desc: Option<wgpu::InstanceDescriptor>) -> Result<Self, AppError> {
        let instance = wgpu::Instance::new(desc.unwrap_or_else(|| wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        }));

        let adapter = instance_request_adapter(
            &instance,
            None,
            wgpu::PowerPreference::HighPerformance,
            false,
        )
        .block_on();
        let (device, queue) = adapter_request_device(
            &adapter,
            &GpuConfig::default(),
            wgpu::MemoryHints::Performance,
        )
        .block_on();

        Ok(GPUGlobal {
            _device: Arc::new(device.into()),
            _queue: Arc::new(queue.into()),
            _instance: Arc::new(RwLock::new(instance)),
            _adapter: Arc::new(RwLock::new(adapter)),
        })
    }

    pub fn device(&self) -> Arc<RwLock<wgpu::Device>> {
        Arc::clone(&self._device)
    }

    pub fn queue(&self) -> Arc<RwLock<wgpu::Queue>> {
        Arc::clone(&self._queue)
    }

    pub fn instance(&self) -> Arc<RwLock<wgpu::Instance>> {
        Arc::clone(&self._instance)
    }

    pub fn adapter(&self) -> Arc<RwLock<wgpu::Adapter>> {
        Arc::clone(&self._adapter)
    }
}
