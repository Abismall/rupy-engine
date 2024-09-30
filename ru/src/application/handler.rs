use crate::{gpu::GPUGlobal, log_debug, log_error, render::surface::TargetSurface, Rupy};
use std::{borrow::BorrowMut, sync::Arc};
use wgpu::InstanceDescriptor;
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use super::{
    event::{EventHandler, EventProcessor},
    state::ApplicationState,
};

impl winit::application::ApplicationHandler for Rupy {
    fn resumed(&mut self, el: &winit::event_loop::ActiveEventLoop) {
        self.state
            .is_none()
            .then(|| self.state = Some(rehydrate(el)));
    }
    fn new_events(
        &mut self,
        el: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
    }

    fn window_event(
        &mut self,
        el: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match self.state.borrow_mut() {
            Some(app_mut) => app_mut,
            None => {
                log_debug!("Failed to borrow state as mutable: {:?}", event);
                return;
            }
        };
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

fn rehydrate(el: &ActiveEventLoop) -> ApplicationState {
    let window = Arc::new(
        el.create_window(crate::utilities::default_window_attributes(None, None))
            .expect("Create window failed on resume"),
    );

    let gpu_res = pollster::block_on(GPUGlobal::initialize(Some(InstanceDescriptor::default())));
    let gpu = match gpu_res {
        Ok(initialized) => initialized,
        Err(e) => {
            log_error!("{:?}", e);
            panic!("{}", format!("{:?}", e));
        }
    };

    let instance = gpu.instance();

    let instance = instance.read().expect("Failed to lock the instance");

    let surface = instance
        .create_surface(window.clone())
        .expect("Failed to create surface");

    let device = gpu.device().clone();
    let adapter = gpu.adapter();

    let adapter = adapter.read().expect("Failed to lock the adapter");

    let surface_format = surface.get_capabilities(&adapter).formats[0];
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    match pollster::block_on(ApplicationState::build(
        TargetSurface::new(window, device, surface_config, surface),
        Some(InstanceDescriptor::default()),
    )) {
        Ok(rehydrated) => rehydrated,
        Err(e) => {
            log_error!("Failed to initialize application state: {:?}", e);
            panic!("{}", format!("{:?}", e));
        }
    }
}
