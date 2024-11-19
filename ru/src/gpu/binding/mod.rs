pub mod groups;
pub mod layouts;

pub trait HasBindGroup {
    fn bind_group_layout(&self) -> wgpu::BindGroupLayout;
    fn bind_group(&self) -> wgpu::BindGroup;
}
