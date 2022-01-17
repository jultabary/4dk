use std::sync::{Arc, Mutex};
use std::thread;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_bus::EventBus;

pub struct EventsProducedByCommandBusDispatcher {
    command_bus: Box<dyn CommandBus>,
    event_bus: Arc<Mutex<dyn EventBus>>,
}

impl EventsProducedByCommandBusDispatcher {
    pub fn new(command_bus: Box<dyn CommandBus>, event_bus: Arc<Mutex<dyn EventBus>>) -> EventsProducedByCommandBusDispatcher {
        EventsProducedByCommandBusDispatcher {
            command_bus,
            event_bus,
        }
    }
}

impl CommandBus for EventsProducedByCommandBusDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Arc<dyn Event>> {
        let events = self.command_bus.dispatch(command);

        let mut events_clone = Vec::new();
        events.iter().for_each(|event| { events_clone.push(event.clone()) });

        let event_bus_clone = self.event_bus.clone();

        thread::spawn(move || {
            let event_bus_clone = event_bus_clone.lock().unwrap();
            events_clone.iter()
                .for_each(|event| {
                    event_bus_clone.dispatch(event.clone());
                });
        });
        return events;
    }
}
