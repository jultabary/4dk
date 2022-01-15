use std::any::TypeId;
use std::collections::HashMap;
use crate::dddk::query::query::Query;
use crate::dddk::query::query_bus::QueryBus;
use crate::dddk::query::query_handler::QueryHandlerInBus;
use crate::dddk::query::response::Response;

pub struct QueryDispatcher {
    query_handlers: HashMap<TypeId, Box<dyn QueryHandlerInBus>>,
}

impl QueryDispatcher {
    pub fn new(query_handler_values: Vec<Box<dyn QueryHandlerInBus>>) -> QueryDispatcher {
        let mut map = HashMap::new();
        query_handler_values.into_iter().for_each(|item| {
            map.insert(item.get_associated_query_from_bus(), item);
        });
        return QueryDispatcher {
            query_handlers: map
        };
    }

    pub fn get_query_handler_by_its_query(&self, type_id: TypeId) -> Option<&Box<dyn QueryHandlerInBus>> {
        if let Some(query_handler) = self.query_handlers.get(&type_id) {
            return Some(query_handler);
        }
        None
    }
}

impl QueryBus for QueryDispatcher {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> Vec<Box<dyn Response>> {
        if let Option::Some(query_handler) = self.query_handlers.get(&query.as_any().type_id()) {
            let responses = query_handler.handle_from_bus(query);
            return responses;
        }
        return Vec::new();
    }
}
