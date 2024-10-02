use crate::{math::Vec3, object::object::ObjectDescription};

pub mod scene;
pub struct SceneDescription {
    pub objects: Vec<ObjectDescription>, // List of object descriptions
    pub camera_position: Vec3,           // Initial camera position
}
