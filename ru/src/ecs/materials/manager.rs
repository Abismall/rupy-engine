use std::collections::HashMap;

use crate::ecs::model::Material;

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

    pub fn get_material(&self, id: u64) -> std::option::Option<&Material> {
        self.materials.get(&id)
    }

    pub fn add_material(&mut self, id: u64, material: Material) -> Option<Material> {
        match self.materials.insert(id, material) {
            Some(material) => Some(material),
            None => None,
        }
    }

    pub fn set_material_texture(&mut self, id: u64, texture_id: u64) -> Result<(), String> {
        if let Some(material) = self.materials.get_mut(&id) {
            material.texture_id = Some(texture_id);
            Ok(())
        } else {
            Err(format!("Material '{}' not found.", id))
        }
    }
}
