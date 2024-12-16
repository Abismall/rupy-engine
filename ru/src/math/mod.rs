pub use nalgebra::*;
pub const PI: f64 = std::f64::consts::PI;
pub const FRAC_2_PI: f64 = std::f64::consts::FRAC_2_PI;
pub const FRAC_PI_4: f64 = std::f64::consts::FRAC_PI_4;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
pub fn pixel_to_ndc(x: f32, y: f32, window_width: f32, window_height: f32) -> [f32; 2] {
    let x_ndc = (x / window_width) * 2.0 - 1.0;
    let y_ndc = 1.0 - (y / window_height) * 2.0;
    [x_ndc, y_ndc]
}

pub fn mat4_translation(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
