use crate::prelude::constant::Paddings;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding: u32,
    pub color: [f32; 3],
    pub _padding2: u32,
}

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        LightUniform {
            position,
            _padding: Paddings::PADDING,
            color,
            _padding2: Paddings::PADDING,
        }
    }
}
impl Default for LightUniform {
    fn default() -> Self {
        Self {
            position: [1.0, 1.0, 1.0],
            _padding: Paddings::PADDING,
            color: [1.0, 1.0, 1.0],
            _padding2: Paddings::PADDING,
        }
    }
}
