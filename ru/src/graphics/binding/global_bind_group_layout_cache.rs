use crate::graphics::binding::bind_group_layout::{
    create_global_uniform_bind_group_layout, create_model_uniform_bind_group_layout,
    create_texture_bind_group_layout,
};
use crate::log_debug;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub static GLOBAL_BIND_GROUP_LAYOUT_CACHE: Lazy<
    RwLock<HashMap<String, Arc<wgpu::BindGroupLayout>>>,
> = Lazy::new(|| {
    let map = HashMap::new();
    RwLock::new(map)
});

pub fn initialize_bind_group_layout_cache(device: &wgpu::Device) {
    let mut cache = GLOBAL_BIND_GROUP_LAYOUT_CACHE.write().unwrap();

    cache.insert(
        String::from("GlobalUniforms"),
        Arc::new(create_global_uniform_bind_group_layout(device)),
    );

    cache.insert(
        String::from("ModelUniforms"),
        Arc::new(create_model_uniform_bind_group_layout(device)),
    );

    cache.insert(
        String::from("Texture"),
        Arc::new(create_texture_bind_group_layout(device)),
    );
    log_debug!("Global layout cache initialized!");
}

pub fn get_bind_group_layout(label: &str) -> Option<Arc<wgpu::BindGroupLayout>> {
    let cache = GLOBAL_BIND_GROUP_LAYOUT_CACHE.read().unwrap();
    log_debug!("Returning cache for label: {}", label);
    cache.get(label).cloned()
}
