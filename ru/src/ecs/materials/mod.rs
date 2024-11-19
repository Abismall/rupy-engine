use super::model::Material;

pub mod manager;

impl Material {
    pub fn new(
        texture_id: u64,
        color: [f32; 4],
        shininess: Option<f32>,
        ambient_strength: Option<f32>,
        diffuse_strength: Option<f32>,
        specular_strength: Option<f32>,
    ) -> Material {
        Material {
            texture_id: Some(texture_id),
            color,
            shininess,
            ambient_strength,
            diffuse_strength,
            specular_strength,
        }
    }
}
