use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_bus::EventBus;
use crate::dddk::event::event_handler::EventHandlerInBus;

pub struct EventDispatcher {
    event_handlers: HashMap<TypeId, Vec<Box<dyn EventHandlerInBus>>>,
}

impl EventDispatcher {
    pub fn new(event_handlers: Vec<Box<dyn EventHandlerInBus>>) -> EventDispatcher {
        let mut map = HashMap::new() as HashMap<TypeId, Vec<Box<dyn EventHandlerInBus>>>;
        event_handlers.into_iter().for_each(|item| {
            if let Some(a_vec) = map.get_mut(&item.get_associated_event_from_bus()) {
                a_vec.push(item);
            } else {
                let mut vec = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
                let type_id = item.get_associated_event_from_bus();
                vec.push(item);
                map.insert(type_id, vec);
            }
        });
        EventDispatcher {
            event_handlers: map
        }
    }

    pub fn get_event_handlers_by_its_events(&self, type_id: TypeId) -> Option<&Vec<Box<dyn EventHandlerInBus>>> {
        if let Some(event_handlers) = self.event_handlers.get(&type_id) {
            return Some(event_handlers);
        }
        None
    }
}

impl EventBus for EventDispatcher {
    fn dispatch(&self, event: Arc<dyn Event>) {
        if let Some(event_handlers) = self.event_handlers.get(&event.as_any().type_id()) {
            event_handlers.iter()
                .for_each(|event_handler| { event_handler.handle_from_bus(event.clone()); });
        }
    }
}

unsafe impl Send for EventDispatcher { }
unsafe impl Sync for EventDispatcher { }