use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use bytemuck::{Pod, Zeroable};
use nalgebra::clamp;
use serde::{Deserialize, Serialize};

use super::lerp;

pub trait GetValue {
    fn get(&self) -> u32;
}

#[repr(C)]
#[derive(Clone, Pod, Copy, Zeroable, Debug, Serialize, Deserialize)]
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

#[repr(C)]
#[derive(Clone, Pod, Copy, Zeroable, Debug, Serialize, Deserialize)]
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

#[repr(C)]
#[derive(Clone, Pod, Copy, Zeroable, Debug, Serialize, Deserialize)]
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

#[repr(C)]
#[derive(Clone, Pod, Copy, Zeroable, Debug, Serialize, Default, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
