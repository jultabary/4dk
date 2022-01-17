
/*struct EventsProducedByCommandBusDispatcher {
    command_bus: Box<dyn CommandBus>,
    event_bus: Box<dyn EventBus>,
}

impl EventsProducedByCommandBusDispatcher {
    pub fn new(command_bus: Box<dyn CommandBus>, event_bus: Box<dyn EventBus>) -> EventsProducedByCommandBusDispatcher {
        EventsProducedByCommandBusDispatcher {
            command_bus,
            event_bus,
        }
    }
}

impl CommandBus for EventsProducedByCommandBusDispatcher {
    fn dispatch<'b>(&self, _command: &'b dyn Command) -> Vec<Arc<dyn Event>> {
        let events = self.command_bus.dispatch(command);
        let mut events_clone = Vec::new();
        events.iter().for_each(|event| { events_clone.push(event.clone())});
        thread::spawn(|| {
            events_clone.iter()
                .for_each(|event| {
                    self.event_bus.dispatch(event.clone());
                });
        });
        return events;
    }
}*/