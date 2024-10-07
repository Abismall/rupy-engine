use crossbeam::channel::Receiver;
use std::collections::HashMap;
use std::sync::Arc;

use crate::log_debug;

use super::event::EventProxyTrait;

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

    pub async fn process_events(&self) {
        while let Ok(event) = self.receiver.recv() {
            for handler in self.subscribers.values() {
                handler(&event);
            }

            if let Err(e) = self.event_loop_proxy.send_event(event) {
                log_debug!("Failed to send event: {:?}", e);
            }
        }
    }
}
