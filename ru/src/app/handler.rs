use crate::{
    app::context::RenderContext,
    events::{RupyAppEvent, WorkerTaskCompletion},
    graphics::RenderMode,
    input::InputListener,
    log_error, log_info,
    prelude::rupy::Rupy,
    texture::resize_depth_texture,
};
use pollster::block_on;
use wgpu::rwh::HasDisplayHandle;
use winit::{
    event::{MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

impl winit::application::ApplicationHandler<RupyAppEvent> for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let _ = block_on(self.initialize());
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        if self.state.is_running() {
            if let Some(context) = &mut self.render_context {
                if context.window.display_handle().is_ok() {
                    context.frame.compute();
                    context.update_frustum();
                    context.draw_debug();
                    context.window.request_redraw();
                };
            }
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
            WindowEvent::Resized(size) => {
                let device_binding = &context.device;
                context.render_surface.surface_config.width = size.width;
                context.render_surface.surface_config.height = size.height;
                context
                    .render_surface
                    .surface
                    .configure(&device_binding, &context.render_surface.surface_config);
                let (depth_texture, depth_texture_view) =
                    resize_depth_texture(&device_binding, &context.render_surface.surface_config);
                match (depth_texture, depth_texture_view) {
                    (Some(texture), Some(view)) => {
                        context.depth_texture = Some(texture.into());
                        context.depth_texture_view = Some(view.into());
                        context.render_surface.window.request_redraw();
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, scroll_y) => {
                    context.camera.zoom(scroll_y);
                    context.render_surface.window.request_redraw();
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    context.camera.zoom(delta.y as f32 * 0.1);
                    context.render_surface.window.request_redraw();
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
                    PhysicalKey::Code(KeyCode::KeyQ) if is_pressed => {
                        self.toggle_debug_mode();
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft) if is_pressed => {
                        self.toggle_render_mode();
                    }
                    PhysicalKey::Code(KeyCode::Escape) if is_pressed => {
                        self.state.set_shutting_down();
                    }
                    _ if is_pressed => {
                        context
                            .camera
                            .on_key_event(&event, context.frame.delta_time);
                    }
                    _ => {}
                }
            }

            WindowEvent::RedrawRequested => {
                let _ = context.render_frame();
            }

            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CreateWindow => {
                let window = self.create_window(event_loop);
                match window {
                    Ok(win) => {
                        let binding = block_on(RenderContext::new(
                            win.into(),
                            RenderMode::Depth,
                            self.debug,
                        ));
                        self.render_context = Some(binding);
                        self.state.set_running();
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                        log_info!("Event polling started");
                    }
                    Err(e) => {
                        log_error!("Failed to create window: {:?}", e);
                    }
                }
            }
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
