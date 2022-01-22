use std::any::Any;

pub trait Query {
    fn as_any (&self) -> &dyn Any;

    fn get_query_name(&self) -> String;
}