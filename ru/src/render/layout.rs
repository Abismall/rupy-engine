use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, Device, PipelineLayoutDescriptor, PushConstantRange,
};

pub fn bind_group_layout(
    label: Option<&str>,
    device: &Device,
    entries: &[BindGroupLayoutEntry],
) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label, entries })
}

pub fn pipeline_layout(
    label: Option<&str>,
    push_constant_ranges: &[PushConstantRange],
    bind_group_layouts: &[&BindGroupLayout],
    device: &Device,
) {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    });
}
