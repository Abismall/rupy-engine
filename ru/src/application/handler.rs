use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowAttributes};

use crate::{
    events::RupyAppEvent,
    graphics::texture::TEXTURE_DIR,
    log_error, log_info,
    prelude::{rupy::Rupy, state::AppState},
};

use super::worker;

impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match pollster::block_on(self.initialize()) {
            Ok(_) => {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
            }
            Err(e) => {
                log_error!("Initialization failed: {:?}", e);

                if let Err(e) = self.send_event(RupyAppEvent::Shutdown(0)) {
                    log_error!("Failed to send shutdown event: {:?}", e);
                    self.exit_process(0);
                }
            }
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        match cause {
            winit::event::StartCause::Poll => {
                if let Err(e) = self.update() {
                    log_error!("Error during update: {:?}", e);
                    self.send_event(RupyAppEvent::Shutdown(0)).ok();
                    return;
                }
                if self.contains_flag(AppState::INITIALIZED) {
                    if let Err(e) = self.render_pass() {
                        log_error!("Error during rendering: {:?}", e);
                        self.send_event(RupyAppEvent::Shutdown(0)).ok();
                    }
                }
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if let Err(e) = self.send_event(RupyAppEvent::Shutdown(0)) {
                    log_error!("Failed to send shutdown event: {:?}", e);
                    std::process::exit(1);
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(surface) = &mut self.surface {
                    let _ = surface.resize(size);
                };
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::Initialized => {
                self.set_flag(AppState::INITIALIZED);

                let width = 800;
                let height = 600;
                let mut init_event: Option<RupyAppEvent> = None;

                if self.window.current.is_none() {
                    let attr = WindowAttributes::default();
                    let title = String::from("Rupy");
                    init_event = Some(RupyAppEvent::CreateWindow(
                        attr,
                        title,
                        width as f32,
                        height as f32,
                    ));
                } else if self.surface.is_none() {
                    init_event = Some(RupyAppEvent::CreateSurface(width, height));
                }

                if let Some(event) = init_event {
                    if let Err(e) = self.send_event(event) {
                        log_error!("Failed to send init event: {:?}", e);
                    }
                }

                if let Err(e) = self.send_task(worker::RupyWorkerTask::TextureFileCacheSetup(
                    String::from(TEXTURE_DIR),
                    String::from("png"),
                )) {
                    log_error!("Failed to send TextureFileCacheSetup task: {:?}", e);
                }
            }
            RupyAppEvent::CreateWindow(attr, title, width, height) => {
                self.create_window(event_loop, attr, title, width, height);
                if self.surface.is_none() {
                    if let Err(e) =
                        self.send_event(RupyAppEvent::CreateSurface(width as u32, height as u32))
                    {
                        log_error!("Failed to create surface event: {:?}", e);
                    }
                }
            }
            RupyAppEvent::CreateSurface(width, height) => {
                if let Err(e) = self.create_and_configure_surface(width, height) {
                    log_error!("Failed to create surface: {:?}", e);
                } else {
                    self.set_flag(AppState::SURFACE);
                    self.window.show_window();
                }
            }
            RupyAppEvent::TextureCacheSetupCompleted(textures) => {
                if let Err(e) = self.resources.load_textures(textures) {
                    log_error!("Failed to load textures into resource manager: {:?}", e);
                }
            }
            RupyAppEvent::TextureCacheSetupFailed(error) => {
                log_info!("Texture cache setup failed: {}", error);
            }
            RupyAppEvent::Shutdown(grace_period) => {
                self.exit_process(grace_period);
            }
            _ => {}
        }
    }
}

// use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

// use crate::{
//     prelude::rupy::Rupy,
//     system::input::{InputEvent, KeyInputEventType, MouseButtonElementState, MouseInputEventType},
// };

// use super::RupyAppEvent;

// impl Rupy {
//     pub(crate) fn handle_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         _window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         match event {
//             WindowEvent::Resized(size) => self.resize(size),
//             WindowEvent::RedrawRequested => {
//                 let _ = self.update();
//                 let _ = self.render();
//             }
//             WindowEvent::CloseRequested => event_loop.exit(),
//             WindowEvent::KeyboardInput { event, .. } => {
//                 self.input
//                     .process_event(InputEvent::KeyInput(KeyInputEventType::Key(event)));
//             }
//             WindowEvent::MouseInput { state, button, .. } => {
//                 self.input
//                     .process_event(InputEvent::MouseInput(MouseInputEventType::Button(
//                         MouseButtonElementState { button, state },
//                     )));
//             }
//             WindowEvent::CursorMoved { position, .. } => {
//                 self.input
//                     .process_event(InputEvent::MouseInput(MouseInputEventType::Motion(
//                         position.into(),
//                     )));
//             }
//             _ => {}
//         }
//     }
// }
// impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
//     fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
//         event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
//     }

//     fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
//         let Some(state) = &mut self.state else {
//             return;
//         };

//         match cause {
//             _ => {
//                 state.window.redraw();
//             }
//         }
//     }

//     fn window_event(
//         &mut self,
//         event_loop: &winit::event_loop::ActiveEventLoop,
//         window_id: winit::window::WindowId,
//         event: WindowEvent,
//     ) {
//         self.handle_event(event_loop, window_id, event);
//     }

//     fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
//         match event {
//             RupyAppEvent::TextureCacheMetadaLoaded(textures) => {
//                 self.setup(textures);
//             }
//             RupyAppEvent::ToggleConsole => (),
//             RupyAppEvent::CloseRequested => event_loop.exit(),
//             RupyAppEvent::ToggleDebugMode => self.toggle_debug_mode(),
//             RupyAppEvent::CreateSurfaceRequested => {
//                 self.init_surface(event_loop);
//             }
//             _ => {}
//         }
//     }
// }
