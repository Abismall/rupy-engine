use wgpu::{ShaderModuleDescriptor, ShaderSource};

pub fn create_shader_modules(device: &wgpu::Device) -> (wgpu::ShaderModule, wgpu::ShaderModule) {
    let vertex_shader_source = include_str!("../../static/shader/vertex.wgsl");
    let vertex_shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: ShaderSource::Wgsl(vertex_shader_source.into()),
    });

    let fragment_shader_source = include_str!("../../static/shader/fragment.wgsl");
    let fragment_shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: ShaderSource::Wgsl(fragment_shader_source.into()),
    });

    (vertex_shader_module, fragment_shader_module)
}
pub fn create_outline_shader_modules(
    device: &wgpu::Device,
) -> (wgpu::ShaderModule, wgpu::ShaderModule) {
    let vertex_shader_source = include_str!("../../static/shader/outline.vertex.wgsl");
    let vertex_shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: ShaderSource::Wgsl(vertex_shader_source.into()),
    });

    let fragment_shader_source = include_str!("../../static/shader/outline.fragment.wgsl");
    let fragment_shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: ShaderSource::Wgsl(fragment_shader_source.into()),
    });

    (vertex_shader_module, fragment_shader_module)
}
