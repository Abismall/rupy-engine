use std::{cell::RefCell, rc::Rc, sync::Arc};

use glyphon::Family;
use wgpu::rwh::HasDisplayHandle;
use winit::{
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use crate::{
    input::handler::{InputHandler, InputListener},
    menu::{item::MenuItem, menu::Menu, wrapper::MenuWrapper},
    rupyLogger::LogFactory,
    utilities::TITLE,
};

use super::{gpu::GPU, state::ApplicationState};

pub struct Rupy {
    state: Option<ApplicationState>,
    #[cfg(feature = "logging")]
    pub logger: Option<LogFactory>,
    input: InputHandler,
    menu: Rc<RefCell<Menu<MainMenu, &'static str>>>,
}

impl Rupy {
    pub const TITLE: &str = TITLE;

    pub fn new() -> Self {
        let mut input = InputHandler::new();
        let menu = Rc::new(RefCell::new(main_menu()));
        menu.borrow_mut().activate();

        input.add_listener(Box::new(MenuWrapper::new(menu.clone())) as Box<dyn InputListener>);

        Rupy {
            #[cfg(feature = "logging")]
            logger: Some(Default::default()),
            input,
            state: None,
            menu,
        }
    }

    pub fn initialize_state(&mut self, window: Arc<Window>) {
        let state = pollster::block_on(ApplicationState::new(window, GPU::default()));
        self.state = Some(state);
    }
    pub fn window(mut self, window: Arc<Window>) -> Self {
        if let Some(ref mut state) = self.state {
            state.window = window;
        }
        self // Return the modified self
    }

    pub fn gpu(mut self, gpu: GPU) -> Self {
        if let Some(ref mut state) = self.state {
            state.gpu = gpu;
        }
        self // Return the modified self
    }

    #[cfg(feature = "logging")]
    pub fn logger(mut self, logger: Option<LogFactory>) -> Self {
        use crate::rupyLogger::LogFactory;

        self.logger = logger;
        self
    }
}
// "Hello world! 👋",
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
                GPU::default(),
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
            if state.window.display_handle().is_ok() {
                match event {
                    WindowEvent::Resized(size) => {
                        state.surface_config.width = size.width;
                        state.surface_config.height = size.height;
                        state
                            .surface
                            .configure(&state.device, &state.surface_config);

                        state.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        state.set_text(
                            "️Hello world! 👋",
                            glyphon::Metrics {
                                font_size: 42.0,
                                line_height: 22.0,
                            },
                            Family::SansSerif,
                        );
                        let _ = state.render();
                    }
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    _ => {}
                }
            } else {
                self.initialize_state(Arc::new(
                    event_loop
                        .create_window(WindowAttributes::default())
                        .expect("Create Window"),
                ));
            }
        }
    }
}

//

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
