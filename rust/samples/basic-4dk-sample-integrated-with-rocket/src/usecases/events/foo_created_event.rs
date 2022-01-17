use std::any::{Any, TypeId};
use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use dddk_core::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use uuid::Uuid;

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

impl Event for FooCreatedEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct PrintThatFooHasBeenCreatedEventHandler {}

impl EventHandler<FooCreatedEvent> for PrintThatFooHasBeenCreatedEventHandler {
    fn handle(&self, event: &FooCreatedEvent) {
        println!("foo with {} {} has been created", event.id, event.title);
    }
}

impl EventHandlerInBus for PrintThatFooHasBeenCreatedEventHandler {
    fn handle_from_bus(&self, event: Arc<dyn Event>) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        TypeId::of::<FooCreatedEvent>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}