use std::sync::Arc;

use super::app::Rupy;
use crate::{
    core::{error::AppError, events::RupyAppEvent, input::process_input_events},
    log_error, log_info, log_warning,
    prelude::constant::CUR_MONITOR_FULLSCREEN,
};
use pollster::block_on;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_none() {
            if let Err(e) = block_on(self.initialize()) {
                log_error!("Rupy::resumed: {:?}", e);
                self.send_event(RupyAppEvent::Shutdown);
            };

            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = if let Some(value) = &mut self.state {
            value
        } else {
            return;
        };
        if state.bit_flags.is_shutting_down() || event == WindowEvent::CloseRequested {
            shutdown(event_loop);
        }
        process_input_events(&event, || {
            state.input(&event, state.renderer.ctx.frame_metrics().delta_time);
        });

        match event {
            WindowEvent::Resized(size) => state.resize(size),

            WindowEvent::KeyboardInput { event, .. } => {
                if !event.state.is_pressed() {
                    return;
                }
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyQ) => state.renderer.ctx.set_next_debug_mode(),
                    PhysicalKey::Code(KeyCode::ControlLeft) => {
                        state.renderer.ctx.set_next_topology()
                    }

                    PhysicalKey::Code(KeyCode::Escape) => state.bit_flags.set_shutting_down(),
                    PhysicalKey::Code(KeyCode::Tab) => {
                        if state.window.is_resizable() {
                            if state.window.fullscreen() != None {
                                state.window.set_fullscreen(None);
                            } else {
                                state.window.set_fullscreen(CUR_MONITOR_FULLSCREEN);
                            }
                        }
                    }

                    PhysicalKey::Code(KeyCode::KeyP) => {
                        if state.bit_flags.is_running() {
                            state.bit_flags.set_paused()
                        } else {
                            state.bit_flags.set_running()
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorEntered { .. } => {
                state.window.set_cursor_visible(false);
            }
            WindowEvent::CursorLeft { .. } => {
                state.window.set_cursor_visible(true);
            }
            WindowEvent::RedrawRequested => {
                if state.bit_flags.is_running() {
                    state.update();
                    state.render();
                }
                state.window.request_redraw()
            }

            _ => {}
        }
    }

    fn user_event(&mut self, el: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CreateWindow => {
                let result = self.create_window(el);
                handle_create_window(self, result);
            }
            RupyAppEvent::WindowCreated(window) => {
                handle_window_created(self, window);
            }
            RupyAppEvent::RenderStart(window) => {
                pollster::block_on(handle_render_start(self, window));
            }
            RupyAppEvent::TaskCompleted => {}
            _ => {}
        }
    }
}

async fn handle_render_start(app: &mut Rupy, window: Arc<winit::window::Window>) {
    if app.state.is_none() {
        if app.initialize_state(window).await {
            match &mut app.state {
                Some(state) => {
                    state.bit_flags.set_running();
                }
                None => {
                    log_warning!("State returned none after initialization.");
                    app.send_event(RupyAppEvent::Shutdown);
                }
            }
        }
    }
}

fn handle_create_window(app: &Rupy, result: Result<winit::window::Window, AppError>) {
    match result {
        Ok(window) => {
            let arc_win = Arc::new(window);
            app.send_event(RupyAppEvent::WindowCreated(arc_win));
        }
        Err(e) => {
            log_error!("create_window: {:?}", e);
        }
    }
}

fn handle_window_created(app: &mut Rupy, window: Arc<winit::window::Window>) {
    if app.state.is_none() {
        app.send_event(RupyAppEvent::RenderStart(window))
    }
}

fn shutdown(event_loop: &ActiveEventLoop) {
    if event_loop.exiting() {
        return;
    } else {
        log_info!("Exit");
        event_loop.exit();
    };
}
