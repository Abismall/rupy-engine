use std::sync::Arc;

use crate::{
    app::context::RenderContext,
    camera::{projection::CameraProjection, Camera},
    ecs::geometry::plane::Plane3D,
    events::{RupyAppEvent, WorkerTaskCompletion},
    gpu::RenderMode,
    input::InputListener,
    log_error, log_info,
    prelude::{
        constant::{PERSPECTIVE_FAR, PERSPECTIVE_NEAR, ZERO_F32},
        rupy::Rupy,
    },
};
use nalgebra::{Point3, Vector3};
use pollster::block_on;
use winit::{
    event::{MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let _ = block_on(self.initialize(
            RenderMode::TriangleColorNoDepth,
            Camera::new(
                Point3::new(ZERO_F32, 5.0, ZERO_F32),
                Point3::new(ZERO_F32, ZERO_F32, ZERO_F32),
                *Vector3::y_axis(),
                CameraProjection::new_perspective(
                    16.0 / 9.0,
                    std::f32::consts::FRAC_PI_4,
                    PERSPECTIVE_NEAR,
                    PERSPECTIVE_FAR,
                ),
                1.0,
                Some(Plane3D::new(10.0, 10.0, 1.0)),
            ),
        ));
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        self.update();
        if let Some(context) = &mut self.render_context {
            context.redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.state.is_shutting_down() || event == WindowEvent::CloseRequested {
            self.shutdown(event_loop);
        }
        let context = if let Some(ctx) = &mut self.render_context {
            ctx
        } else {
            return;
        };

        match event {
            WindowEvent::Resized(size) => context.resize(size),
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, scroll_y) => {
                    context.camera.zoom(scroll_y);
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    context.camera.zoom(delta.y as f32 * 0.1);
                }
            },
            WindowEvent::MouseInput {
                button,
                state: button_state,
                ..
            } => {
                context.camera.on_mouse_button(button, button_state);
            }

            WindowEvent::CursorMoved { position, .. } => {
                let delta_x = position.x - context.last_mouse_position.x;
                let delta_y = position.y - context.last_mouse_position.y;

                context.camera.on_mouse_motion((delta_x, delta_y));

                context.last_mouse_position = position;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let is_pressed = event.state.is_pressed();
                let key = event.physical_key;
                match key {
                    PhysicalKey::Code(KeyCode::KeyQ) if is_pressed => context.set_next_debug_mode(),

                    PhysicalKey::Code(KeyCode::ShiftLeft) if is_pressed => {
                        context.set_next_render_mode()
                    }

                    PhysicalKey::Code(KeyCode::Escape) if is_pressed => {
                        self.state.set_shutting_down()
                    }

                    PhysicalKey::Code(KeyCode::KeyP) if is_pressed => {
                        match self.state.is_running() {
                            true => self.state.set_paused(),
                            false => self.state.set_running(),
                        }
                    }

                    _ if is_pressed => {
                        if self.state.is_running() {
                            context
                                .camera
                                .on_key_event(&event, context.frame.delta_time);
                        }
                    }
                    _ => {}
                }
            }

            WindowEvent::RedrawRequested => {
                if let Err(e) = context.render_world() {
                    log_error!("Error rendering world: {:?}", e);
                }
            }

            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CreateWindow(render_mode, camera) => match self.create_window(event_loop)
            {
                Ok(win) => {
                    let window = Arc::new(win);

                    let ctx_bind = RenderContext::new(window, camera, render_mode, self.debug);
                    match block_on(ctx_bind) {
                        Ok(ctx) => {
                            self.render_context = Some(ctx);
                            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                            self.state.set_running();
                        }
                        Err(e) => {
                            log_error!("{:?}", e);
                            return;
                        }
                    };
                }
                Err(e) => {
                    log_error!("Failed to create window: {:?}", e);
                }
            },
            RupyAppEvent::TaskCompleted(task) => match task {
                WorkerTaskCompletion::LoadTextureFiles(vec) => {
                    log_info!("Loaded {} textures", vec.len());
                }
                WorkerTaskCompletion::LoadShaderFiles(vec) => {
                    log_info!("Loaded {} shaders", vec.len());
                }
            },

            _ => {}
        }
    }
}
