use cgmath::{Matrix4, Quaternion, Vector3};

use crate::core::cache::{CacheKey, HasCacheKey};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub const LABEL: &'static str = "component:transform";
}

impl HasCacheKey for Transform {
    fn key(suffixes: Vec<&str>) -> CacheKey {
        let mut base = String::from(Self::LABEL);
        for suffix in suffixes {
            base.push_str(format!(":{}", suffix).as_ref());
        }
        CacheKey::from(&base)
    }
}
impl Transform {
    pub fn to_model_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(Vector3::new(
            self.position[0],
            self.position[1],
            self.position[2],
        ));

        let rotation = Matrix4::from(self.rotation);

        let scale = Matrix4::from_nonuniform_scale(self.scale[0], self.scale[1], self.scale[2]);

        translation * rotation * scale
    }
    pub fn rotate(&mut self, q: cgmath::Quaternion<f32>) {
        self.rotation = q * self.rotation;
    }

    pub fn set_rotation_direction(direction: TransformRotation) -> cgmath::Quaternion<f32> {
        use cgmath::Rotation3;
        match direction {
            TransformRotation::Cw(angle, axis) => Quaternion::from_axis_angle(axis, angle),
            TransformRotation::Ccw(angle, axis) => Quaternion::from_axis_angle(axis, -angle),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransformRotation {
    Cw(cgmath::Rad<f32>, cgmath::Vector3<f32>),
    Ccw(cgmath::Rad<f32>, cgmath::Vector3<f32>),
}

impl TransformRotation {
    pub fn values(self) -> (cgmath::Rad<f32>, cgmath::Vector3<f32>) {
        match self {
            TransformRotation::Cw(rad, vector3) => (rad, vector3),
            TransformRotation::Ccw(rad, vector3) => (rad, vector3),
        }
    }
    pub fn to_quaternion(self) -> cgmath::Quaternion<f32> {
        use cgmath::Rotation3;
        match self {
            TransformRotation::Cw(rad, vector3) => Quaternion::from_axis_angle(vector3, rad),
            TransformRotation::Ccw(rad, vector3) => Quaternion::from_axis_angle(vector3, -rad),
        }
    }
    pub fn random(angle: cgmath::Rad<f32>, axis: cgmath::Vector3<f32>) -> TransformRotation {
        if rand::Rng::gen_bool(&mut rand::thread_rng(), 0.5) {
            TransformRotation::Ccw(angle, axis)
        } else {
            TransformRotation::Cw(angle, axis)
        }
    }
}
