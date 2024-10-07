pub mod reflection;
pub mod shader;
pub mod source;
#[derive(Debug, Clone)]
pub enum ShaderResourceType {
    UniformBuffer {
        has_dynamic_offset: bool,
        min_binding_size: Option<wgpu::BufferSize>,
    },
    StorageBuffer {
        read_only: bool,
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u64>>,
    },
    Texture {
        multisampled: bool,
        view_dimension: wgpu::TextureViewDimension,
        sample_type: wgpu::TextureSampleType,
    },
    Sampler {
        binding_type: wgpu::SamplerBindingType,
    },
}

pub struct ShaderSourceFile;
impl ShaderSourceFile {
    pub fn load_source(device: &Device, source: &str) -> Result<String, AppError> {
        let path = Self::construct_shader_path(source);
        log_debug!("Attempting to load shader source from path: {:?}", path);

        // Check if the file exists before attempting to read
        if !std::path::Path::new(source).exists() {
            return Err(AppError::ShaderSourceFileError(format!(
                "Shader file not found at path: {}",
                source
            )));
        }

        // Attempt to read the shader source
        let shader_source = fs::read_to_string(path).map_err(|err| {
            AppError::ShaderSourceFileError(format!("Failed to load shader source: {}", err))
        })?;

        log_debug!("Shader source successfully read from path: {}", source);

        // Ok(device.create_shader_module(ShaderModuleDescriptor {
        //     label: Some(source),
        //     source: ShaderSource::Wgsl(shader_source.into()),
        // }))
        Ok(shader_source)
    }
    fn construct_shader_path(file_name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Starting from the project directory
                                                                  // Convert the PathBuf to a string for logging
        log_debug!("Full shader path: {:?}", path.to_str());
        path.push("static");
        path.push("shaders");
        path.push(file_name);
        path
    }

    pub fn get_vertex_shader(device: &Device, path: &str) -> Result<String, AppError> {
        log_debug!("searching for path: {:?}", path);
        let src = Self::load_source(device, path)?;
        Ok(src)
    }

    pub fn get_fragment_shader(device: &Device, path: &str) -> Result<String, AppError> {
        log_debug!("searching for path: {:?}", path);
        let src = Self::load_source(device, path)?;
        Ok(src)
    }
}

use naga::front::wgsl::parse_str;
use reflection::naga_reflect_shader_module_bindings;
use std::{collections::HashMap, fs, num::NonZero, path::PathBuf, sync::Arc};
use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, BufferBindingType, Device, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::{log_debug, prelude::AppError};

#[derive(Debug, Clone)]
pub struct ShaderBinding {
    pub binding: u32,
    pub visibility: ShaderStages,
    pub resource_type: ShaderResourceType,
}
type ShaderBindings = Vec<ShaderBinding>;

#[derive(Debug)]
pub struct Shader {
    pub name: String,
    pub vertex_shader: ShaderModule,
    pub fragment_shader: Option<ShaderModule>, // For rendering shaders, can be None for compute shaders
    pub compute_shader: Option<ShaderModule>, // For compute shaders, can be None for rendering shaders
    pub bindings: ShaderBindings,
    pub bind_group_layouts: HashMap<u32, Arc<BindGroupLayout>>, // Cache for bind group layouts
}

impl Shader {
    /// Creates a new shader with reflection data using `naga`.
    pub fn new(
        device: &Device,
        name: &str,
        vs: &str,
        fs: Option<&str>,
        cs: Option<&str>,
    ) -> Result<Self, AppError> {
        let name = name.to_string();
        log_debug!("Creating shader with name: {:?}", name);

        // Read the vertex shader source code as a string
        let vertex_shader_src = ShaderSourceFile::get_vertex_shader(device, vs)?;
        log_debug!("{:?}", vertex_shader_src);

        // Reflect bindings for the vertex shader
        let mut bindings = Shader::reflect_bindings(&vertex_shader_src, Default::default())?;
        log_debug!("Reflected bindings from vertex shader: {:?}", bindings);

        // Read and compile the vertex shader to create a ShaderModule
        let vertex_shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: ShaderSource::Wgsl(vertex_shader_src.into()),
        });
        log_debug!("vertex_shader_module: {:?}", vertex_shader_module);

        // Read, reflect, and compile the fragment shader if provided
        let fragment_shader_module = if let Some(fragment_shader_path) = fs {
            log_debug!("fragment_shader_path {:?}", fragment_shader_path);
            let fragment_shader_src =
                ShaderSourceFile::get_fragment_shader(device, fragment_shader_path)?;
            bindings = Shader::reflect_bindings(&fragment_shader_src, bindings)?;
            log_debug!("Reflected bindings from fragment shader: {:?}", bindings);

            log_debug!("Fragment shader source loaded.");
            Some(device.create_shader_module(ShaderModuleDescriptor {
                label: Some("Fragment Shader"),
                source: ShaderSource::Wgsl(fragment_shader_src.into()),
            }))
        } else {
            None
        };

        // Read, reflect, and compile the compute shader if provided
        let compute_shader_module = if let Some(compute_shader_path) = cs {
            let compute_shader_src =
                ShaderSourceFile::get_fragment_shader(device, compute_shader_path)?;
            log_debug!("Compute shader source loaded.");
            Some(device.create_shader_module(ShaderModuleDescriptor {
                label: Some("Compute Shader"),
                source: ShaderSource::Wgsl(compute_shader_src.into()),
            }))
        } else {
            None
        };

        // Create an empty bind group layout cache
        let mut bind_group_layouts: HashMap<u32, Arc<BindGroupLayout>> = HashMap::new();

        // Iterate over the bindings to create the bind group layouts
        for binding in &bindings {
            let layout_index = binding.binding;

            // If a layout for this index does not exist, create it
            if !bind_group_layouts.contains_key(&layout_index) {
                let bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(&format!("BindGroupLayout {}", layout_index)),
                        entries: &[wgpu::BindGroupLayoutEntry {
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
                                ShaderResourceType::Texture {
                                    view_dimension,
                                    sample_type,
                                    multisampled,
                                } => wgpu::BindingType::Texture {
                                    multisampled: *multisampled,
                                    view_dimension: *view_dimension,
                                    sample_type: *sample_type,
                                },
                                ShaderResourceType::Sampler { binding_type } => {
                                    wgpu::BindingType::Sampler(*binding_type)
                                }
                            },
                            count: None,
                        }],
                    });

                // Store the bind group layout in the HashMap
                bind_group_layouts.insert(layout_index, bind_group_layout.into());
            }
        }

        // Return the constructed shader with the compiled shader modules and populated bind group layouts
        Ok(Shader {
            name,
            vertex_shader: vertex_shader_module,
            fragment_shader: fragment_shader_module,
            compute_shader: compute_shader_module,
            bindings,
            bind_group_layouts,
        })
    }

    /// Use `naga` to reflect shader resources and create bindings.
    fn reflect_bindings(
        wgsl_code: &str,
        bindings: Vec<ShaderBinding>,
    ) -> Result<ShaderBindings, AppError> {
        Ok(naga_reflect_shader_module_bindings(
            parse_str(wgsl_code).map_err(|e| {
                AppError::ShaderSourceFileError(format!("Failed to parse WGSL: {:?}", e))
            })?,
            bindings,
        ))
    }

    /// Creates the bind group layout using the shader's reflection data.
    pub fn create_bind_group_layout(
        &mut self,
        device: &Device,
        label: &str,
    ) -> Arc<BindGroupLayout> {
        let entries: Vec<BindGroupLayoutEntry> = self
            .bindings
            .iter()
            .map(|binding| BindGroupLayoutEntry {
                binding: binding.binding,
                visibility: binding.visibility,
                ty: match &binding.resource_type {
                    ShaderResourceType::UniformBuffer {
                        has_dynamic_offset,
                        min_binding_size,
                    } => wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: *has_dynamic_offset,
                        min_binding_size: *min_binding_size,
                    },
                    ShaderResourceType::StorageBuffer {
                        read_only,
                        has_dynamic_offset,
                        min_binding_size,
                    } => wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: *read_only,
                        },
                        has_dynamic_offset: *has_dynamic_offset,
                        min_binding_size: *min_binding_size,
                    },
                    ShaderResourceType::Texture {
                        view_dimension,
                        sample_type,
                        multisampled,
                    } => wgpu::BindingType::Texture {
                        multisampled: *multisampled,
                        view_dimension: *view_dimension,
                        sample_type: *sample_type,
                    },
                    ShaderResourceType::Sampler { binding_type } => {
                        wgpu::BindingType::Sampler(*binding_type)
                    }
                },
                count: None,
            })
            .collect();

        let bind_group_layout_arc = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &entries,
            },
        ));

        self.bind_group_layouts
            .insert(0, bind_group_layout_arc.clone());
        bind_group_layout_arc
    }

    /// Method to create the bind group based on the bind group layout and GPU resources.
    pub fn create_bind_group(
        &self,
        device: &Device,
        layout_index: u32,
        bindings: &[wgpu::BindGroupEntry],
        label: &str,
    ) -> wgpu::BindGroup {
        let layout = self
            .bind_group_layouts
            .get(&layout_index)
            .expect("Invalid layout index");

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: bindings,
            label: Some(label),
        })
    }
}
