use std::thread::Thread;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_bus::EventBus;

struct EventsProducedByCommandBusDispatcher {
    command_bus: Box<dyn CommandBus>,
    event_bus: Box<dyn EventBus>,
}

impl CommandBus for EventsProducedByCommandBusDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        /*let events = self.command_bus.dispatch(command);
        Thread::spawn(|| {
            events.iter()
                .for_each(|event| {
                    self.event_bus.dispatch(event);
                });
        });
        return events;*/
        todo!()
    }
}