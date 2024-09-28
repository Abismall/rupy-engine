use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

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

// Implement From<u32> for Width
impl From<u32> for Width {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// Implement From<Width> for u32
impl From<Width> for u32 {
    fn from(width: Width) -> u32 {
        width.0
    }
}

// Implement From<Width> for f32
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

// Implement From<u32> for Height
impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// Implement From<Height> for u32
impl From<Height> for u32 {
    fn from(height: Height) -> u32 {
        height.0
    }
}

// Implement From<Height> for f32
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

// Implement From<u32> for Depth
impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// Implement From<Depth> for u32
impl From<Depth> for u32 {
    fn from(depth: Depth) -> u32 {
        depth.0
    }
}

// Implement From<Depth> for f32
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
            width: Width::new(width),
            height: Height::new(height),
        }
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width.set(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height.set(height);
    }
}

impl fmt::Display for Size2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size2D (Width: {}, Height: {})", self.width, self.height)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Size3D {
    pub size_2d: Size2D,
    pub depth: Depth,
}

impl Size3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            size_2d: Size2D::new(width, height),
            depth: Depth::new(depth),
        }
    }

    pub fn width(&self) -> Width {
        self.size_2d.width()
    }

    pub fn height(&self) -> Height {
        self.size_2d.height()
    }

    pub fn depth(&self) -> Depth {
        self.depth
    }

    pub fn set_width(&mut self, width: u32) {
        self.size_2d.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.size_2d.set_height(height);
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth.set(depth);
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
