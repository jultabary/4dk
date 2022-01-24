use std::any::Any;
use std::fmt::Debug;

pub trait Query: Debug {
    fn as_any (&self) -> &dyn Any;

    fn get_query_name(&self) -> String;
}