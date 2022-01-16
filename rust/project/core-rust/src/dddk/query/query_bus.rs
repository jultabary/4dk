use crate::dddk::query::query::Query;
use crate::dddk::query::response::Response;

pub trait QueryBus {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> Vec<Box<dyn Response>>;
}