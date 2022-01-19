#[cfg(test)]
mod tests {
    use crate::dddk::errors::NoQueryHandlerRegisterForGivenQuery;
    use crate::dddk::query::bus_impl::query_dispatcher::QueryDispatcher;
    use crate::dddk::query::query_bus::QueryBus;
    use crate::dddk::query::query_handler::QueryHandlerInBus;
    use crate::dddk::test_tools::some_query_for_test::some_query_for_test::{AnotherQuery, AQuery};
    use crate::dddk::test_tools::some_query_handler_for_test::some_query_handler_for_test::{AnotherQueryHandler, AQueryHandler};
    use crate::dddk::test_tools::some_response_for_test::some_response_for_test::AResponse;

    #[test]
    #[should_panic]
    fn it_should_panic_when_register_two_command_handler_for_a_same_command() {
        // Given
        let a_query_handler_1 = AQueryHandler::new();
        let a_query_handler_2 = AQueryHandler::new();

        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_1));
        query_handlers.push(Box::new(a_query_handler_2));

        // When
        QueryDispatcher::new(query_handlers);

        // Then
        // should panic
    }

    #[test]
    fn it_should_return_an_error_when_dispatch_a_query_with_no_register_handler() {
        // Given
        let a_query_handler = AQueryHandler::new();

        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler));

        let query_bus = QueryDispatcher::new(query_handlers);
        let another_query = AnotherQuery {};

        // When
        let responses = query_bus.dispatch(&another_query);

        // Then
        assert_eq!(true, responses.is_err());
        assert_eq!(true, responses.err().unwrap().downcast_ref::<NoQueryHandlerRegisterForGivenQuery>().is_some());
    }


    #[test]
    fn it_should_be_handled_by_correct_handler_when_dispatch_query() {
        // Given
        let a_query_handler = AQueryHandler::new();
        let another_query_handler = AnotherQueryHandler::new();

        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler));
        query_handlers.push(Box::new(another_query_handler));

        let query_bus = QueryDispatcher::new(query_handlers);
        let a_query = AQuery {};

        // When
        let responses = query_bus.dispatch(&a_query);

        // Then
        assert_eq!(true, responses.is_ok());
        let responses = responses.unwrap();
        assert_eq!(1, responses.len());
        let response = responses.get(0).unwrap();
        let a_response = response.as_ref().as_any().downcast_ref::<AResponse>();
        assert_eq!(true, a_response.is_some());
    }
}