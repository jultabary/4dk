#[cfg(test)]
pub mod some_query_handler_for_test {
    use std::any::{Any, TypeId};
    use crate::dddk::query::query::Query;
    use crate::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
    use crate::dddk::query::response::Response;
    use crate::dddk::test_tools::some_query_for_test::some_query_for_test::{AnotherQuery, AQuery};
    use crate::dddk::test_tools::some_response_for_test::some_response_for_test::{AnotherResponse, AResponse};

    pub struct AQueryHandler {}

    impl QueryHandler<AQuery> for AQueryHandler {
        fn handle(&self, _query: &AQuery) -> Vec<Box<dyn Response>> {
            let a_response = AResponse {};
            vec![Box::new(a_response)]
        }
    }

    impl AQueryHandler {
        pub fn new() -> AQueryHandler {
            AQueryHandler {}
        }
    }

    impl QueryHandlerInBus for AQueryHandler {
        fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Vec<Box<dyn Response>> {
            self.handle_generic_query(query)
        }

        fn get_associated_query_from_bus(&self) -> TypeId {
            self.get_associated_query()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherQueryHandler {}

    impl AnotherQueryHandler {
        pub fn new() -> AnotherQueryHandler {
            AnotherQueryHandler {}
        }
    }

    impl QueryHandlerInBus for AnotherQueryHandler {
        fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Vec<Box<dyn Response>> {
            self.handle_generic_query(query)
        }

        fn get_associated_query_from_bus(&self) -> TypeId {
            self.get_associated_query()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl QueryHandler<AnotherQuery> for AnotherQueryHandler {
        fn handle(&self, _query: &AnotherQuery) -> Vec<Box<dyn Response>> {
            let another_response = AnotherResponse {};
            vec![Box::new(another_response)]
        }
    }
}