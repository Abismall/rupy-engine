use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::vector::col_mat4_multiply;
use super::{clamp, lerp, Vec3};

use vecmath::mat4_id as vecmath_mat4_id;
use vecmath::{col_mat4_mul, Matrix4};

pub type Mat4 = Matrix4<f32>;

pub type Point3 = Vec3;

pub fn mat4_id() -> Mat4 {
    vecmath_mat4_id()
}

pub fn mat4_mul(a: Mat4, b: Mat4) -> Mat4 {
    col_mat4_mul(a, b)
}

/// Converts a 3x3 matrix to a 4x4 matrix, optionally adding a translation vector.
///
/// # Arguments
///
/// * `mat3` - A 3x3 matrix representing rotation and scaling.
/// * `translation` - An optional 3D translation vector.
///
/// # Returns
///
/// A 4x4 transformation matrix.
pub fn mat3_to_mat4(mat3: [[f32; 3]; 3], translation: Option<Vec3>) -> Mat4 {
    let zero = 0.0f32;
    let one = 1.0f32;

    let mut mat4: Mat4 = [
        [zero, zero, zero, zero],
        [zero, zero, zero, zero],
        [zero, zero, zero, zero],
        [zero, zero, zero, one],
    ];

    // Copy the Mat3 elements into the top-left of Mat4
    for i in 0..3 {
        for j in 0..3 {
            mat4[i][j] = mat3[i][j];
        }
    }

    // Set the translation components if provided
    if let Some(t) = translation {
        mat4[0][3] = t[0];
        mat4[1][3] = t[1];
        mat4[2][3] = t[2];
    }

    mat4
}

/// Creates a translation matrix from a 3D vector.
///
/// # Arguments
///
/// * `translation` - A 3D translation vector.
///
/// # Returns
///
/// A 4x4 translation matrix.
pub fn translate(translation: Vec3) -> Mat4 {
    let mut mat = mat4_id();
    mat[0][3] = translation[0];
    mat[1][3] = translation[1];
    mat[2][3] = translation[2];
    mat
}

/// Creates a rotation matrix around the X-axis.
///
/// # Arguments
///
/// * `angle_degrees` - Rotation angle in degrees.
///
/// # Returns
///
/// A 4x4 rotation matrix around the X-axis.
pub fn rotate_x(angle_degrees: f32) -> Mat4 {
    let angle = crate::math::trigonometry::radians(angle_degrees);
    let cos_theta = crate::math::trigonometry::cos(angle);
    let sin_theta = crate::math::trigonometry::sin(angle);

    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_theta, -sin_theta, 0.0],
        [0.0, sin_theta, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Creates a rotation matrix around the Y-axis.
///
/// # Arguments
///
/// * `angle_degrees` - Rotation angle in degrees.
///
/// # Returns
///
/// A 4x4 rotation matrix around the Y-axis.
pub fn rotate_y(angle_degrees: f32) -> Mat4 {
    let angle = crate::math::trigonometry::radians(angle_degrees);
    let cos_theta = crate::math::trigonometry::cos(angle);
    let sin_theta = crate::math::trigonometry::sin(angle);

    [
        [cos_theta, 0.0, sin_theta, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_theta, 0.0, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Creates a rotation matrix around the Z-axis.
///
/// # Arguments
///
/// * `angle_degrees` - Rotation angle in degrees.
///
/// # Returns
///
/// A 4x4 rotation matrix around the Z-axis.
pub fn rotate_z(angle_degrees: f32) -> Mat4 {
    let angle = crate::math::trigonometry::radians(angle_degrees);
    let cos_theta = crate::math::trigonometry::cos(angle);
    let sin_theta = crate::math::trigonometry::sin(angle);

    [
        [cos_theta, -sin_theta, 0.0, 0.0],
        [sin_theta, cos_theta, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
/// Creates a combined rotation matrix from Euler angles around X, Y, and Z axes.
///
/// # Arguments
///
/// * `rotation` - A Vec3 containing rotation angles in degrees around the X, Y, and Z axes.
///
/// # Returns
///
/// A 4x4 combined rotation matrix as a Mat4.
pub fn new_rotation(rotation: Vec3) -> Mat4 {
    let rot_x = rotate_x(rotation[0]);
    let rot_y = rotate_y(rotation[1]);
    let rot_z = rotate_z(rotation[2]);

    // Combine rotations: Y * X * Z
    let rot_xy = col_mat4_multiply(rot_y, rot_x);
    let rotation_matrix = col_mat4_multiply(rot_xy, rot_z);

    rotation_matrix
}

/// Creates a model matrix by combining translation, rotation, and scaling.
///
/// # Arguments
///
/// * `translation` - A 3D translation vector.
/// * `rotation_x` - Rotation angle around the X-axis in degrees.
/// * `rotation_y` - Rotation angle around the Y-axis in degrees.
/// * `rotation_z` - Rotation angle around the Z-axis in degrees.
/// * `scale` - A 3D scaling vector.
///
/// # Returns
///
/// A 4x4 model matrix.
pub fn model_matrix(
    translation: Vec3,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    scale: Vec3,
) -> Mat4 {
    let t = translate(translation);
    let rx = rotate_x(rotation_x);
    let ry = rotate_y(rotation_y);
    let rz = rotate_z(rotation_z);
    let s = scale_matrix(scale);

    // Combine transformations: M = T * Rz * Ry * Rx * S
    let combined = mat4_mul(t, mat4_mul(rz, mat4_mul(ry, mat4_mul(rx, s))));
    combined
}

/// Creates a scaling matrix from a 3D vector.
///
/// # Arguments
///
/// * `scale` - A 3D scaling vector.
///
/// # Returns
///
/// A 4x4 scaling matrix.
pub fn scale_matrix(scale: Vec3) -> Mat4 {
    [
        [scale[0], 0.0, 0.0, 0.0],
        [0.0, scale[1], 0.0, 0.0],
        [0.0, 0.0, scale[2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Creates a non-uniform scaling matrix.
///
/// # Arguments
///
/// * `scale` - Scaling factors along the X, Y, and Z axes.
///
/// # Returns
///
/// A 4x4 non-uniform scaling matrix.
pub fn new_nonuniform_scaling(scale: Vec3) -> Mat4 {
    [
        [scale[0], 0.0, 0.0, 0.0],
        [0.0, scale[1], 0.0, 0.0],
        [0.0, 0.0, scale[2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub trait GetValue {
    fn get(&self) -> u32;
}

#[derive(Clone, Copy, Debug)]
pub struct Width(pub u32);

impl GetValue for Width {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Width {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Width {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Width> for u32 {
    fn from(width: Width) -> u32 {
        width.0
    }
}

impl From<Width> for f32 {
    fn from(width: Width) -> f32 {
        width.0 as f32
    }
}

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Height(pub u32);

impl GetValue for Height {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Height {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Height> for u32 {
    fn from(height: Height) -> u32 {
        height.0
    }
}

impl From<Height> for f32 {
    fn from(height: Height) -> f32 {
        height.0 as f32
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Depth(u32);

impl GetValue for Depth {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Depth {
    fn default() -> Self {
        Self(0)
    }
}

impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Depth> for u32 {
    fn from(depth: Depth) -> u32 {
        depth.0
    }
}

impl From<Depth> for f32 {
    fn from(depth: Depth) -> f32 {
        depth.0 as f32
    }
}

#[derive(Clone, Debug, Default)]
pub struct Size2D {
    pub width: Width,
    pub height: Height,
}

impl Size2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: Width(width),
            height: Height(height),
        }
    }

    pub fn scale(&self, scale_x: f32, scale_y: f32) -> Self {
        Self {
            width: Width((self.width.get() as f32 * scale_x) as u32),
            height: Height((self.height.get() as f32 * scale_y) as u32),
        }
    }

    pub fn clamp(&self, min_width: u32, min_height: u32, max_width: u32, max_height: u32) -> Self {
        Self {
            width: Width(clamp(self.width.get() as f32, min_width as f32, max_width as f32) as u32),
            height: Height(clamp(
                self.height.get() as f32,
                min_height as f32,
                max_height as f32,
            ) as u32),
        }
    }

    pub fn interpolate(&self, target: &Size2D, t: f32) -> Self {
        Self {
            width: Width(lerp(self.width.get() as f32, target.width.get() as f32, t) as u32),
            height: Height(lerp(self.height.get() as f32, target.height.get() as f32, t) as u32),
        }
    }
}

/// 3D size representation with width, height, and depth.
#[derive(Clone, Debug, Default)]
pub struct Size3D {
    pub size_2d: Size2D,
    pub depth: Depth,
}

impl Size3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            size_2d: Size2D::new(width, height),
            depth: Depth(depth),
        }
    }

    pub fn scale(&self, scale_x: f32, scale_y: f32, scale_z: f32) -> Self {
        Self {
            size_2d: self.size_2d.scale(scale_x, scale_y),
            depth: Depth((self.depth.get() as f32 * scale_z) as u32),
        }
    }

    pub fn clamp(
        &self,
        min_width: u32,
        min_height: u32,
        min_depth: u32,
        max_width: u32,
        max_height: u32,
        max_depth: u32,
    ) -> Self {
        Self {
            size_2d: self
                .size_2d
                .clamp(min_width, min_height, max_width, max_height),
            depth: Depth(clamp(self.depth.get() as f32, min_depth as f32, max_depth as f32) as u32),
        }
    }

    pub fn interpolate(&self, target: &Size3D, t: f32) -> Self {
        Self {
            size_2d: self.size_2d.interpolate(&target.size_2d, t),
            depth: Depth(lerp(self.depth.get() as f32, target.depth.get() as f32, t) as u32),
        }
    }
}
impl Width {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl Add for Width {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Width {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for Width {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for Width {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl Rem for Width {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}

impl Height {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl Add for Height {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Height {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for Height {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for Height {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl Rem for Height {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}

impl Depth {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }
}

impl Add for Depth {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Depth {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for Depth {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for Depth {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl Rem for Depth {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}
