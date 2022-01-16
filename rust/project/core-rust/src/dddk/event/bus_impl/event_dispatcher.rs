use crate::dddk::event::event::Event;
use crate::dddk::event::event_bus::EventBus;

pub struct EventDispatcher { }

impl EventBus for EventDispatcher {
    fn dispatch<'b>(&self, event: &'b dyn Event) {
        todo!()
    }
}