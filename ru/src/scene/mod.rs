use object::object::ObjectDescription;

use crate::prelude::Vec3;

pub mod object;
pub mod scene;
pub mod texture;
pub struct SceneDescription {
    pub objects: Vec<ObjectDescription>, // List of object descriptions
    pub camera_position: Vec3,           // Initial camera position
}
