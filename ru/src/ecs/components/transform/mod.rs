pub mod manager;

use cgmath::{Matrix4, Quaternion, Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: Quaternion<f32>,
    pub scale: [f32; 3],
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
}
