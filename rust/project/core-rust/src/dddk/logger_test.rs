#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use log::LevelFilter;
    use crate::dddk::command::bus_impl::command_logging_middleware_test::command_logging_middleware_test::{it_should_log_command_handling_with_error_returned_when_an_error_occurred, it_should_log_command_handling_with_produced_events};
    use crate::dddk::event::event_handler_logger_test::event_handler_logger_test::it_should_log_event_handling_when_there_is_an_handler_for_event;
    use crate::dddk::query::bus_impl::query_logging_middleware_test::query_logging_middleware_test::{it_should_log_query_handling_with_error_returned_when_an_error_occurred, it_should_log_query_handling_with_produced_responses};
    use crate::dddk::test_tools::some_logger_for_test::some_logger_for_test::MockLogger;

    pub static LOGGER: MockLogger = MockLogger { messages: RefCell::new(Vec::new()) };

    #[test]
    fn launch_logger_tests_in_a_sequential_way() {
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
        it_should_log_command_handling_with_produced_events(&LOGGER);
        it_should_log_command_handling_with_error_returned_when_an_error_occurred(&LOGGER);
        it_should_log_query_handling_with_produced_responses(&LOGGER);
        it_should_log_query_handling_with_error_returned_when_an_error_occurred(&LOGGER);
        it_should_log_event_handling_when_there_is_an_handler_for_event(&LOGGER);
    }

}