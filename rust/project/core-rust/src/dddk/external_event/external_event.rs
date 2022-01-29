use std::any::Any;
use std::fmt::Debug;

pub trait ExternalEvent: Debug {
    fn as_any(&self) -> &dyn Any;

    fn get_external_event_name(&self) -> String;
}