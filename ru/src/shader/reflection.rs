use naga::{ResourceBinding, TypeInner};
use wgpu::ShaderStages;

use crate::log_debug;

use super::{ShaderBinding, ShaderResourceType};

pub struct ShaderReflection {
    bindings: Vec<ShaderBinding>,
}
impl ShaderReflection {
    pub fn create_bind_group_layout(
        &self,
        device: &wgpu::Device,
        label: &str,
    ) -> wgpu::BindGroupLayout {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = self
            .bindings
            .iter()
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding: binding.binding,
                visibility: binding.visibility,
                ty: match &binding.resource_type {
                    ShaderResourceType::UniformBuffer {
                        has_dynamic_offset,
                        min_binding_size,
                    } => wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: *has_dynamic_offset,
                        min_binding_size: *min_binding_size,
                    },
                    ShaderResourceType::Texture {
                        multisampled,
                        view_dimension,
                        sample_type,
                    } => wgpu::BindingType::Texture {
                        multisampled: *multisampled,
                        view_dimension: *view_dimension,
                        sample_type: *sample_type,
                    },
                    ShaderResourceType::Sampler { binding_type } => {
                        wgpu::BindingType::Sampler(*binding_type)
                    }
                    ShaderResourceType::StorageBuffer {
                        read_only,
                        has_dynamic_offset,
                        min_binding_size,
                    } => wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: *read_only,
                        },
                        has_dynamic_offset: *has_dynamic_offset,
                        min_binding_size: *min_binding_size,
                    },
                },
                count: None,
            })
            .collect();

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &entries,
        })
    }
}

// Bind Group

impl ShaderReflection {
    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        label: &str,
        resources: &[wgpu::BindingResource],
    ) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = self
            .bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| wgpu::BindGroupEntry {
                binding: binding.binding,
                resource: resources[i].clone(),
            })
            .collect();
        log_debug!("BIND GROUP ENTRIES: {:?}", entries);
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &entries,
            label: Some(label),
        })
    }
}
pub fn naga_reflect_shader_module_bindings(
    module: naga::Module,
    mut bindings: Vec<ShaderBinding>,
) -> Vec<ShaderBinding> {
    for (_, global) in module.global_variables.iter() {
        if let Some(ResourceBinding { binding, .. }) = &global.binding {
            let shader_stage = ShaderStages::VERTEX | ShaderStages::FRAGMENT;

            let resource_type = match &module.types[global.ty].inner {
                TypeInner::Struct { .. } => ShaderResourceType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                TypeInner::Image { .. } => ShaderResourceType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    multisampled: false,
                },
                TypeInner::Sampler { .. } => ShaderResourceType::Sampler {
                    binding_type: wgpu::SamplerBindingType::Filtering,
                },
                TypeInner::Pointer { base, .. } => {
                    let base_type = &module.types[*base];
                    if let TypeInner::Struct { .. } = &base_type.inner {
                        ShaderResourceType::UniformBuffer {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        }
                    } else {
                        ShaderResourceType::StorageBuffer {
                            read_only: false,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        }
                    }
                }
                _ => continue,
            };

            bindings.push(ShaderBinding {
                binding: *binding,
                visibility: shader_stage,
                resource_type,
            });
        }
    }

    bindings
}
