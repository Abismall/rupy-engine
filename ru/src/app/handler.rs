use pollster::block_on;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    events::{RupyAppEvent, WorkerTaskCompletion},
    graphics::context::GpuResourceCache,
    input::process_input_events,
    log_error, log_info, log_warning,
};

use super::{app::Rupy, flags::BitFlags, state::State};
impl<'a> winit::application::ApplicationHandler<RupyAppEvent> for Rupy<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let _ = block_on(self.initialize());

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
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
            log_warning!("Service not available!");
            return;
        };
        if state.bit_flags.is_shutting_down() || event == WindowEvent::CloseRequested {
            Rupy::shutdown(event_loop);
        }
        process_input_events(&event, || {
            state.input(&event, state.renderer.ctx.frame_metrics().delta_time);
        });

        match event {
            WindowEvent::Resized(size) => state.resize(size),

            WindowEvent::KeyboardInput { event, .. } => {
                let is_pressed = event.state.is_pressed();
                let key = event.physical_key;
                match key {
                    PhysicalKey::Code(KeyCode::KeyQ) if is_pressed => {
                        state.renderer.ctx.set_next_debug_mode()
                    }
                    PhysicalKey::Code(KeyCode::ControlLeft) if is_pressed => {
                        state.renderer.ctx.set_next_topology()
                    }

                    PhysicalKey::Code(KeyCode::Escape) if is_pressed => {
                        state.bit_flags.set_shutting_down()
                    }

                    PhysicalKey::Code(KeyCode::KeyP) if is_pressed => {
                        match state.bit_flags.is_running() {
                            true => state.bit_flags.set_paused(),
                            false => state.bit_flags.set_running(),
                        }
                    }

                    _ => {}
                }
            }

            WindowEvent::RedrawRequested => {
                if state.bit_flags.is_running() {
                    state.compute();
                    state.update();
                    state.render();
                }
            }

            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CreateWindow => match self.create_window(event_loop) {
                Ok(win) => {
                    let gpu = block_on(GpuResourceCache::new());
                    let bit_flags = BitFlags::empty();
                    match block_on(State::new(gpu, bit_flags, win)) {
                        Ok(value) => {
                            self.state = Some(value);
                        }
                        Err(e) => {
                            log_error!("Failed to create renderer: {:?}", e);
                            return;
                        }
                    };
                    if let Some(state) = &mut self.state {
                        state.bit_flags.set_running();
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                    }
                }

                Err(e) => {
                    log_error!("Failed to create window: {:?}", e);
                    return;
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
