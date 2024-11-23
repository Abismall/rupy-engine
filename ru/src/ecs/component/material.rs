#[derive(Debug, Default, Clone)]
pub struct Material {
    pub color: [f32; 4],
    pub texture_name: Option<String>,
}

impl Material {
    pub fn new(color: [f32; 4], texture_name: Option<String>) -> Self {
        Self {
            color,
            texture_name,
        }
    }
}
