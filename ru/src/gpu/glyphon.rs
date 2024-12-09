use glyphon::{
    cosmic_text::LineEnding, Attrs, AttrsList, Cache, Family, FontSystem, Shaping, SwashCache,
    TextArea, TextAtlas, TextRenderer, Viewport,
};
use glyphon::{Buffer, Resolution, TextBounds};
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};

use crate::app::DebugMode;
use crate::camera::Camera;
use crate::prelude::frame::FrameTime;

pub struct GlyphonRender {
    pub font_system: ::glyphon::FontSystem,
    pub swash_cache: ::glyphon::SwashCache,
    pub viewport: ::glyphon::Viewport,
    pub atlas: ::glyphon::TextAtlas,
    pub renderer: ::glyphon::TextRenderer,
    pub buffer: Buffer,
}

pub struct GlyphonManager {
    font_system: FontSystem,
    atlas: TextAtlas,
    renderer_2d: TextRenderer,
    renderer_3d: TextRenderer,
    swash_cache: SwashCache,
    viewport: Viewport,
    glyphon_buffer: glyphon::Buffer,
}

impl GlyphonManager {
    pub fn new(
        device: &Device,
        queue: &Queue,
        texture_format: wgpu::TextureFormat,
        depth_stencil: wgpu::DepthStencilState,
    ) -> Self {
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, texture_format);

        let renderer_2d =
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        let renderer_3d = TextRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState::default(),
            Some(depth_stencil),
        );

        let font_system = FontSystem::new();
        let glyphon_buffer = glyphon::Buffer::new_empty(glyphon::Metrics {
            font_size: 10.0,
            line_height: 10.0,
        });

        GlyphonManager {
            font_system,
            atlas,
            renderer_2d,
            renderer_3d,
            swash_cache,
            viewport,
            glyphon_buffer,
        }
    }

    pub fn reconfigure(&mut self, queue: &wgpu::Queue, resolution: glyphon::Resolution) {
        self.viewport.update(queue, resolution);
    }

    pub fn get_text_bounds(
        resolution: Resolution,
        left: Option<i32>,
        top: Option<i32>,
    ) -> TextBounds {
        TextBounds {
            left: left.unwrap_or(10),
            top: top.unwrap_or(10),
            right: resolution.width as i32,
            bottom: resolution.height as i32,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        pass: &mut RenderPass<'a>,
        use_depth: bool,
        device: &Device,
        queue: &Queue,
        surface_config: &SurfaceConfiguration,
    ) {
        self.reconfigure(
            queue,
            Resolution {
                width: surface_config.width,
                height: surface_config.height,
            },
        );
        let _ = if use_depth {
            let _ = &self
                .renderer_3d
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
            self.renderer_3d
                .render(&self.atlas, &self.viewport, pass)
                .expect("Failed to render text");
        } else {
            let _ = &self
                .renderer_2d
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
            self.renderer_2d
                .render(&self.atlas, &self.viewport, pass)
                .expect("Failed to render text");
        };
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

    pub fn draw_debug_info(
        &mut self,
        debug_mode: DebugMode,
        frame_time: &FrameTime,
        camera: &Camera,
    ) {
        self.clear_buffer();

        match debug_mode {
            DebugMode::None => {}
            DebugMode::FPS => {
                self.draw_fps([10.0, 10.0], frame_time.fps);
            }
            DebugMode::Frame => {
                self.draw_frame_time([10.0, 10.0], frame_time);
            }
            DebugMode::Camera => {
                self.draw_camera([10.0, 30.0], camera.position.into(), camera.target.into());
            }
            DebugMode::Verbose => {
                self.draw_fps([10.0, 10.0], frame_time.fps);
                self.draw_frame_time([10.0, 30.0], frame_time);
                self.draw_camera([10.0, 50.0], camera.position.into(), camera.target.into());
            }
        }
        self.glyphon_buffer
            .shape_until_scroll(&mut self.font_system, false);
    }

    pub fn draw_fps(&mut self, position: [f32; 2], fps: f32) {
        self.push_buffer_lines(
            &format!("FPS: {:.2}", fps),
            [position[0], position[1]],
            None,
            Some(LineEnding::Lf),
            None,
            None,
        );
    }

    pub fn draw_frame_time(&mut self, position: [f32; 2], frame_time: &FrameTime) {
        let now = std::time::Instant::now();
        let frame_duration = now.duration_since(frame_time.last_frame_time);
        let frame_time_ms = frame_duration.as_secs_f32() * 1000.0;

        self.push_buffer_lines(
            &format!("Frame Time: {:.2} ms", frame_time_ms),
            [position[0], position[1]],
            None,
            Some(LineEnding::Lf),
            None,
            None,
        );
    }

    pub fn draw_camera(
        &mut self,
        position: [f32; 2],
        camera_position: [f32; 3],
        camera_target: [f32; 3],
    ) {
        self.push_buffer_lines(
            &format!(
                "Camera Position: {:.2?}\nCamera Target: {:.2?}",
                camera_position, camera_target,
            ),
            [position[0], position[1]],
            None,
            Some(LineEnding::Lf),
            None,
            None,
        );
    }
}
