pub(crate) use nalgebra::*;
pub(crate) const PI: f64 = std::f64::consts::PI;
pub(crate) const FRAC_PI_2: f64 = std::f64::consts::FRAC_2_PI;
pub(crate) fn create_translation_matrix(position: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::new_translation(&position)
}
