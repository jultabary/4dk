use std::any::{Any, TypeId};
use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use dddk_core::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use dddk_macro::Event;
use dddk_macro::EventHandlerInBus;
use uuid::Uuid;

#[derive(Event)]
pub struct FooCreatedEvent {
    pub id: Uuid,
    title: String,
}

impl FooCreatedEvent {
    pub fn new(id: Uuid, title: String) -> FooCreatedEvent {
        FooCreatedEvent {
            id,
            title
        }
    }
}

#[derive(EventHandlerInBus)]
pub struct PrintThatFooHasBeenCreatedEventHandler {}

impl EventHandler<FooCreatedEvent> for PrintThatFooHasBeenCreatedEventHandler {
    fn handle(&self, event: &FooCreatedEvent) {
        println!("foo with {} {} has been created", event.id, event.title);
    }
}
