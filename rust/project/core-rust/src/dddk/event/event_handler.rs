use std::any::{Any, TypeId};
use std::sync::Arc;
use crate::dddk::event::event::Event;
use crate::dddk::aliases::GenericError;

pub trait EventHandler<E: Sized + Any + Event> {
    fn handle_generic_event(&self, event: Arc<dyn Event>) {
        if let Some(event_ref) = event.as_any().downcast_ref::<E>() {
            let _result = self.handle(event_ref.clone());
        } else {
            panic!("Given event is not associated with the handler !");
        }
    }

    fn handle(&self, event: &E) -> Result<(), GenericError>;

    fn get_associated_event(&self) -> TypeId {
        return TypeId::of::<E>();
    }
}

pub trait EventHandlerInBus {
    fn handle_from_bus(&self, event: Arc<dyn Event>);

    fn get_associated_event_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;

    fn get_event_handler_name(&self) -> String;
}