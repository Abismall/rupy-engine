pub const UNIFORM_BINDING_IDX: u32 = 0;
pub const COLOR_BINDING_IDX: u32 = 0;
pub const INSTANCE_DATA_BINDING_IDX: u32 = 0;
pub const TEXTURE_BINDING_IDX: u32 = 1;
pub const SAMPLER_BINDING_IDX: u32 = 0;

pub const WGSL_SHADER_EXT: &str = "wgsl";

pub const WGSL_VERTEX_MAIN_DEFAULT: &str = "vs_main";
pub const WGSL_FRAGMENT_MAIN_DEFAULT: &str = "fs_main";

pub const PERSPECTIVE_FAR: f32 = 100.0;
pub const PERSPECTIVE_NEAR: f32 = 0.1;

pub const ORTHOGRAPHIC_FAR: f32 = 1.0;
pub const ORTHOGRAPHIC_NEAR: f32 = -0.1;

pub const ZERO_F32: f32 = 0.0;

pub struct Paddings;
impl Paddings {
    pub const PADDING: f32 = 0.0;
    pub const PAD_4: f32 = Self::PADDING;
    pub const PAD_8: [f32; 2] = [Self::PADDING, Self::PADDING];
    pub const PAD_12: [f32; 3] = [Self::PADDING, Self::PADDING, Self::PADDING];
    pub const PAD_16: [f32; 4] = [Self::PADDING, Self::PADDING, Self::PADDING, Self::PADDING];
}
