use std::any::Any;

pub trait Command {
    fn as_any (&self) -> &dyn Any;
}