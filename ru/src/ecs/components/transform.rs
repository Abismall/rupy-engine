use crate::ecs::model::Transform;

use nalgebra::{Matrix4, Quaternion, Unit, UnitQuaternion, Vector3};

impl Transform {
    pub fn rotate(&mut self) {
        let axis = Vector3::new(self.position[0], self.position[1], self.position[2]);
        let angle = std::f32::consts::FRAC_PI_4;

        let unit_axis = Unit::new_normalize(axis);

        self.rotation = *UnitQuaternion::from_axis_angle(&unit_axis, angle);
    }
    pub fn update(&mut self, delta_time: f32) {
        self.rotate();
        self.scale(delta_time);
        self.translate(delta_time);
    }
    pub fn translate(&mut self, delta_time: f32) {
        for i in 0..3 {
            self.position[i] += self.velocity[i] * delta_time;
        }
    }
    pub fn scale(&mut self, delta_time: f32) {
        let scale_delta = 1.0 * delta_time;
        for i in 0..3 {
            self.scale[i] += scale_delta;
        }
    }
    pub fn to_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&Vector3::new(
            self.position[0],
            self.position[1],
            self.position[2],
        )) * Matrix4::new_nonuniform_scaling(&Vector3::new(
            self.scale[0],
            self.scale[1],
            self.scale[2],
        )) * UnitQuaternion::from_quaternion(Quaternion::new(
            self.rotation[3], // w
            self.rotation[0], // x
            self.rotation[1], // y
            self.rotation[2], // z
        ))
        .to_homogeneous()
    }
}
