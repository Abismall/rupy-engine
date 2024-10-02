use super::event::{EventHandler, EventProcessor};
use crate::{log_debug, Rupy};
use pollster::FutureExt;
use std::borrow::{BorrowMut, Cow};
use wgpu::ShaderModuleDescriptor;
use wgpu::ShaderSource;
use winit::event::DeviceEvent;

fn load_shaders(device: &wgpu::Device) -> (wgpu::ShaderModule, wgpu::ShaderModule) {
    let vertex_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!(
            "../../static/shader/vertex.wgsl"
        ))),
    });

    let fragment_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!(
            "../../static/shader/fragment.wgsl"
        ))),
    });

    (vertex_shader, fragment_shader)
}

impl winit::application::ApplicationHandler for Rupy {
    fn resumed(&mut self, el: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_none() {
            let _ = self.rehydrate(&el).block_on();
        }
    }
    fn new_events(
        &mut self,
        el: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        let res = self.state.borrow_mut();
        let state = match res {
            Some(state) => state,
            None => return,
        };
        match cause {
            winit::event::StartCause::ResumeTimeReached {
                start,
                requested_resume,
            } => {
                // Handle logic when the resume time is reached
            }
            winit::event::StartCause::WaitCancelled {
                start,
                requested_resume,
            } => {
                // Handle logic when wait is cancelled
            }
            winit::event::StartCause::Poll => {}
            winit::event::StartCause::Init => {
                // Initial setup for the rendering
            }
        }
    }

    fn window_event(
        &mut self,
        el: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(state) = &mut self.state {
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    log_debug!("Redraw requested!");
                    let frame = match state.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            log::error!("Failed to acquire next swap chain texture: {:?}", e);
                            return;
                        }
                    };

                    // Create a view of the frame's texture
                    let output_view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let device = state.device.clone();
                    let queue = state.queue.clone();
                    let swap_chain_format = state.swap_chain_format;
                    let perspective = state.perspective.clone();
                    // Create a command encoder
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });
                    let (vertex, fragment) = load_shaders(&device);
                    // Render the scene
                    state.create_and_render_scene(
                        &device,
                        &queue,
                        swap_chain_format,
                        &output_view, // Pass the texture view as output_view
                        &mut encoder, // Pass the command encoder
                        &vertex,
                        &fragment,
                        &perspective,
                    );

                    // Submit the command encoder
                    state.queue.submit(Some(encoder.finish()));

                    // Present the frame
                    frame.present();
                }
                _ => {}
            }
        }
    }
    fn device_event(
        &mut self,
        el: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        <EventHandler as EventProcessor>::process::<DeviceEvent>(
            winit::event::Event::UserEvent(event),
            el,
        );
    }
}
