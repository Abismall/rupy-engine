use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Material {
    pub color: [f32; 4],
    pub texture_id: Option<u64>,
}

impl Material {
    pub fn colored(rgba: [f32; 4]) -> Material {
        Material {
            color: rgba,
            texture_id: None,
        }
    }

    pub fn textured(texture_id: u64) -> Material {
        Material {
            color: [0.0, 0.0, 0.0, 0.0],
            texture_id: Some(texture_id),
        }
    }
}
