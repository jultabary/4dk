use std::fmt::{Debug, Formatter};
use dddk_core::dddk::aliases::GenericError;
use dddk_core::dddk::event::event_handler::EventHandler;
use dddk_macro::Event;
use dddk_macro::EventHandlerInBus;
use uuid::Uuid;

#[derive(Event)]
pub struct FooCreatedEvent {
    pub id: Uuid,
    title: String,
}

impl Debug for FooCreatedEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FooCreatedEvent")
    }
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
    fn handle(&self, event: &FooCreatedEvent) -> Result<(), GenericError> {
        println!("foo with {} {} has been created", event.id, event.title);
        Ok(())
    }
}
