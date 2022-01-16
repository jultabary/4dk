use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_bus::EventBus;
use crate::dddk::event::event_handler::{EventHandler, EventHandlerInBus};

pub struct EventDispatcher {
    event_handlers: HashMap<TypeId, Box<dyn EventHandlerInBus>>
}

impl EventDispatcher {
    pub(crate) fn new(event_handlers: Vec<Box<dyn EventHandlerInBus>>) -> EventDispatcher {
        let mut map = HashMap::new();
        event_handlers.into_iter().for_each(|item| {
            map.insert(item.get_associated_event_from_bus(), item);
        });
        EventDispatcher {
            event_handlers: map
        }
    }
}

impl EventBus for EventDispatcher {
    fn dispatch<'b>(&self, event: &'b dyn Event) {
        if let Some(event_handler) = self.event_handlers.get(&event.as_any().type_id()) {
            event_handler.handle_from_bus(event);
        }
    }
}