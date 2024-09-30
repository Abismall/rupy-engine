// math/trigonometry.rs

use std::f32::consts::PI;

/// Converts degrees to radians.
///
/// # Arguments
///
/// * `degrees` - Angle in degrees.
///
/// # Returns
///
/// Angle in radians.
pub fn radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

/// Converts radians to degrees.
///
/// # Arguments
///
/// * `radians` - Angle in radians.
///
/// # Returns
///
/// Angle in degrees.
pub fn degrees(radians: f32) -> f32 {
    radians * (180.0 / PI)
}

/// Computes the sine of an angle in radians.
///
/// # Arguments
///
/// * `angle_radians` - Angle in radians.
///
/// # Returns
///
/// Sine of the angle.
pub fn sin(angle_radians: f32) -> f32 {
    angle_radians.sin()
}

/// Computes the cosine of an angle in radians.
///
/// # Arguments
///
/// * `angle_radians` - Angle in radians.
///
/// # Returns
///
/// Cosine of the angle.
pub fn cos(angle_radians: f32) -> f32 {
    angle_radians.cos()
}
