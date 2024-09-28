use crate::{
    geometry::{self},
    input::{handler::InputHandler, InputListener},
    log_debug,
    material::color::Color,
    rupyLogger::LogFactory,
    utilities::TITLE,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};

use super::state::{ApplicationState, RenderMode};
use winit::{dpi::PhysicalPosition, event::WindowEvent, keyboard::KeyCode, window::Window};

pub struct Rupy {
    state: Option<ApplicationState>,
    #[cfg(feature = "logging")]
    pub logger: Option<LogFactory>,
    pub(crate) input: InputHandler,
}

impl Rupy {
    pub const TITLE: &str = TITLE;

    pub fn new() -> Self {
        let input = InputHandler::new();

        Rupy {
            #[cfg(feature = "logging")]
            logger: Some(Default::default()),
            input,
            state: None,
        }
    }

    pub fn initialize_state(
        &mut self,
        window: Arc<Window>,
        instance_desc: Option<wgpu::InstanceDescriptor>,
    ) {
        self.state = Some(pollster::block_on(ApplicationState::new(
            window,
            instance_desc,
        )));
    }

    pub fn add_object_to_scene(
        &mut self,
        geometry: geometry::Shape,
        position: nalgebra::Vector3<f32>,
        color: Color,
    ) {
        if let Some(state) = &mut self.state {
            log_debug!(
                "Adding object with ID: {:?}, position: {:?}",
                geometry,
                position
            );
            state.add_scene_object(geometry, position, color);
        }
    }

    pub fn toggle_debug_mode(&mut self) {
        if let Some(state) = &mut self.state {
            state.cycle_debug_mode();
            state.add_test_objects();
        }
    }
    pub fn set_debug_mode_for_all(&mut self, enabled: RenderMode) {
        if let Some(state) = &mut self.state {
            state.set_debug_mode_for_all(enabled);
        }
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
            self.toggle_debug_mode();
        }
    }
    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match cause {
            winit::event::StartCause::ResumeTimeReached { .. } | winit::event::StartCause::Init => {
                // It's time to render a new frame
                if let Some(ref mut state) = self.state {
                    state.debug.update();
                    state.camera.update(state.debug.delta_time as f32);
                    state.render_system.target_surface.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(ref mut state) = self.state {
            let next_frame_time = state.debug.last_frame_time + state.frame_duration;
            event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame_time));
        }
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(ref mut state) = self.state {
            match event {
                WindowEvent::Resized(size) => {
                    state.resize(size.width, size.height);
                }
                WindowEvent::RedrawRequested => {
                    state.render_frame();
                }
                WindowEvent::CloseRequested => {
                    // Log close request
                    event_loop.exit();
                }

                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => {
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                            KeyCode::ControlLeft => {
                                state.camera.look_at_object([3.0, 0.0, -5.0].into())
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                    state.camera.on_key_event(&event);
                }
                WindowEvent::CursorMoved {
                    device_id,
                    position,
                } => {
                    let delta_x = position.x - state.last_mouse_position.x;
                    let delta_y = position.y - state.last_mouse_position.y;
                    state.camera.on_mouse_motion((delta_x, delta_y));
                    state.last_mouse_position = PhysicalPosition::new(delta_x, delta_y);
                }

                WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                } => {
                    state.camera.on_scroll(delta);
                }
                _ => (),
            }
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        // Ensure that we have a valid application state and access to the camera
        if let Some(ref mut state) = self.state {
            // Match on the type of device event and pass it directly to the camera
            match event {
                winit::event::DeviceEvent::Key(raw_key_event) => {
                    // If a raw key event is received, handle it by sending to the camera
                    match raw_key_event.physical_key {
                        winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                            KeyCode::KeyQ => self.toggle_debug_mode(),
                            _ => (),
                        },
                        _ => (),
                    }
                }
                _ => (), // Ignore unsupported events
            }
        }
    }
}
