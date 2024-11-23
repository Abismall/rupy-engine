use std::sync::{Arc, RwLock};

use crate::core::error::AppError;
use once_cell::sync::Lazy;
use wgpu::Queue;
use wgpu::{Adapter, Device, Instance, InstanceDescriptor};

pub struct GPU {
    pub device: Arc<Device>,
    pub adapter: Arc<Adapter>,
    pub instance: Arc<Instance>,
}

#[derive(Debug, Clone)]
pub struct GpuQueue {
    pub queue: Arc<Queue>,
}

fn new_instance() -> wgpu::Instance {
    let instance = Instance::new(InstanceDescriptor::default());
    instance
}

async fn request_adapter(instance: &wgpu::Instance) -> Result<wgpu::Adapter, AppError> {
    let power_preference = wgpu::PowerPreference::HighPerformance;
    let compatible_surface = None;
    let force_fallback_adapter = false;
    let options = wgpu::RequestAdapterOptions {
        power_preference,
        compatible_surface,
        force_fallback_adapter,
    };
    match instance.request_adapter(&options).await {
        Some(adapter) => Ok(adapter),
        None => Err(AppError::AdapterRequestError),
    }
}
async fn request_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), AppError> {
    let required_features = wgpu::Features::POLYGON_MODE_LINE
        | wgpu::Features::POLYGON_MODE_POINT
        | wgpu::Features::VERTEX_WRITABLE_STORAGE;
    let required_limits = wgpu::Limits::default();
    let memory_hints = wgpu::MemoryHints::Performance;
    let desc = wgpu::DeviceDescriptor {
        label: Some("Device"),
        required_features,
        required_limits,
        memory_hints,
    };
    let (device, queue) = adapter
        .request_device(&desc, None)
        .await
        .map_err(|e| AppError::RequestDeviceError(e))?;
    Ok((device, queue))
}
pub static CACHED_DEVICE: Lazy<RwLock<Option<Arc<GPU>>>> = Lazy::new(|| RwLock::new(None));

pub static CACHED_QUEUE: Lazy<RwLock<Option<Arc<GpuQueue>>>> = Lazy::new(|| RwLock::new(None));

pub async fn initialize_instance() -> Result<(), AppError> {
    let instance = new_instance();
    let adapter = request_adapter(&instance)
        .await
        .expect("Failed to set adapter to global cache");

    let (device, queue) = request_device(&adapter).await?;
    let mut device_cache = CACHED_DEVICE
        .write()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    *device_cache = Some(Arc::new(GPU {
        device: Arc::new(device),
        adapter: Arc::new(adapter),
        instance: Arc::new(instance),
    }));

    let mut queue_cache = CACHED_QUEUE
        .write()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    *queue_cache = Some(Arc::new(GpuQueue {
        queue: Arc::new(queue),
    }));

    Ok(())
}

pub fn get_device() -> Result<Arc<Device>, AppError> {
    let device_cache = CACHED_DEVICE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    device_cache
        .as_ref()
        .map(|gpu_device| &gpu_device.device)
        .ok_or(AppError::InstanceInitializationError)
        .cloned()
}
pub fn get_adapter() -> Result<Arc<Adapter>, AppError> {
    let device_cache = CACHED_DEVICE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    device_cache
        .as_ref()
        .map(|gpu_device| &gpu_device.adapter)
        .ok_or(AppError::InstanceInitializationError)
        .cloned()
}
pub fn get_instance() -> Result<Arc<Instance>, AppError> {
    let device_cache = CACHED_DEVICE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    device_cache
        .as_ref()
        .map(|gpu_device| &gpu_device.instance)
        .ok_or(AppError::InstanceInitializationError)
        .cloned()
}

pub fn get_queue() -> Result<Arc<Queue>, AppError> {
    let queue_cache = CACHED_QUEUE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    queue_cache
        .as_ref()
        .map(|gpu_queue| &gpu_queue.queue)
        .ok_or(AppError::InstanceInitializationError)
        .cloned()
}
