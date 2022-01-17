use std::any::{Any, TypeId};
use std::sync::Arc;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use crate::dddk::test::some_event_for_test::{AnEvent, AnotherEvent};

pub trait EventHandlerForTest<E: Event + Any>: EventHandlerInBus + EventHandler<E> {
    fn get_handled_events(&self) -> &Vec<i32>;

    fn has_event_been_handled(&self, id: i32) -> bool {
        self.get_handled_events().contains(&id)
    }
}

pub struct AnEventHandler {
    handled_events: Vec<i32>,
}

impl AnEventHandler {
    pub fn new() -> AnEventHandler {
        AnEventHandler { handled_events: Vec::new() }
    }
}

impl EventHandlerForTest<AnEvent> for AnEventHandler {
    fn get_handled_events(&self) -> &Vec<i32> {
        &self.handled_events
    }
}

impl EventHandler<AnEvent> for AnEventHandler {
    fn handle(&mut self, event: &AnEvent) {
        self.handled_events.push(event.id);
    }
}

impl EventHandlerInBus for AnEventHandler {
    fn handle_from_bus(&mut self, event: Arc<dyn Event>) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AnotherEventHandler {
    handled_events: Vec<i32>,
}

impl AnotherEventHandler {
    pub fn new() -> AnotherEventHandler {
        AnotherEventHandler { handled_events: Vec::new() }
    }
}

impl EventHandlerForTest<AnotherEvent> for AnotherEventHandler {
    fn get_handled_events(&self) -> &Vec<i32> {
        &self.handled_events
    }
}

impl EventHandler<AnotherEvent> for AnotherEventHandler {
    fn handle(&mut self, event: &AnotherEvent) {
        self.handled_events.push(event.id);
    }
}

impl EventHandlerInBus for AnotherEventHandler {
    fn handle_from_bus(&mut self, event: Arc<dyn Event>) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub struct AThirdEventHandler {
    handled_events: Vec<i32>,
}

impl EventHandlerForTest<AnEvent> for AThirdEventHandler {
    fn get_handled_events(&self) -> &Vec<i32> {
        &self.handled_events
    }
}

impl EventHandler<AnEvent> for AThirdEventHandler {
    fn handle(&mut self, event: &AnEvent) {
        self.handled_events.push(event.id);
    }
}

impl AThirdEventHandler {
    pub fn new() -> AThirdEventHandler {
        AThirdEventHandler { handled_events: Vec::new() }
    }
}

impl EventHandlerInBus for AThirdEventHandler {
    fn handle_from_bus(&mut self, event: Arc<dyn Event>) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
