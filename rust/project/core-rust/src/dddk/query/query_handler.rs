use std::any::{Any, TypeId};
use crate::dddk::query::query::Query;
use crate::dddk::query::response::Response;

pub trait QueryHandlerInBus {
    fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Vec<Box<dyn Response>>;

    fn get_associated_query_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;
}

pub trait QueryHandler<C: Sized + Any + Query> {
    fn handle_generic_query<'a>(&self, query: &'a dyn Query) -> Vec<Box<dyn Response>> {
        let mut events = Vec::new() as Vec<Box<dyn Response>>;
        let cast_query = query.as_any().downcast_ref::<C>();
        if cast_query.is_some() {
            events = self.handle(cast_query.unwrap());
        }
        return events;
    }

    fn handle(&self, query: &C) -> Vec<Box<dyn Response>>;

    fn get_associated_query(&self) -> TypeId {
        return TypeId::of::<C>();
    }
}