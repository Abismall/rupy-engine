use super::model::Uniforms;

impl Uniforms {
    pub fn new(
        view_projection: [[f32; 4]; 4],
        model: [[f32; 4]; 4],
        color: [f32; 4],
        view_position: [f32; 4],
        light_position: [f32; 4],
    ) -> Self {
        Self {
            view_proj: view_projection,
            model,
            color,
            light_position,
            light_color: [1.0, 1.0, 1.0, 1.0],
            view_position,
            ambient_strength: 0.8,
            diffuse_strength: 12.0,
            specular_strength: 12.0,
            shininess: 32.0,
        }
    }
}
