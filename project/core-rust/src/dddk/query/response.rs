use std::any::Any;

pub trait Response {
    fn as_any(&self) -> &dyn Any;
}