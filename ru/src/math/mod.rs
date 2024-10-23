pub mod spatial;
pub mod trigonometry;
pub mod vector;

pub use spatial::mat4_id;
pub use spatial::mat4_mul;
pub use spatial::{translate, Mat4, Point3};
pub use trigonometry::{cos, degrees, radians, sin};
pub use vector::{
    add_vec2, add_vec3, add_vec4, cross_vec3, dot_vec2, dot_vec3, dot_vec4, normalize_vec2,
    normalize_vec3, normalize_vec4, scale_vec2, scale_vec3, scale_vec4, subtract_vec2,
    subtract_vec3, subtract_vec4, Vec2, Vec3, Vec4,
};

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
