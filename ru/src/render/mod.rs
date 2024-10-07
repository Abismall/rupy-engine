pub mod context;
pub mod renderer;

pub mod command;
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
// pub trait Renderable {
//     fn vertex_buffer(&self) -> &wgpu::Buffer;
//     fn index_buffer(&self) -> &wgpu::Buffer;
//     fn num_indices(&self) -> u32;
//     fn is_textured(&self) -> bool;

//     fn update_model_uniform(&mut self, queue: &wgpu::Queue, model_matrix: &Mat4);

//     fn render<'a>(
//         &mut self,
//         render_pass: &mut wgpu::RenderPass<'a>,
//         pipeline: &'a wgpu::RenderPipeline,
//         global_bind_group: &'a wgpu::BindGroup,
//     );
// }
