pub const INDEX_TEXTURE_VIEW: u32 = 0;
pub const INDEX_TEXTURE_SAMPLER: u32 = 1;

pub const WGSL_SHADER_EXT: &str = "wgsl";

pub const WGSL_VS_MAIN: &str = "vs_main";
pub const WGSL_FS_MAIN: &str = "fs_main";
pub const CUR_MONITOR_FULLSCREEN: std::option::Option<winit::window::Fullscreen> =
    Some(winit::window::Fullscreen::Borderless(None));
pub struct Paddings;
impl Paddings {
    pub const PADDING: u32 = 0;
    pub const PAD_4: u32 = Self::PADDING;
    pub const PAD_8: [u32; 2] = [Self::PADDING, Self::PADDING];
    pub const PAD_12: [u32; 3] = [Self::PADDING, Self::PADDING, Self::PADDING];
    pub const PAD_16: [u32; 4] = [Self::PADDING, Self::PADDING, Self::PADDING, Self::PADDING];
}
