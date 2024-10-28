use wgpu::{DepthStencilState, RenderPass};

use glyphon::{Buffer, Resolution, TextBounds};

use crate::core::error::AppError;
pub struct GlyphonRender {
    pub font_system: ::glyphon::FontSystem,
    pub swash_cache: ::glyphon::SwashCache,
    pub viewport: ::glyphon::Viewport,
    pub atlas: ::glyphon::TextAtlas,
    pub renderer: ::glyphon::TextRenderer,
    pub buffer: Buffer,
}

impl GlyphonRender {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        metrics: glyphon::Metrics,
        depth_stencil: Option<DepthStencilState>,
    ) -> Self {
        let mut font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(&device);
        let mut atlas = glyphon::TextAtlas::new(device, queue, &cache, format);
        let renderer =
            glyphon::TextRenderer::new(&mut atlas, device, Default::default(), depth_stencil);
        let viewport = glyphon::Viewport::new(device, &glyphon::Cache::new(device));
        let buffer = Buffer::new(&mut font_system, metrics);

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            renderer,
            buffer,
        }
    }

    pub fn reconfigure(&mut self, queue: &wgpu::Queue, resolution: glyphon::Resolution) {
        self.viewport.update(queue, resolution);
    }
    pub fn set_buffer_size(&mut self, size: (f32, f32)) {
        self.buffer.set_size(
            &mut self.font_system,
            Some(size.0 as f32),
            Some(size.1 as f32),
        );
    }
    pub fn set_buffer_text(&mut self, text: &str) {
        self.buffer.set_text(
            &mut self.font_system,
            &text,
            glyphon::Attrs::new().family(glyphon::Family::SansSerif),
            glyphon::Shaping::Advanced,
        );
    }

    pub fn shape_until_scroll(&mut self, prune: bool) {
        self.buffer.shape_until_scroll(&mut self.font_system, prune);
    }
    pub fn shape_until_cursor(&mut self, cursor: glyphon::Cursor, prune: bool) {
        self.buffer
            .shape_until_cursor(&mut self.font_system, cursor, prune);
    }
    pub fn render<'a>(&'a self, mut pass: &mut RenderPass<'a>) -> Result<(), AppError> {
        self.renderer
            .render(&self.atlas, &self.viewport, &mut pass)
            .map_err(|e| AppError::RenderError(e))
    }
    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let _ = self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            [glyphon::TextArea {
                buffer: &self.buffer,
                left: 10.0,
                top: 10.0,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 10,
                    top: 10,
                    right: self.viewport.resolution().width as i32,
                    bottom: self.viewport.resolution().height as i32,
                },
                default_color: glyphon::Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            }],
            &mut self.swash_cache,
        );
    }
    pub fn prepare_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: glyphon::TextBounds, // Pass in the text bounds, no need for the console reference
        scale: f32,
    ) -> Result<(), AppError> {
        self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            [glyphon::TextArea {
                buffer: &self.buffer,
                left: bounds.left as f32,
                top: bounds.top as f32,
                scale,
                bounds,
                default_color: glyphon::Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            }],
            &mut self.swash_cache,
        )?;

        Ok(())
    }
    pub fn finalize(&mut self) {
        self.atlas.trim();
        self.buffer.lines.clear();
    }
}
pub fn get_text_bounds(resolution: Resolution, left: Option<i32>, top: Option<i32>) -> TextBounds {
    TextBounds {
        left: left.unwrap_or(10),
        top: top.unwrap_or(10),
        right: resolution.width as i32,
        bottom: resolution.height as i32,
    }
}
