#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8, a: u8) {
        *self = Self::new(r, g, b, a);
    }

    pub fn invert(&mut self) {
        self.r = u8::MAX - self.r;
        self.g = u8::MAX - self.g;
        self.b = u8::MAX - self.b;
    }
}

impl From<RgbaColor> for image::Rgba<u8> {
    fn from(val: RgbaColor) -> Self {
        image::Rgba([val.r, val.g, val.b, val.a])
    }
}

impl From<RgbaColor> for [u8; 4] {
    fn from(val: RgbaColor) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
        *self = Self::new(r, g, b, a);
    }

    pub fn invert(&mut self) {
        self.r = 1.0 - self.r;
        self.g = 1.0 - self.g;
        self.b = 1.0 - self.b;
    }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<Color> for RgbaColor {
    fn from(color: Color) -> Self {
        RgbaColor {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
            a: (color.a * 255.0) as u8,
        }
    }
}

impl From<RgbaColor> for Color {
    fn from(color: RgbaColor) -> Self {
        Color {
            r: color.r as f32 / 255.0,
            g: color.g as f32 / 255.0,
            b: color.b as f32 / 255.0,
            a: color.a as f32 / 255.0,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        wgpu::Color {
            r: val.r as f64,
            g: val.g as f64,
            b: val.b as f64,
            a: val.a as f64,
        }
    }
}
