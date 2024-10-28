use crate::core::error::AppError;

use super::layout::UILayout;

#[derive(Debug)]
pub struct Menu {
    pub name: String,
    pub layout: UILayout,
}

impl Menu {
    pub fn new(name: String, window_size: (f32, f32)) -> Self {
        let layout = UILayout::new(window_size);
        Self { name, layout }
    }

    pub fn render<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) -> Result<(), AppError> {
        Ok(())
    }
}
