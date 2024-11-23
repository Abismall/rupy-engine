use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Vector3};

#[repr(C)]
#[derive(Debug, Clone, Default, Copy, Pod, Zeroable)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [[f32; 4]; 4],
    pub scale: [f32; 3],
}

impl Transform {
    pub fn to_model_matrix(&self) -> Matrix4<f32> {
        let translation =
            nalgebra::Translation3::new(self.position[0], self.position[1], self.position[2])
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
