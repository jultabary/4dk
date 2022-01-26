use crate::dddk::aliases::ResponseFromHandler;
use crate::dddk::query::query::Query;

pub trait QueryBus {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> ResponseFromHandler;
}