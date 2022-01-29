use std::any::Any;

pub trait ExternalEvent {
    fn as_any(&self) -> &dyn Any;

    fn get_external_event_name(&self) -> String;
}