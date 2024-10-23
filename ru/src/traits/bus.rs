use crate::events::proxy::EventProxy;
use crate::events::RupyAppEvent;
use std::sync::{Arc, RwLock};

pub trait EventBusListenTrait {
    fn subscribe<T>(component: Arc<RwLock<T>>, bus: &mut EventProxy<RupyAppEvent>)
    where
        T: 'static + std::marker::Send + std::marker::Sync;
}

pub trait EventProxyTrait<T: 'static + std::fmt::Debug> {
    fn send_event(&self, event: T) -> Result<(), winit::event_loop::EventLoopClosed<T>>;
}
