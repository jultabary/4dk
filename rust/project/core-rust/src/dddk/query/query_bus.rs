use crate::dddk::aliases::Responses;
use crate::dddk::query::query::Query;

pub trait QueryBus {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> Responses;
}