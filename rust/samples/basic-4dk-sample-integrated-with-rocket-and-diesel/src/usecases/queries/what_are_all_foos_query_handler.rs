use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use dddk_core::dddk::aliases::ResponseFromHandler;
use dddk_core::dddk::query::query_handler::QueryHandler;
use dddk_macro::Query;
use dddk_macro::QueryHandlerInBus;
use crate::domain::repository::FooRepository;

#[derive(Query)]
pub struct WhatAreAllTheFoosQuery {}

impl Debug for WhatAreAllTheFoosQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WhatAreAllTheFoosQuery")
    }
}

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
    fn handle(&self, _query: &WhatAreAllTheFoosQuery) -> ResponseFromHandler {
        Ok(self.foo_repository.get_all_foo())
    }
}
