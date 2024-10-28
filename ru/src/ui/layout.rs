use std::sync::Arc;

use crate::scene::{components::mesh::Mesh, material::MaterialStructure};

#[derive(Debug)]
pub struct UIComponent {
    pub material: Arc<MaterialStructure>,
    pub mesh: Mesh,
}

impl UIComponent {
    pub fn new(material: Arc<MaterialStructure>, mesh: Mesh) -> Self {
        Self { material, mesh }
    }
}

#[derive(Debug)]
pub struct UILayout {
    pub components: Vec<UIComponent>,
    pub window_size: (f32, f32),
}

impl UILayout {
    pub fn new(window_size: (f32, f32)) -> Self {
        Self {
            components: Vec::new(),
            window_size,
        }
    }

    pub fn add_component(&mut self, component: UIComponent) {
        self.components.push(component);
    }
}
