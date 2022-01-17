use std::any::Any;

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}