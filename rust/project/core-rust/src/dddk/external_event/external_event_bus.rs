use crate::dddk::aliases::Commands;
use crate::dddk::external_event::external_event::ExternalEvent;

pub trait ExternalEventBus {
    fn dispatch(&self, external_event: &dyn ExternalEvent) -> Commands;
}