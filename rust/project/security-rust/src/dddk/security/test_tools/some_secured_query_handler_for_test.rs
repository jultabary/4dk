#[cfg(test)]
pub mod some_secured_query_handler_for_test {
    use dddk_core::dddk::aliases::ResponseFromHandler;
    use dddk_core::dddk::query::query_handler::QueryHandler;
    use dddk_core::dddk::query::response::Response;
    use dddk_macro::QueryHandlerInBus;
    use crate::dddk::security::query::secured_query_handler::SecuredQueryHandler;
    use crate::dddk::security::test_tools::some_response_for_test::some_response_for_test::{AnotherResponse, AResponse};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_permission_1;
    use crate::dddk::security::test_tools::some_secured_query_for_test::some_secured_query_for_test::{AnotherQuery, AQuery};

    #[derive(QueryHandlerInBus)]
    pub struct AQueryHandler {}

    impl AQueryHandler {
        pub fn new() -> AQueryHandler {
            AQueryHandler {}
        }
    }

    impl QueryHandler<AQuery> for AQueryHandler {
        fn handle(&self, _command: &AQuery) -> ResponseFromHandler {
            let response = Box::new(AResponse::new()) as Box<dyn Response>;
            Ok(response)
        }
    }

    pub fn get_a_query_handler_secured() -> SecuredQueryHandler {
        SecuredQueryHandler::new(
            Box::new(AQueryHandler::new()),
            get_permission_1(),
        )
    }

    #[derive(QueryHandlerInBus)]
    pub struct AnotherQueryHandler {}

    impl AnotherQueryHandler {
        pub fn new() -> AnotherQueryHandler {
            AnotherQueryHandler {}
        }
    }

    impl QueryHandler<AnotherQuery> for AnotherQueryHandler {
        fn handle(&self, _query: &AnotherQuery) -> ResponseFromHandler {
            let response = Box::new(AnotherResponse::new()) as Box<dyn Response>;
            Ok(response)
        }
    }
}