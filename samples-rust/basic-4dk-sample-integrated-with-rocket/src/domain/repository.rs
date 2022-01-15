use std::any::Any;

pub trait Repository {
    fn as_any(&self) -> &dyn Any;
}