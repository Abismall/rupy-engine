use std::collections::HashMap;

use crate::{
    ecs::components::{model::Material, uniform::UniformColor},
    prelude::helpers::string_to_u64,
};

#[derive(Debug, Default)]
pub struct MaterialManager {
    materials: HashMap<u64, Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        MaterialManager {
            materials: HashMap::new(),
        }
    }

    pub fn create_colored_material(&mut self, name: &str, color: [f32; 4]) -> Result<(), String> {
        self.materials.insert(
            string_to_u64(name),
            Material {
                name: name.to_string(),
                color: UniformColor::from(color),
                texture_id: None,
            },
        );
        Ok(())
    }

    pub fn create_textured_material(&mut self, name: &str, texture_id: u64) -> Result<(), String> {
        self.materials.insert(
            string_to_u64(name),
            Material {
                name: name.to_string(),
                color: UniformColor::default(),
                texture_id: Some(texture_id),
            },
        );
        Ok(())
    }

    pub fn get_material(&self, name: &str) -> Option<Material> {
        self.materials.get(&string_to_u64(name)).cloned()
    }

    pub fn get_material_mut(&mut self, name: &str) -> Option<&mut Material> {
        self.materials.get_mut(&string_to_u64(name))
    }

    pub fn set_material_color(&mut self, name: &str, color: [f32; 4]) -> Result<(), String> {
        if let Some(material) = self.materials.get_mut(&string_to_u64(name)) {
            material.color = UniformColor::from(color);
            Ok(())
        } else {
            Err(format!("Material '{}' not found.", name))
        }
    }

    pub fn set_material_texture(&mut self, name: &str, texture_id: u64) -> Result<(), String> {
        if let Some(material) = self.materials.get_mut(&string_to_u64(name)) {
            material.texture_id = Some(texture_id);
            Ok(())
        } else {
            Err(format!("Material '{}' not found.", name))
        }
    }
}
