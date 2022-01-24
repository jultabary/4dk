use std::any::Any;
use std::fmt::Debug;

pub trait Event: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;

    fn get_event_name(&self) -> String;
}