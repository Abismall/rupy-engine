use crate::prelude::constant::Paddings;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub model: [[f32; 4]; 4],     // 64 bytes
    pub view_proj: [[f32; 4]; 4], // 64 bytes
    pub color: [f32; 4],          // 16 bytes
    pub light_color: [f32; 4],    // 16 bytes
    pub light_position: [f32; 4], // 16 bytes
    pub view_position: [f32; 3],  // 12 bytes
    pub _padding1: f32,           // 4 bytes (for alignment)
    pub ambient_strength: f32,    // 4 bytes
    pub diffuse_strength: f32,    // 4 bytes
    pub specular_strength: f32,   // 4 bytes
    pub shininess: f32,           // 4 bytes
    pub _padding2: [f32; 3],      // 12 bytes (for alignment to 16 bytes)
}

impl Uniforms {
    pub fn new(
        view_projection: [[f32; 4]; 4],
        model: [[f32; 4]; 4],
        color: [f32; 4],
        light_position: [f32; 4],
        light_color: [f32; 4],
        view_position: [f32; 3],
        ambient_strength: f32,
        diffuse_strength: f32,
        specular_strength: f32,
        shininess: f32,
    ) -> Self {
        Self {
            view_proj: view_projection,
            model,
            color,
            light_position,
            light_color,
            view_position,
            ambient_strength,
            diffuse_strength,
            specular_strength,
            shininess,
            _padding1: Paddings::PAD_4,
            _padding2: Paddings::PAD_12,
        }
    }
}
pub struct UniformDataStorage<T> {
    data: Vec<T>,
}

impl<T> UniformDataStorage<T> {
    pub fn new(size: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; size],
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            panic!(
                "Index out of bounds: {} for size {}",
                index,
                self.data.len()
            );
        }
    }

    pub fn get(&self, index: usize) -> &T {
        &self.data[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}
