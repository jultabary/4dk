#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use log::{Level, LevelFilter, Log, Metadata, Record};
    use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
    use crate::dddk::command::bus_impl::command_logging_middleware::CommandLoggingMiddleware;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::errors::NoCommandHandlerRegisterForGivenCommand;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::{ACommand, AnotherCommand};
    use crate::dddk::test_tools::some_command_handler_for_test::some_command_handler_for_test::ACommandHandler;

    pub struct FakeLogger {
        messages: RefCell<Vec<String>>,
    }

    impl Log for FakeLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut message = String::new();
                message.push_str(record.level().as_str());
                message.push_str("_");
                message.push_str(&*record.args().to_string());
                self.messages.borrow_mut().push(message);
            }
        }

        fn flush(&self) {
            self.messages.borrow_mut().clear();
        }
    }

    unsafe impl Sync for FakeLogger {}

    unsafe impl Send for FakeLogger {}

    static LOGGER: FakeLogger = FakeLogger { messages: RefCell::new(Vec::new()) };

    #[test]
    fn launch_logger_tests_in_a_sequential_way() {
        it_should_log_command_handling_with_produced_events();
        it_should_log_command_handling_with_error_returned_when_an_error_occurred();
    }

    fn it_should_log_command_handling_with_produced_events() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

        let a_command_handler = ACommandHandler::new();
        let command_dispatcher = CommandDispatcher::new(vec![Box::new(a_command_handler)]);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let a_command = ACommand {};

        // When
        let events = command_logging_middleware.dispatch(&a_command);

        // Then
        assert_eq!(true, events.is_ok());
        assert_eq!(1, events.unwrap().len());
        assert_eq!(2, LOGGER.messages.borrow().len());
        let first_log = "INFO_Dispatching a command [ACommand].".to_string();
        let second_log = "INFO_Command[ACommand] has been handled and has produced [1] events [AnEvent ].".to_string();
        assert_eq!(&first_log, LOGGER.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, LOGGER.messages.borrow().get(1).unwrap());
        LOGGER.flush();
    }

    fn it_should_log_command_handling_with_error_returned_when_an_error_occurred() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

        let a_command_handler = ACommandHandler::new();
        let command_dispatcher = CommandDispatcher::new(vec![Box::new(a_command_handler)]);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let a_command = AnotherCommand {};

        // When
        let error = command_logging_middleware.dispatch(&a_command);

        // Then
        assert_eq!(true, error.is_err());
        assert_eq!(true, error.err().unwrap().downcast_ref::<NoCommandHandlerRegisterForGivenCommand>().is_some());
        assert_eq!(2, LOGGER.messages.borrow().len());
        let first_log = "INFO_Dispatching a command [AnotherCommand].".to_string();
        let second_log = "ERROR_An error has occurred when dispatching command [AnotherCommand]: No CommandHandler is registered for given command !".to_string();
        assert_eq!(&first_log, LOGGER.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, LOGGER.messages.borrow().get(1).unwrap());
        LOGGER.flush();
    }

}