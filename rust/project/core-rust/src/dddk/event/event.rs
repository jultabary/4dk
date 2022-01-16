use std::any::Any;

pub trait Event {
    fn as_any(&self) -> &dyn Any;
}