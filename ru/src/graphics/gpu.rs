use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use wgpu::Queue;
use wgpu::{Adapter, Device, Instance, InstanceDescriptor};

use crate::core::error::AppError;

pub struct GpuDevice {
    pub device: Arc<Device>,
    pub adapter: Arc<Adapter>,
    pub instance: Arc<Instance>,
}

#[derive(Debug, Clone)]
pub struct GpuQueue {
    pub queue: Arc<Queue>,
}

fn get_default_instance() -> wgpu::Instance {
    let instance = Instance::new(InstanceDescriptor::default());
    instance
}
pub static GLOBAL_GPU_DEVICE_CACHE: Lazy<RwLock<Option<Arc<GpuDevice>>>> =
    Lazy::new(|| RwLock::new(None));

pub static GLOBAL_GPU_QUEUE_CACHE: Lazy<RwLock<Option<Arc<GpuQueue>>>> =
    Lazy::new(|| RwLock::new(None));

pub async fn initialize_gpu_resources_cache() -> Result<(), AppError> {
    let instance = get_default_instance();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(AppError::AdapterRequestError)?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::POLYGON_MODE_LINE
                    | wgpu::Features::POLYGON_MODE_POINT,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await
        .map_err(|e| AppError::DeviceCreationError(e.to_string()))?;
    let mut device_cache = GLOBAL_GPU_DEVICE_CACHE
        .write()
        .map_err(|_| AppError::LockAcquisitionError)?;
    *device_cache = Some(Arc::new(GpuDevice {
        device: Arc::new(device),
        adapter: Arc::new(adapter),
        instance: Arc::new(instance),
    }));

    let mut queue_cache = GLOBAL_GPU_QUEUE_CACHE
        .write()
        .map_err(|_| AppError::LockAcquisitionError)?;
    *queue_cache = Some(Arc::new(GpuQueue {
        queue: Arc::new(queue),
    }));

    Ok(())
}

pub fn get_device() -> Result<Arc<Device>, AppError> {
    let device_cache = GLOBAL_GPU_DEVICE_CACHE
        .read()
        .map_err(|_| AppError::LockAcquisitionError)?;
    device_cache
        .as_ref()
        .map(|gpu_device| Arc::clone(&gpu_device.device))
        .ok_or(AppError::InstanceInitializationError)
}
pub fn get_adapter() -> Result<Arc<Adapter>, AppError> {
    let device_cache = GLOBAL_GPU_DEVICE_CACHE
        .read()
        .map_err(|_| AppError::LockAcquisitionError)?;
    device_cache
        .as_ref()
        .map(|gpu_device| Arc::clone(&gpu_device.adapter))
        .ok_or(AppError::InstanceInitializationError)
}
pub fn get_instance() -> Result<Arc<Instance>, AppError> {
    let device_cache = GLOBAL_GPU_DEVICE_CACHE
        .read()
        .map_err(|_| AppError::LockAcquisitionError)?;
    device_cache
        .as_ref()
        .map(|gpu_device| Arc::clone(&gpu_device.instance))
        .ok_or(AppError::InstanceInitializationError)
}

pub fn get_queue() -> Result<Arc<Queue>, AppError> {
    let queue_cache = GLOBAL_GPU_QUEUE_CACHE
        .read()
        .map_err(|_| AppError::LockAcquisitionError)?;
    queue_cache
        .as_ref()
        .map(|gpu_queue| Arc::clone(&gpu_queue.queue))
        .ok_or(AppError::InstanceInitializationError)
}
