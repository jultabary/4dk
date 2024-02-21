use std::sync::Arc;
use std::thread;
use crate::dddk::aliases::Events;
use crate::dddk::bus::Bus;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event_bus::EventBus;

pub struct EventsProducedByCommandBusDispatcher {
    command_bus: Box<dyn CommandBus>,
    event_bus: Arc<dyn EventBus>,
    async_dispatching: bool,
}

impl EventsProducedByCommandBusDispatcher {
    pub fn new(command_bus: Box<dyn CommandBus>, event_bus: Arc<dyn EventBus>, async_dispatching: bool) -> EventsProducedByCommandBusDispatcher {
        EventsProducedByCommandBusDispatcher {
            command_bus,
            event_bus,
            async_dispatching
        }
    }
}

impl CommandBus for EventsProducedByCommandBusDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command, bus_opt: Option<& dyn Bus>) -> Events {
        let events = self.command_bus.dispatch(command,  bus_opt);
        if events.is_err() {
            return events;
        }
        let events = events.unwrap();
        let mut events_clone = Vec::new();
        events.iter().for_each(|event| { events_clone.push(event.clone()) });

        let event_bus_clone = self.event_bus.clone();
        let thread_execution = thread::spawn(move || {
            events_clone.iter()
                .for_each(|event| {
                    event_bus_clone.dispatch(event.clone());
                });
        });
        if !self.async_dispatching {
            thread_execution.join().unwrap();
        }
        return Ok(events);
    }
}
