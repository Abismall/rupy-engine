use std::sync::{Arc, RwLock};

use crate::core::error::AppError;
use once_cell::sync::Lazy;
use wgpu::{Adapter, Device, Instance, InstanceDescriptor};
use wgpu::{Features, Queue};

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
    Instance::new(InstanceDescriptor::default())
}

async fn request_adapter(instance: &wgpu::Instance) -> Result<wgpu::Adapter, AppError> {
    let options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };
    instance
        .request_adapter(&options)
        .await
        .ok_or(AppError::GPUResourceError(String::from(
            "GPU adapter is not available",
        )))
}

async fn request_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), AppError> {
    let adapter_features = adapter.features();
    let desired_features = Features::all_webgpu_mask();
    let supported_features = adapter_features & desired_features;
    let desc = wgpu::DeviceDescriptor {
        label: Some("Device"),
        required_features: supported_features,
        required_limits: wgpu::Limits::downlevel_defaults(),
        memory_hints: wgpu::MemoryHints::Performance,
    };
    adapter
        .request_device(&desc, None)
        .await
        .map_err(AppError::RequestDeviceError)
}

pub static GPU_INSTANCE: Lazy<Arc<RwLock<Option<GPU>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

pub static GPU_QUEUE: Lazy<Arc<RwLock<Option<GpuQueue>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

pub async fn initialize_instance() -> Result<(), AppError> {
    {
        let gpu_instance = GPU_INSTANCE
            .read()
            .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
        if gpu_instance.is_some() {
            return Ok(());
        }
    }

    let instance = new_instance();
    let adapter = request_adapter(&instance).await?;
    let (device, queue) = request_device(&adapter).await?;

    {
        let mut gpu_instance = GPU_INSTANCE
            .write()
            .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
        *gpu_instance = Some(GPU {
            device: Arc::new(device),
            adapter: Arc::new(adapter),
            instance: Arc::new(instance),
        });
    }

    {
        let mut gpu_queue = GPU_QUEUE
            .write()
            .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
        *gpu_queue = Some(GpuQueue {
            queue: Arc::new(queue),
        });
    }

    Ok(())
}

pub fn get_device() -> Result<Arc<Device>, AppError> {
    let gpu_instance = GPU_INSTANCE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    gpu_instance
        .as_ref()
        .map(|gpu| gpu.device.clone())
        .ok_or(AppError::GPUResourceError(String::from(
            "GPU device is not available",
        )))
}

pub fn get_adapter() -> Result<Arc<Adapter>, AppError> {
    let gpu_instance = GPU_INSTANCE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    gpu_instance
        .as_ref()
        .map(|gpu| gpu.adapter.clone())
        .ok_or(AppError::GPUResourceError(String::from(
            "GPU adapter is not available",
        )))
}

pub fn get_instance() -> Result<Arc<Instance>, AppError> {
    let gpu_instance = GPU_INSTANCE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    gpu_instance
        .as_ref()
        .map(|gpu| gpu.instance.clone())
        .ok_or(AppError::GPUResourceError(String::from(
            "GPU instance is not available",
        )))
}

pub fn get_queue() -> Result<Arc<Queue>, AppError> {
    let gpu_queue = GPU_QUEUE
        .read()
        .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))?;
    gpu_queue
        .as_ref()
        .map(|queue| queue.queue.clone())
        .ok_or(AppError::GPUResourceError(String::from(
            "GPU queue is not available",
        )))
}
