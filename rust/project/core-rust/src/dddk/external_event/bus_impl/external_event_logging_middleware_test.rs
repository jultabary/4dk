#[cfg(test)]
pub mod external_event_logging_middleware_test {
    use log::Log;
    use crate::dddk::errors::NoPolicyHandlerRegisterForGivenExternalEvent;
    use crate::dddk::external_event::bus_impl::external_event_dispatcher::ExternalEventDispatcher;
    use crate::dddk::external_event::bus_impl::external_event_logging_middleware::ExternalEventLoggingMiddleware;
    use crate::dddk::external_event::external_event_bus::ExternalEventBus;
    use crate::dddk::test_tools::some_external_event_for_test::some_external_event_for_test::{AnExternalEvent, AnotherExternalEvent};
    use crate::dddk::test_tools::some_logger_for_test::some_logger_for_test::MockLogger;
    use crate::dddk::test_tools::some_policy_handler_for_test::some_policy_handler_for_test::APolicyHandler;

    pub fn it_should_log_external_event_handling_with_produced_commands(logger: &MockLogger) {
        // Given
        let a_external_event_handler = APolicyHandler::new();
        let external_event_dispatcher = ExternalEventDispatcher::new(vec![Box::new(a_external_event_handler)]);
        let external_event_logging_middleware = ExternalEventLoggingMiddleware::new(Box::new(external_event_dispatcher));

        let a_external_event = AnExternalEvent {};

        // When
        let commands = external_event_logging_middleware.dispatch(&a_external_event);

        // Then
        assert_eq!(true, commands.is_ok());
        assert_eq!(1, commands.unwrap().len());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching an external event [AnExternalEvent] [AnExternalEvent].".to_string();
        let second_log = "INFO_ExternalEvent[AnExternalEvent] [AnExternalEvent] has been handled and has produced [1] commands [ACommand ].".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

    pub fn it_should_log_external_event_handling_with_error_returned_when_an_error_occurred(logger: &MockLogger) {
        // Given
        let a_external_event_handler = APolicyHandler::new();
        let external_event_dispatcher = ExternalEventDispatcher::new(vec![Box::new(a_external_event_handler)]);
        let external_event_logging_middleware = ExternalEventLoggingMiddleware::new(Box::new(external_event_dispatcher));

        let a_external_event = AnotherExternalEvent {};

        // When
        let error = external_event_logging_middleware.dispatch(&a_external_event);

        // Then
        assert_eq!(true, error.is_err());
        assert_eq!(true, error.err().unwrap().downcast_ref::<NoPolicyHandlerRegisterForGivenExternalEvent>().is_some());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching an external event [AnotherExternalEvent] [AnotherExternalEvent].".to_string();
        let second_log = "ERROR_An error has occurred when dispatching external event [AnotherExternalEvent] [AnotherExternalEvent]: No PolicyHandler is registered for given external event !".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

}