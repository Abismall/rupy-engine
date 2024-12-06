use wgpu::{BindGroup, Device};

pub fn create_light_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    let light_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });
    light_bind_group_layout
}

pub fn create_light_bind_group(device: &Device, light_buffer: &wgpu::Buffer) -> BindGroup {
    let light_bg_layout = create_light_bind_group_layout(device);
    let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &light_bg_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: light_buffer.as_entire_binding(),
        }],
        label: None,
    });
    light_bind_group
}
