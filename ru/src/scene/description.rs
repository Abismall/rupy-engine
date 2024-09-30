use crate::{
    math::{Mat4, Vec3},
    object::object::Object,
};

#[derive(Clone, Debug)]
pub struct SceneDescription {
    pub objects: Vec<Object>,
    pub camera: CameraDescription,
    pub lighting: Option<LightingDescription>,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraDescription {
    pub projection: Mat4,
    pub view: Mat4,
}

#[derive(Clone, Copy, Debug)]
pub struct LightingDescription {
    pub light_position: Vec3,
    pub light_color: [f32; 4],
}
