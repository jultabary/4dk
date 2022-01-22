use std::any::{Any, TypeId};
use std::rc::Rc;
use dddk_core::dddk::aliases::Responses;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
use crate::domain::repository::FooRepository;

pub struct WhatAreAllTheFoosQuery {}

impl Query for WhatAreAllTheFoosQuery {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

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

impl QueryHandlerInBus for WhatAreAllTheFoosQueryHandler {
    fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Responses {
        self.handle_generic_query(query)
    }

    fn get_associated_query_from_bus(&self) -> TypeId {
        TypeId::of::<WhatAreAllTheFoosQuery>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
