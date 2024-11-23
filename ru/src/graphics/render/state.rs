use crate::{
    app::DebugMode, camera::Camera, graphics::RenderMode, log_info, prelude::frame::FrameTime,
};

pub struct RenderState {
    camera: Camera,
    frame: FrameTime,
    debug: DebugMode,
    render: RenderMode,
}

impl RenderState {
    pub fn new(camera: Camera, frame: FrameTime, debug: DebugMode, render: RenderMode) -> Self {
        Self {
            camera,
            frame,
            debug,
            render,
        }
    }
    pub fn set_next_debug_mode(&mut self) {
        self.debug = self.debug.next();
        log_info!("Debug Mode: {:?}", self.debug);
    }

    pub fn set_next_render_mode(&mut self) {
        self.render = self.render.next();
        log_info!("Render Mode: {:?}", self.render);
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn frame_time(&self) -> &FrameTime {
        &self.frame
    }

    pub fn set_frame_time(&mut self, frame: FrameTime) {
        self.frame = frame;
    }

    pub fn debug_mode(&self) -> DebugMode {
        self.debug
    }

    pub fn set_debug_mode(&mut self, debug: DebugMode) {
        self.debug = debug;
    }

    pub fn render_mode(&self) -> RenderMode {
        self.render
    }

    pub fn set_render_mode(&mut self, render: RenderMode) {
        self.render = render;
    }
}
