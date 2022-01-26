#[cfg(test)]
pub mod query_logging_middleware_test {
    use log::Log;
    use crate::dddk::errors::NoQueryHandlerRegisterForGivenQuery;
    use crate::dddk::query::bus_impl::query_dispatcher::QueryDispatcher;
    use crate::dddk::query::bus_impl::query_logging_middleware::QueryLoggingMiddleware;
    use crate::dddk::query::query_bus::QueryBus;
    use crate::dddk::test_tools::some_logger_for_test::some_logger_for_test::MockLogger;
    use crate::dddk::test_tools::some_query_for_test::some_query_for_test::{AnotherQuery, AQuery};
    use crate::dddk::test_tools::some_query_handler_for_test::some_query_handler_for_test::AQueryHandler;

    pub fn it_should_log_query_handling_with_produced_responses(logger: &MockLogger) {
        // Given
        let a_query_handler = AQueryHandler::new();
        let query_dispatcher = QueryDispatcher::new(vec![Box::new(a_query_handler)]);
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));

        let a_query = AQuery {};

        // When
        let response = query_logging_middleware.dispatch(&a_query);

        // Then
        assert_eq!(true, response.is_ok());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching a query [AQuery] [AQuery].".to_string();
        let second_log = "INFO_Query[AQuery] [AQuery] has been handled and has returned response [AResponse].".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

    pub fn it_should_log_query_handling_with_error_returned_when_an_error_occurred(logger: &MockLogger) {
        // Given
        let a_query_handler = AQueryHandler::new();
        let query_dispatcher = QueryDispatcher::new(vec![Box::new(a_query_handler)]);
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));

        let a_query = AnotherQuery {};

        // When
        let error = query_logging_middleware.dispatch(&a_query);

        // Then
        assert_eq!(true, error.is_err());
        assert_eq!(true, error.err().unwrap().downcast_ref::<NoQueryHandlerRegisterForGivenQuery>().is_some());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching a query [AnotherQuery] [AnotherQuery].".to_string();
        let second_log = "ERROR_An error has occurred when dispatching query [AnotherQuery] [AnotherQuery]: No QueryHandler is registered for given query !".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

}