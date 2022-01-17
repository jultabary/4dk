use std::sync::Arc;
use crate::dddk::event::event::Event;

pub trait EventBus: Send + Sync {
    fn dispatch(&self, event: Arc<dyn Event>);
}