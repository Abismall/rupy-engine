use winit::{
    event::{Event, InnerSizeWriter, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

pub enum WindowAction {
    Resize(winit::dpi::PhysicalSize<u32>),
    Resume(bool),
    Redraw(WindowId),
    Scale(f64, InnerSizeWriter),
    Flush(WindowId),
    Exit(WindowId),
    None,
}

pub enum Actions {
    Window(WindowAction),
}

pub trait EventProcessor {
    fn process<T>(event: Event<T>, el: &ActiveEventLoop) -> Actions;
    fn process_window_event(
        window_id: WindowId,
        event: WindowEvent,
        el: &ActiveEventLoop,
    ) -> Actions;
}
pub struct EventHandler;
impl EventProcessor for EventHandler {
    fn process<T>(event: Event<T>, el: &ActiveEventLoop) -> Actions {
        let action = match event {
            Event::NewEvents(start_cause) => process_new_events(start_cause),
            Event::WindowEvent { window_id, event } => {
                <EventHandler as EventProcessor>::process_window_event(window_id, event, el)
            }
            Event::DeviceEvent { .. } => Actions::Window(WindowAction::None),
            Event::UserEvent(_) => Actions::Window(WindowAction::None),
            Event::Suspended => Actions::Window(WindowAction::None),
            Event::Resumed => Actions::Window(WindowAction::None),
            Event::AboutToWait => Actions::Window(WindowAction::None),
            Event::LoopExiting => Actions::Window(WindowAction::None),
            Event::MemoryWarning => Actions::Window(WindowAction::None),
        };
        match action {
            Actions::Window(WindowAction::Resize(..)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::Resume(_)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::Redraw(..)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::Scale(_, ..)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::Flush(..)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::Exit(..)) => Actions::Window(WindowAction::None),
            Actions::Window(WindowAction::None) => Actions::Window(WindowAction::None),
        }
    }
    fn process_window_event(
        window_id: WindowId,
        event: WindowEvent,
        el: &ActiveEventLoop,
    ) -> Actions {
        match event {
            WindowEvent::Resized(physical_size) => {
                Actions::Window(WindowAction::Resize(physical_size))
            }
            WindowEvent::CloseRequested => Actions::Window(WindowAction::Exit(window_id)),
            WindowEvent::RedrawRequested => Actions::Window(WindowAction::Redraw(window_id)),
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            } => Actions::Window(WindowAction::Scale(scale_factor, inner_size_writer)),
            _ => Actions::Window(WindowAction::None),
        }
    }
}

// pub fn device_event(device_id: DeviceId, event: DeviceEvent, el: &ActiveEventLoop) -> Actions {
//     match event {
//         // DeviceEvent::MouseMotion { delta } => Actions::Device(Actions;:(delta)),
//         // DeviceEvent::MouseWheel { delta } => match delta {
//         //     MouseScrollDelta::LineDelta(x, y) => Actions::Device(DeviceEnum::MouseWheel(x, y)),
//         //     _ => Actions::Device(DeviceEnum::None(device_id)),
//         // },
//         // DeviceEvent::Key(key_event) => match key_event.physical_key {
//         //     PhysicalKey::Code(KeyCode::KeyW) => {
//         //         Actions::Device(DeviceEnum::MoveForward(key_event.state))
//         //     }
//         //     PhysicalKey::Code(KeyCode::KeyS) => {
//         //         Actions::Device(DeviceEnum::MoveBackward(key_event.state))
//         //     }
//         //     PhysicalKey::Code(KeyCode::KeyA) => {
//         //         Actions::Device(DeviceEnum::MoveLeft(key_event.state))
//         //     }
//         //     PhysicalKey::Code(KeyCode::KeyD) => {
//         //         Actions::Device(DeviceEnum::MoveRight(key_event.state))
//         //     }
//         //     _ => Actions::Device(DeviceEnum::None(device_id)),
//         // },
//         _ => Actions::Device(Actions::None),
//     }
// }

pub fn process_new_events(cause: winit::event::StartCause) -> Actions {
    match cause {
        winit::event::StartCause::ResumeTimeReached { .. } => Actions::Window(WindowAction::None),
        winit::event::StartCause::WaitCancelled { .. } => Actions::Window(WindowAction::None),
        winit::event::StartCause::Poll | winit::event::StartCause::Init => {
            Actions::Window(WindowAction::None)
        }
    }
}
