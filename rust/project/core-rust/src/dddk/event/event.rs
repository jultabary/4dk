use std::any::Any;

pub trait Event: Send {
    fn as_any(&self) -> &dyn Any;
}