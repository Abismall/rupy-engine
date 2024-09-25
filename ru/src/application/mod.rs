pub mod state;

use crate::{
    gpu::GPUGlobal,
    input::handler::InputHandler,
    input::InputListener,
    menu::{item::MenuItem, menu::Menu, wrapper::MenuWrapper},
    rupyLogger::LogFactory,
    utilities::TITLE,
};
use rand::Rng;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wgpu::rwh::HasDisplayHandle;

use super::state::ApplicationState;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::{
    event::{MouseScrollDelta, RawKeyEvent, WindowEvent},
    window::{Window, WindowAttributes},
};

pub struct Rupy {
    state: Option<ApplicationState>,
    #[cfg(feature = "logging")]
    pub logger: Option<LogFactory>,
    pub(crate) input: InputHandler,
    pub(crate) menu: Rc<RefCell<Menu<MainMenu, &'static str>>>,
    _target_fps: u64,
    _frame_duraion: Duration,
}

impl Rupy {
    pub const TITLE: &str = TITLE;

    pub fn new() -> Self {
        let mut input = InputHandler::new();
        let menu = Rc::new(RefCell::new(main_menu()));
        menu.borrow_mut().activate();
        let target_fps = 300;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
        input.add_listener(Box::new(MenuWrapper::new(menu.clone())) as Box<dyn InputListener>);

        Rupy {
            #[cfg(feature = "logging")]
            logger: Some(Default::default()),
            input,
            state: None,
            menu,
            _target_fps: target_fps,
            _frame_duraion: frame_duration,
        }
    }

    pub fn initialize_state(
        &mut self,
        window: Arc<Window>,
        instance_desc: Option<wgpu::InstanceDescriptor>,
    ) {
        let state = pollster::block_on(ApplicationState::new(window, instance_desc));
        let camera_listener: Box<dyn InputListener> = Box::new(state.camera);
        self.input.add_listener(camera_listener);

        self.state = Some(state);
    }
    pub fn window(mut self, window: Arc<Window>) -> Self {
        if let Some(ref mut state) = self.state {
            state.render_surface.window = window;
        }
        self
    }

    pub fn gpu(mut self, gpu: GPUGlobal) -> Self {
        if let Some(ref mut state) = self.state {
            state.gpu = gpu;
        }
        self
    }

    #[cfg(feature = "logging")]
    pub fn logger(mut self, logger: Option<LogFactory>) -> Self {
        self.logger = logger;
        self
    }
}

impl winit::application::ApplicationHandler for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_some() {
            return;
        } else {
            self.state = Some(pollster::block_on(ApplicationState::new(
                Arc::new(
                    event_loop
                        .create_window(Window::default_attributes())
                        .unwrap(),
                ),
                None,
            )));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(ref mut state) = self.state {
            if state.render_surface.window.display_handle().is_ok() {
                match event {
                    WindowEvent::Resized(size) => {
                        let device_binding = state.gpu.device();
                        state.render_surface.surface_config.width = size.width;
                        state.render_surface.surface_config.height = size.height;
                        state
                            .render_surface
                            .surface
                            .configure(&device_binding, &state.render_surface.surface_config);
                        state.material_manager.resize_depth_texture(
                            &device_binding,
                            &state.render_surface.surface_config,
                        );

                        state.render_surface.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let frame_start = Instant::now();
                        state.debug.compute_metrics();
                        state.draw_debug();
                        state.render_frame();

                        // Ensure the frame time is kept to the target frame duration (FPS cap)
                        let frame_time = frame_start.elapsed();
                        if frame_time < self._frame_duraion {
                            sleep(self._frame_duraion - frame_time);
                        }

                        // Request redraw again to continue the loop at the desired FPS
                        state.render_surface.window.request_redraw();
                    }

                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(_, scroll_y) => {
                            state.camera.zoom(scroll_y);
                            state.render_surface.window.request_redraw();
                        }
                        MouseScrollDelta::PixelDelta(delta) => {
                            state.camera.zoom(delta.y as f32 * 0.1);
                            state.render_surface.window.request_redraw();
                        }
                    },
                    WindowEvent::MouseInput {
                        button,
                        state: button_state,
                        ..
                    } => {
                        state.camera.on_mouse_button(button, button_state);
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let delta_x = position.x - state.last_mouse_position.x;
                        let delta_y = position.y - state.last_mouse_position.y;

                        // Handle mouse motion for camera
                        state.camera.on_mouse_motion((delta_x, delta_y));

                        state.last_mouse_position = position;
                        state.render_surface.window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        state.camera.on_key_event(&event);
                        state.render_surface.window.request_redraw();
                    }

                    _ => {}
                }
            } else {
                let window = event_loop
                    .create_window(WindowAttributes::default())
                    .expect("Window");
                self.initialize_state(Arc::new(window), None);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MainMenu {
    Options,
    Quit,
}

pub fn main_menu() -> Menu<MainMenu, &'static str> {
    let mut options: Vec<MenuItem<MainMenu, &str>> = Vec::new();
    options.insert(0, MenuItem::new("Options", MainMenu::Options));
    options.insert(1, MenuItem::new("Quit", MainMenu::Quit));
    Menu::new(options)
}
