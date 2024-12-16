use crate::{
    core::cache::{CacheKey, HasCacheKey},
    ecs::components::transform::Transform,
    graphics::vertex::Vertex,
};

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    pub transform: Transform,
}
impl Instance {
    const LABEL: &'static str = "component:instance";
}
impl HasCacheKey for Instance {
    fn key(suffixes: Vec<&str>) -> CacheKey {
        let mut base = String::from(Self::LABEL);
        for suffix in suffixes {
            base.push_str(format!(":{}", suffix).as_ref());
        }
        CacheKey::from(&base)
    }
}

impl Instance {
    pub fn to_raw(&self, color: [f32; 4]) -> InstanceRaw {
        InstanceRaw {
            model: self.transform.to_model_matrix().into(),
            normal: cgmath::Matrix3::from(self.transform.rotation).into(),
            color,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
    color: [f32; 4],
}
impl InstanceRaw {
    pub fn cast_slice(data: &[InstanceRaw]) -> &[u8] {
        bytemuck::cast_slice(data)
    }
}

impl Vertex for InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5, // Model matrix row 0
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6, // Model matrix row 1
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7, // Model matrix row 2
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8, // Model matrix row 3
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9, // Normal matrix row 0
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 16]>() + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 10, // Normal matrix row 1
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 16]>() + mem::size_of::<[f32; 6]>())
                        as wgpu::BufferAddress,
                    shader_location: 11, // Normal matrix row 2
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 16]>() + mem::size_of::<[f32; 9]>())
                        as wgpu::BufferAddress,
                    shader_location: 12, // Color
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
