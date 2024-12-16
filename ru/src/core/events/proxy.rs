use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crossbeam::channel::Receiver;
use winit::event_loop::EventLoopProxy;

use crate::log_warning;

use super::RupyAppEvent;

pub trait EventBusListenTrait {
    fn subscribe<T>(component: Arc<RwLock<T>>, bus: &mut EventProxy<RupyAppEvent>)
    where
        T: 'static + std::marker::Send + std::marker::Sync;
}

pub trait EventProxyTrait<T: 'static + std::fmt::Debug> {
    fn send_event(&self, event: T) -> Result<(), winit::event_loop::EventLoopClosed<T>>;
}

pub struct EventProxy<T: 'static + std::fmt::Debug> {
    event_loop_proxy: Arc<EventLoopProxy<T>>,
}

impl<T: 'static + std::fmt::Debug> EventProxy<T> {
    pub fn new(event_loop_proxy: Arc<EventLoopProxy<T>>) -> Self {
        Self { event_loop_proxy }
    }
}

impl<T: 'static + std::fmt::Debug> EventProxyTrait<T> for EventProxy<T> {
    fn send_event(&self, event: T) -> Result<(), winit::event_loop::EventLoopClosed<T>> {
        self.event_loop_proxy.send_event(event)
    }
}
pub struct EventBusProxy<T: 'static + std::fmt::Debug + Send> {
    receiver: Receiver<T>,
    event_loop_proxy: Arc<dyn EventProxyTrait<T> + Send + Sync>,
    subscribers: HashMap<String, Box<dyn Fn(&T) + Send + Sync>>,
}

impl<T: 'static + std::fmt::Debug + Send> EventBusProxy<T> {
    pub fn new(
        receiver: Receiver<T>,
        event_loop_proxy: Arc<dyn EventProxyTrait<T> + Send + Sync>,
    ) -> Self {
        Self {
            receiver,
            event_loop_proxy,
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, key: &str, handler: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.subscribers.insert(key.to_string(), Box::new(handler));
    }

    pub async fn start(&self) {
        while let Ok(event) = self.receiver.recv() {
            for handler in self.subscribers.values() {
                handler(&event);
            }
            if let Err(e) = self.event_loop_proxy.send_event(event) {
                log_warning!("Failed to send event: {:?}", e);
            }
        }
    }
}
