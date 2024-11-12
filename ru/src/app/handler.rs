use crate::{
    app::context::RenderContext,
    events::{RupyAppEvent, WorkerTaskCompletion},
    gpu::RenderMode,
    input::InputListener,
    log_error, log_info,
    prelude::rupy::Rupy,
    texture::create_depth_texture_with_view,
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
                    context.draw_debug_info();
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
                context.render_surface.surface_config.width = size.width;
                context.render_surface.surface_config.height = size.height;
                let _ = context
                    .render_surface
                    .resize(size.width, size.height, &context.device);

                context.depth_texture_view = create_depth_texture_with_view(
                    &context.device,
                    &context.render_surface.surface_config,
                )
                .1;
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, scroll_y) => {
                    context.world.camera.zoom(scroll_y);
                    context.render_surface.window.request_redraw();
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    context.world.camera.zoom(delta.y as f32 * 0.1);
                    context.render_surface.window.request_redraw();
                }
            },
            WindowEvent::MouseInput {
                button,
                state: button_state,
                ..
            } => {
                context.world.camera.on_mouse_button(button, button_state);
            }

            WindowEvent::CursorMoved { position, .. } => {
                let delta_x = position.x - context.last_mouse_position.x;
                let delta_y = position.y - context.last_mouse_position.y;

                context.world.camera.on_mouse_motion((delta_x, delta_y));

                context.last_mouse_position = position;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let is_pressed = event.state.is_pressed();
                let key = event.physical_key;
                match key {
                    PhysicalKey::Code(KeyCode::KeyQ) if is_pressed => {
                        context.toggle_debug_mode();
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft) if is_pressed => {
                        context.toggle_render_mode();
                    }
                    PhysicalKey::Code(KeyCode::Escape) if is_pressed => {
                        self.state.set_shutting_down();
                    }
                    PhysicalKey::Code(KeyCode::KeyP) if is_pressed => {
                        self.state.set_paused();
                    }
                    _ if is_pressed => {
                        if self.state.is_running() {
                            context
                                .world
                                .camera
                                .on_key_event(&event, context.frame.delta_time);
                        }
                    }
                    _ => {}
                }
            }

            WindowEvent::RedrawRequested => context.render_world(),

            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CreateWindow => match self.create_window(event_loop) {
                Ok(win) => {
                    let render_context = match block_on(RenderContext::new(
                        win.into(),
                        RenderMode::TriangleTextureWithDepth,
                        self.debug,
                    )) {
                        Ok(ctx) => ctx,
                        Err(e) => {
                            log_error!("{:?}", e);
                            return;
                        }
                    };

                    self.render_context = Some(render_context);
                    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                    self.state.set_running();
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
