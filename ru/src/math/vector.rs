use super::Mat4;
use quaternion::rotate_vector as q_vector_rotate;
use quaternion::Quaternion;
use vecmath::{
    col_mat4_mul, mat4_id, vec2_add, vec2_dot, vec2_normalized, vec2_scale, vec2_sub, vec3_add,
    vec3_cross, vec3_dot, vec3_normalized, vec3_scale, vec3_sub, vec4_add, vec4_dot, vec4_mul,
    vec4_normalized, vec4_scale, vec4_sub, Vector2, Vector3, Vector4,
};

/// Type alias for a 2D vector.
pub type Vec2 = Vector2<f32>;

/// Type alias for a 3D vector.
pub type Vec3 = Vector3<f32>;

/// Type alias for a 4D vector.
pub type Vec4 = Vector4<f32>;
pub type Quatf32 = Quaternion<f32>;

pub fn rotate_vector(q: Quatf32, v: Vec3) -> Vec3 {
    q_vector_rotate(q, v)
}

pub fn col_mat4_multiply(a: Mat4, b: Mat4) -> Mat4 {
    col_mat4_mul(a, b)
}
pub fn vec4_multiply(a: Vec4, b: Vec4) -> Vec4 {
    vec4_mul(a, b)
}
/// Adds two 2D vectors.
pub fn add_vec2(a: Vec2, b: Vec2) -> Vec2 {
    vec2_add(a, b)
}

/// Subtracts the second 2D vector from the first.
pub fn subtract_vec2(a: Vec2, b: Vec2) -> Vec2 {
    vec2_sub(a, b)
}

/// Scales a 2D vector by a scalar.
pub fn scale_vec2(v: Vec2, scalar: f32) -> Vec2 {
    vec2_scale(v, scalar)
}

/// Computes the dot product of two 2D vectors.
pub fn dot_vec2(a: Vec2, b: Vec2) -> f32 {
    vec2_dot(a, b)
}

/// Normalizes a 2D vector.
pub fn normalize_vec2(v: Vec2) -> Vec2 {
    vec2_normalized(v)
}

/// Adds two 3D vectors.
pub fn add_vec3(a: Vec3, b: Vec3) -> Vec3 {
    vec3_add(a, b)
}

/// Subtracts the second 3D vector from the first.
pub fn subtract_vec3(a: Vec3, b: Vec3) -> Vec3 {
    vec3_sub(a, b)
}

/// Scales a 3D vector by a scalar.
pub fn scale_vec3(v: Vec3, scalar: f32) -> Vec3 {
    vec3_scale(v, scalar)
}

/// Computes the cross product of two 3D vectors.
pub fn cross_vec3(a: Vec3, b: Vec3) -> Vec3 {
    vec3_cross(a, b)
}

/// Computes the dot product of two 3D vectors.
pub fn dot_vec3(a: Vec3, b: Vec3) -> f32 {
    vec3_dot(a, b)
}

/// Normalizes a 3D vector.
pub fn normalize_vec3(v: Vec3) -> Vec3 {
    vec3_normalized(v)
}

/// Adds two 4D vectors.
pub fn add_vec4(a: Vec4, b: Vec4) -> Vec4 {
    vec4_add(a, b)
}

/// Subtracts the second 4D vector from the first.
pub fn subtract_vec4(a: Vec4, b: Vec4) -> Vec4 {
    vec4_sub(a, b)
}

/// Scales a 4D vector by a scalar.
pub fn scale_vec4(v: Vec4, scalar: f32) -> Vec4 {
    vec4_scale(v, scalar)
}

/// Computes the dot product of two 4D vectors.
pub fn dot_vec4(a: Vec4, b: Vec4) -> f32 {
    vec4_dot(a, b)
}

/// Normalizes a 4D vector.
pub fn normalize_vec4(v: Vec4) -> Vec4 {
    vec4_normalized(v)
}

pub fn vec3_to_mat4_translation(translation: Vec3) -> Mat4 {
    let mut mat = mat4_id();
    mat[0][3] = translation[0];
    mat[1][3] = translation[1];
    mat[2][3] = translation[2];
    mat
}
