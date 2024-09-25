use glyphon::{
    cosmic_text::LineEnding, Attrs, AttrsList, Cache, Family, FontSystem, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureView};

pub struct GlyphonManager {
    font_system: FontSystem,
    atlas: TextAtlas,                // Cache for glyphs
    renderer: TextRenderer,          // Renderer for text
    swash_cache: SwashCache,         // Swash cache for glyph rasterization
    viewport: Viewport,              // Handles viewport sizing for text
    glyphon_buffer: glyphon::Buffer, // Buffer for holding the text lines
}

impl GlyphonManager {
    pub fn new(device: &Device, queue: &Queue, texture_format: wgpu::TextureFormat) -> Self {
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, texture_format);
        let renderer =
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);
        let font_system = FontSystem::new();
        let glyphon_buffer = glyphon::Buffer::new_empty(glyphon::Metrics {
            font_size: 10.0,
            line_height: 10.0,
        });

        GlyphonManager {
            font_system,
            atlas,
            renderer,
            swash_cache,
            viewport,
            glyphon_buffer,
        }
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        device: &Device,
        queue: &Queue,
        view: &TextureView,
        depth_texture_view: &TextureView,
        surface_config: &SurfaceConfiguration,
    ) {
        self.viewport.update(
            queue,
            Resolution {
                width: surface_config.width,
                height: surface_config.height,
            },
        );

        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.glyphon_buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: surface_config.width as i32,
                        bottom: surface_config.height as i32,
                    },
                    default_color: glyphon::Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .expect("Failed to prepare text rendering");

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: Default::default(),
                occlusion_query_set: Default::default(),
            });

            self.renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .expect("Failed to render text");
        }

        self.atlas.trim();
    }

    pub fn clear_buffer(&mut self) {
        self.glyphon_buffer.lines.clear();
    }

    pub fn set_text(&mut self, text: &str, metrics: glyphon::Metrics, font_family: Option<Family>) {
        self.glyphon_buffer
            .set_metrics(&mut self.font_system, metrics);
        self.glyphon_buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(font_family.unwrap_or(Family::Serif)),
            Shaping::Advanced,
        );
        self.glyphon_buffer
            .shape_until_scroll(&mut self.font_system, false);
    }

    pub fn push_buffer_lines(
        &mut self,
        text: &str,
        font_size: [f32; 2],
        font_family: Option<Family>,
        line_ending: Option<LineEnding>,
        attrs_list: Option<AttrsList>,
        shaping: Option<Shaping>,
    ) {
        self.glyphon_buffer.set_metrics(
            &mut self.font_system,
            glyphon::Metrics {
                font_size: font_size[0],
                line_height: font_size[1],
            },
        );

        let attrs_list = attrs_list.unwrap_or(AttrsList::new(
            Attrs::new().family(font_family.unwrap_or(Family::Serif)),
        ));
        let line = glyphon::BufferLine::new(
            text,
            line_ending.unwrap_or(LineEnding::Lf),
            attrs_list,
            shaping.unwrap_or(Shaping::Advanced),
        );
        self.glyphon_buffer.lines.push(line);
        self.glyphon_buffer
            .shape_until_scroll(&mut self.font_system, false);
    }

    pub fn draw_frame_time(&mut self, position: [f32; 2], last_frame_time: std::time::Instant) {
        let now = std::time::Instant::now();
        let frame_time = now.duration_since(last_frame_time);
        let frame_time_ms = frame_time.as_secs_f32() * 1000.0;

        self.push_buffer_lines(
            &format!("Frame Time: {:.6} ms", frame_time_ms),
            [position[0], position[1]],
            None,
            Some(LineEnding::CrLf),
            None,
            None,
        );
    }

    pub fn draw_fps(&mut self, position: [f32; 2], fps: f32) {
        self.push_buffer_lines(
            &format!("FPS: {:.2}", fps),
            [position[0], position[1]],
            None,
            Some(LineEnding::CrLf),
            None,
            None,
        );
    }
}
