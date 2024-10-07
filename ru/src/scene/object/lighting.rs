use crate::prelude::Vec3;

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,       // Basic intensity control
    pub shadow_enabled: bool, // Placeholder for shadow casting
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, intensity: f32, shadow_enabled: bool) -> Self {
        Self {
            position,
            color,
            intensity,
            shadow_enabled,
        }
    }

    // Placeholder for shadow map generation
    pub fn generate_shadow_map(&self) {
        // Future implementation of shadow mapping logic
    }
}
