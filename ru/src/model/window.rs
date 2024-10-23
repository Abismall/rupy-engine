use std::sync::Arc;

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::log_error;

pub struct WindowWrapper {
    pub current: Option<Arc<winit::window::Window>>,
    pub id: Option<WindowId>,
}

impl WindowWrapper {
    pub fn new() -> Self {
        Self {
            current: None,
            id: None,
        }
    }
    pub fn redraw(&self) {
        if let Some(win) = &self.current {
            win.request_redraw();
        }
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        if let Some(window) = &self.current {
            let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
            size
        } else {
            PhysicalSize::new(800, 600)
        }
    }

    pub fn set_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
        title: String,
        width: f32,
        height: f32,
    ) {
        match event_loop.create_window(
            attributes
                .with_inner_size(LogicalSize::new(width, height))
                .with_title(title)
                .with_visible(false),
        ) {
            Ok(window) => {
                self.id = Some(window.id());
                self.current = Some(Arc::new(window));
            }
            Err(e) => {
                log_error!("Failed to create window: {:?}", e);
            }
        }
    }

    pub fn show_window(&self) {
        if let Some(window) = &self.current {
            window.set_visible(true);
        }
    }
    pub fn is_window_open(&self) -> bool {
        self.current.is_some()
    }
}
