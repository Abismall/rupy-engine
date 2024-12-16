use camera::CameraUniform;
use lighting::LightUniform;

pub mod camera;
pub mod lighting;

pub struct Uniforms {
    pub camera: CameraUniform,
    pub lighting: LightUniform,
}
