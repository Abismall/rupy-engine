use crate::core::cache::CacheId;

use super::material::manager::MaterialManager;

pub mod manager;
pub mod model;

pub fn prepare_model_bind_groups<'a>(
    material_ids: &'a [CacheId],
    model_bind_groups: &[&'a wgpu::BindGroup],
    material_manager: &'a MaterialManager,
) -> Vec<&'a wgpu::BindGroup> {
    let mut bind_groups = material_ids
        .iter()
        .filter_map(|id| {
            crate::ecs::traits::Cache::get(material_manager, id.value())
                .map(|material| &material.bind_group)
        })
        .collect::<Vec<_>>();
    for group in model_bind_groups {
        bind_groups.push(group);
    }
    bind_groups
}
