use std::{borrow::Cow, fs, path::PathBuf};

use wgpu::{Device, ShaderModule};

const SHADER_FILE_SUFFIX: &str = "wgsl";

pub fn load_shader_module(device: &Device, shader_source: &str, label: &str) -> ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
    })
}

pub struct Library {
    modules: Vec<ShaderModule>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            modules: Vec::new(),
        }
    }

    pub fn add_shader(&mut self, shader_module: ShaderModule) {
        self.modules.push(shader_module);
    }

    pub fn get(&self, index: usize) -> Option<&ShaderModule> {
        self.modules.get(index)
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn load_from_directory(device: &Device, directory: &str) -> Self {
        let mut library = Library::new();

        let path = PathBuf::from(directory);

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == SHADER_FILE_SUFFIX {
                            if let Ok(shader_source) = fs::read_to_string(&path) {
                                let shader_label = path.file_name().unwrap().to_str().unwrap();
                                let shader_module =
                                    load_shader_module(device, &shader_source, shader_label);
                                library.add_shader(shader_module);
                            }
                        }
                    }
                }
            }
        }

        library
    }
}
