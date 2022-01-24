#[cfg(test)]
pub mod command_logging_middleware_test {
    use log::Log;
    use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
    use crate::dddk::command::bus_impl::command_logging_middleware::CommandLoggingMiddleware;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::errors::NoCommandHandlerRegisterForGivenCommand;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::{ACommand, AnotherCommand};
    use crate::dddk::test_tools::some_command_handler_for_test::some_command_handler_for_test::ACommandHandler;
    use crate::dddk::test_tools::some_logger_for_test::some_logger_for_test::MockLogger;


    pub fn it_should_log_command_handling_with_produced_events(logger: &MockLogger) {
        // Given
        let a_command_handler = ACommandHandler::new();
        let command_dispatcher = CommandDispatcher::new(vec![Box::new(a_command_handler)]);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let a_command = ACommand {};

        // When
        let events = command_logging_middleware.dispatch(&a_command);

        // Then
        assert_eq!(true, events.is_ok());
        assert_eq!(1, events.unwrap().len());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching a command [ACommand] [ACommand].".to_string();
        let second_log = "INFO_Command[ACommand] [ACommand] has been handled and has produced [1] events [AnEvent ].".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

    pub fn it_should_log_command_handling_with_error_returned_when_an_error_occurred(logger: &MockLogger) {
        // Given
        let a_command_handler = ACommandHandler::new();
        let command_dispatcher = CommandDispatcher::new(vec![Box::new(a_command_handler)]);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let a_command = AnotherCommand {};

        // When
        let error = command_logging_middleware.dispatch(&a_command);

        // Then
        assert_eq!(true, error.is_err());
        assert_eq!(true, error.err().unwrap().downcast_ref::<NoCommandHandlerRegisterForGivenCommand>().is_some());
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Dispatching a command [AnotherCommand] [AnotherCommand].".to_string();
        let second_log = "ERROR_An error has occurred when dispatching command [AnotherCommand] [AnotherCommand]: No CommandHandler is registered for given command !".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }

}