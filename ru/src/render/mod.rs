pub mod context;
pub mod renderer;
pub mod window;
#[derive(Debug, Clone, Copy)]
pub enum FrontFace {
    Ccw,
    Cw,
}
impl FrontFace {
    pub fn to_wgpu(self) -> wgpu::FrontFace {
        match self {
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
            FrontFace::Cw => wgpu::FrontFace::Cw,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum VFace {
    None,
    Front,
    Back,
}

impl VFace {
    pub fn to_wgpu(self) -> Option<wgpu::Face> {
        match self {
            VFace::None => None,
            VFace::Front => Some(wgpu::Face::Front),
            VFace::Back => Some(wgpu::Face::Back),
        }
    }
}
