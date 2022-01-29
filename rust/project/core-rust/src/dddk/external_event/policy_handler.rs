use std::any::{Any, TypeId};
use crate::dddk::aliases::Commands;
use crate::dddk::errors::ExternalEventIsNotAssociatedWithThisPolicyHandler;
use crate::dddk::external_event::external_event::ExternalEvent;

pub trait PolicyHandlerInBus {
    fn handle_from_bus<'a>(&self, external_event: &'a dyn ExternalEvent) -> Commands;

    fn get_associated_external_event_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;

    fn get_policy_handler_name(&self) -> String;
}

pub trait PolicyHandler<EE: Sized + Any + ExternalEvent> {
    fn handle_generic_external_event<'a>(&self, external_event: &'a dyn ExternalEvent) -> Commands {
        let cast_external_event = external_event.as_any().downcast_ref::<EE>();
        if cast_external_event.is_some() {
            return self.handle(cast_external_event.unwrap());
        }
        return Err(Box::new(ExternalEventIsNotAssociatedWithThisPolicyHandler {}));
    }

    fn handle(&self, external_event: &EE) -> Commands;

    fn get_associated_external_event(&self) -> TypeId {
        return TypeId::of::<EE>();
    }
}
