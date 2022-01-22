use std::any::{Any, TypeId};
use crate::dddk::aliases::Responses;
use crate::dddk::errors::QueryIsNotAssociatedWithHandler;
use crate::dddk::query::query::Query;

pub trait QueryHandlerInBus {
    fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Responses;

    fn get_associated_query_from_bus(&self) -> TypeId;

    fn get_query_handler_name(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}

pub trait QueryHandler<C: Sized + Any + Query> {
    fn handle_generic_query<'a>(&self, query: &'a dyn Query) -> Responses {
        let cast_query = query.as_any().downcast_ref::<C>();
        if cast_query.is_some() {
            return self.handle(cast_query.unwrap());
        }
        return Err(Box::new(QueryIsNotAssociatedWithHandler {}));
    }

    fn handle(&self, query: &C) -> Responses;

    fn get_associated_query(&self) -> TypeId {
        return TypeId::of::<C>();
    }
}