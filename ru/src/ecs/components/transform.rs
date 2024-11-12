use nalgebra::{Matrix4, Translation3, Vector3};

use super::model::Transform;

impl Transform {
    pub fn to_model_matrix(&self) -> Matrix4<f32> {
        let translation = Translation3::new(self.position[0], self.position[1], self.position[2])
            .to_homogeneous();

        let rotation = Matrix4::from(self.rotation);

        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(
            self.scale[0],
            self.scale[1],
            self.scale[2],
        ));

        translation * rotation * scale
    }
}
