use crate::{
    events::RupyAppEvent,
    input::{InputEvent, KeyInputEventType},
    log_error, log_info,
    prelude::rupy::Rupy,
};
use pollster::block_on;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use super::state::ApplicationStateFlags;

impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Err(e1) = block_on(self.initialize()) {
            log_error!("Initialization failed: {:?}", e1);
            if let Err(_) = self.send_event(RupyAppEvent::Shutdown(0)) {
                self.exit_process(0);
            }
        } else {
            if let Err(e) = self.create_window(event_loop) {
                log_error!("Failed to create window on resume: {:?}", e);
            }
            self.state.set_flag(ApplicationStateFlags::RUNNING);
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        if self.state.is_running() {
            let _ = self.render();
            let _ = self.update();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.state.is_running() {
            match event {
                WindowEvent::CloseRequested => {
                    if let Err(e) = self.send_event(RupyAppEvent::Shutdown(0)) {
                        log_error!("Failed to send shutdown event: {:?}", e);
                        std::process::exit(1);
                    }
                }
                WindowEvent::Resized(..) => self.window.update(),
                WindowEvent::KeyboardInput { event, .. } => self
                    .input
                    .process_event(InputEvent::KeyInput(KeyInputEventType::Key(event))),
                WindowEvent::RedrawRequested => {
                    let _ = self.window.request_redraw();
                }
                _ => {}
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::ToggleDebugMode => self.debug_mode.toggle(),
            RupyAppEvent::LoadTextureTaskCompleted(textures) => {
                log_info!("Loaded {} textures", textures.len());
            }
            RupyAppEvent::ListShaderFilesTaskCompleted(shaders) => {
                log_info!("Loaded {} shaders", shaders.len());
            }
            RupyAppEvent::ExitRequest => {
                if let Err(e) = self.send_event(RupyAppEvent::Shutdown(0)) {
                    log_error!("Failed to send shutdown event: {:?}", e);
                    std::process::exit(1);
                }
            }
            RupyAppEvent::Shutdown(grace_period) => {
                self.exit_process(grace_period);
            }
            _ => {}
        }
    }
}
