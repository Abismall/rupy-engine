pub mod binding;
pub mod context;
pub mod geometry;
pub mod global;
pub mod glyphon;
pub mod pipelines;
pub mod shaders;
pub mod textures;
pub mod uniform;
pub mod vertex;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl PrimitiveTopology {
    pub fn label(&self) -> &'static str {
        match self {
            PrimitiveTopology::PointList => "point_list",
            PrimitiveTopology::LineList => "line_list",
            PrimitiveTopology::LineStrip => "line_strip",
            PrimitiveTopology::TriangleList => "triangle_list",
            PrimitiveTopology::TriangleStrip => "triangle_strip",
        }
    }

    pub fn to_wgpu_topology(&self) -> wgpu::PrimitiveTopology {
        match self {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
    pub fn next(self) -> PrimitiveTopology {
        use PrimitiveTopology::*;
        match self {
            LineList => PrimitiveTopology::LineStrip,
            LineStrip => PrimitiveTopology::TriangleList,
            TriangleList => PrimitiveTopology::TriangleStrip,
            TriangleStrip => PrimitiveTopology::PointList,
            PointList => PrimitiveTopology::LineList,
        }
    }
}
