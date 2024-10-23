use std::{collections::HashMap, sync::Arc, time::SystemTime};
use wgpu::ShaderModule;

use crate::{core::error::AppError, graphics::gpu::get_device};

use super::create_shader_module_from_path;

impl Default for ShaderCache {
    fn default() -> Self {
        Self {
            shaders: Default::default(),
        }
    }
}

pub struct ShaderInfo {
    pub module: Arc<ShaderModule>,
    pub path: String,
    pub entry_points: (String, String),
    pub shader_type: Option<String>,
    pub last_updated: SystemTime,
}

pub struct ShaderCache {
    pub shaders: HashMap<String, ShaderInfo>,
}

impl ShaderCache {
    pub fn load_shader(&mut self, path: &str) -> Result<Arc<ShaderModule>, AppError> {
        if let Some(shader_info) = self.shaders.get(path) {
            return Ok(shader_info.module.clone());
        }
        let device = &get_device()?;
        let shader_module = Arc::new(create_shader_module_from_path(device, path)?);

        let shader_source = std::fs::read_to_string(path)?;
        let entry_points = self.detect_entry_points(&shader_source);
        let shader_type = self.detect_shader_type(&shader_source, path);

        self.add_shader(
            shader_module.clone(),
            path.to_string(),
            entry_points,
            shader_type,
        );

        Ok(shader_module)
    }

    pub fn add_shader(
        &mut self,
        module: Arc<ShaderModule>,
        shader_path: String,
        entry_points: (String, String),
        shader_type: Option<String>,
    ) {
        let shader_info = ShaderInfo {
            module,
            path: shader_path.clone(),
            entry_points,
            shader_type,
            last_updated: SystemTime::now(),
        };
        self.shaders.insert(shader_path, shader_info);
    }

    fn detect_entry_points(&self, shader_source: &str) -> (String, String) {
        let mut vertex_entry = None;
        let mut fragment_entry = None;

        for line in shader_source.lines() {
            if vertex_entry.is_none() && line.contains("@vertex") {
                vertex_entry = Some("main_vertex".to_string());
            }
            if fragment_entry.is_none() && line.contains("@fragment") {
                fragment_entry = Some("main_fragment".to_string());
            }

            if vertex_entry.is_some() && fragment_entry.is_some() {
                break;
            }
        }

        (
            vertex_entry.unwrap_or_else(|| "main".to_string()),
            fragment_entry.unwrap_or_else(|| "main".to_string()),
        )
    }
    fn detect_shader_type(&self, shader_source: &str, path: &str) -> Option<String> {
        let binding = std::path::PathBuf::from(path);
        let extension = binding.extension()?.to_str()?;
        match extension {
            "vert" => Some("vertex".to_string()),
            "frag" => Some("fragment".to_string()),
            "wgsl" => {
                if shader_source.contains("@vertex") {
                    Some("vertex".to_string())
                } else if shader_source.contains("@fragment") {
                    Some("fragment".to_string())
                } else {
                    Some("unknown".to_string())
                }
            }
            _ => None,
        }
    }
}
