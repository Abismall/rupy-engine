pub mod cache;
pub mod key;

// Debug
pub const LABEL_SHADED_SIMPLE_PIPELINE: &str = "Shaded Simple RenderPipeline";
pub const LABEL_SHADED_SIMPLE_PIPELINE_LAYOUT: &str = "Shaded Simple RenderPipeline Layout";

pub const LABEL_SHADED_COMPLEX_PIPELINE: &str = "Shaded Complex RenderPipeline";
pub const LABEL_SHADED_COMPLEX_PIPELINE_LAYOUT: &str = "Shaded Complex RenderPipeline Layout";

pub trait RenderState {
    fn topology(&self) -> wgpu::PrimitiveTopology;
    fn blend_state(&self) -> Option<wgpu::BlendState>;
    fn front_face(&self) -> wgpu::FrontFace;
    fn cull_mode(&self) -> Option<wgpu::Face>;
    fn polygon_mode(&self) -> wgpu::PolygonMode;
    fn depth_stencil_state(&self) -> Option<wgpu::DepthStencilState>;
    fn primitive_state(&self) -> wgpu::PrimitiveState;
    fn write_mask(&self) -> wgpu::ColorWrites;
    fn color_target_state(&self, swap_chain: wgpu::TextureFormat) -> wgpu::ColorTargetState;
}
