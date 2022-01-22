use std::any::{Any, TypeId};
use std::rc::Rc;
use dddk_core::dddk::aliases::Responses;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
use dddk_macro::Query;
use dddk_macro::QueryHandlerInBus;
use crate::domain::repository::FooRepository;

#[derive(Query)]
pub struct WhatAreAllTheFoosQuery {}

#[derive(QueryHandlerInBus)]
pub struct WhatAreAllTheFoosQueryHandler {
    foo_repository: Rc<dyn FooRepository>,
}

impl WhatAreAllTheFoosQueryHandler {
    pub fn new(foo_repository: Rc<dyn FooRepository>) -> WhatAreAllTheFoosQueryHandler {
        WhatAreAllTheFoosQueryHandler {
            foo_repository
        }
    }
}

impl QueryHandler<WhatAreAllTheFoosQuery> for WhatAreAllTheFoosQueryHandler {
    fn handle(&self, _query: &WhatAreAllTheFoosQuery) -> Responses {
        Ok(self.foo_repository.get_all_foo())
    }
}
