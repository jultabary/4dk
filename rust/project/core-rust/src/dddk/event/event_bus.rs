use std::sync::Arc;
use crate::dddk::event::event::Event;

pub trait EventBus {
    fn dispatch(&mut self, event: Arc<dyn Event>);
}