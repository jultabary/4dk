#[cfg(test)]
pub mod some_secured_query_handler_for_test {
    use std::any::{Any, TypeId};
    use dddk_core::dddk::aliases::Responses;
    use dddk_core::dddk::query::query::Query;
    use dddk_core::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
    use dddk_core::dddk::query::response::Response;
    use crate::dddk::security::query::secured_query_handler::SecuredQueryHandler;
    use crate::dddk::security::test_tools::some_response_for_test::some_response_for_test::{AnotherResponse, AResponse};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_permission_1;
    use crate::dddk::security::test_tools::some_secured_query_for_test::some_secured_query_for_test::{AnotherQuery, AQuery};

    pub struct AQueryHandler {}

    impl AQueryHandler {
        pub fn new() -> AQueryHandler {
            AQueryHandler {}
        }
    }

    impl QueryHandler<AQuery> for AQueryHandler {
        fn handle(&self, _command: &AQuery) -> Responses {
            let response = Box::new(AResponse::new()) as Box<dyn Response>;
            Ok(vec![response])
        }
    }

    impl QueryHandlerInBus for AQueryHandler {
        fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Responses {
            self.handle_generic_query(query)
        }

        fn get_associated_query_from_bus(&self) -> TypeId {
            TypeId::of::<AQuery>()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub fn get_a_query_handler_secured() -> SecuredQueryHandler {
        SecuredQueryHandler::new(
            Box::new(AQueryHandler::new()),
            get_permission_1(),
        )
    }

    pub struct AnotherQueryHandler {}

    impl AnotherQueryHandler {
        pub fn new() -> AnotherQueryHandler {
            AnotherQueryHandler {}
        }
    }

    impl QueryHandler<AnotherQuery> for AnotherQueryHandler {
        fn handle(&self, _query: &AnotherQuery) -> Responses {
            let response = Box::new(AnotherResponse::new()) as Box<dyn Response>;
            Ok(vec![response])
        }
    }

    impl QueryHandlerInBus for AnotherQueryHandler {
        fn handle_from_bus<'a>(&self, query: &'a dyn Query) -> Responses {
            self.handle_generic_query(query)
        }

        fn get_associated_query_from_bus(&self) -> TypeId {
            TypeId::of::<AnotherQuery>()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}