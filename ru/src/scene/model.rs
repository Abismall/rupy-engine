use serde::{Deserialize, Serialize};

use crate::ecs::components::model::Material;

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneData {
    pub name: String,
    pub entities: Vec<EntityData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityData {
    pub id: String,
    pub components: ComponentsData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentsData {
    pub transform: Option<TransformData>,
    pub material: Option<Material>,
    pub mesh: Option<MeshData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformData {
    pub position: [f32; 3],
    pub rotation: [[f32; 4]; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeshData {
    pub geometry_type: String,
}
