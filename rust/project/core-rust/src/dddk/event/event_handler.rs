use std::any::{Any, TypeId};
use crate::dddk::event::event::Event;

pub trait EventHandler<E: Sized + Any + Event> {
    fn handle_generic_event<'a>(&self, event: &'a dyn Event) {
        let cast_event = event.as_any().downcast_ref::<E>();
        if cast_event.is_some() {
            self.handle(cast_event.unwrap());
        }
    }

    fn handle(&self, event: &E);

    fn get_associated_event(&self) -> TypeId {
        return TypeId::of::<E>();
    }
}

pub trait EventHandlerInBus {
    fn handle_from_bus<'a>(&self, query: &'a dyn Event);

    fn get_associated_event_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;
}