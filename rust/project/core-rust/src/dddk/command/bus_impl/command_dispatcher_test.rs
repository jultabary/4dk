#[cfg(test)]
mod tests {
    use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::ACommand;
    use crate::dddk::test_tools::some_command_handler_for_test::some_command_handler_for_test::{ACommandHandler, AnotherCommandHandler};
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::AnEvent;

    #[test]
    fn it_should_be_handled_by_correct_handler_when_dispatch_command() {
        // Given
        let a_command_handler = ACommandHandler::new();
        let another_command_handler = AnotherCommandHandler::new();

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));
        command_handlers.push(Box::new(another_command_handler));

        let command_bus = CommandDispatcher::new(command_handlers);
        let a_command = ACommand {};

        // When
        let events = command_bus.dispatch(&a_command);

        // Then
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        let an_event = event.as_ref().as_any().downcast_ref::<AnEvent>();
        assert_eq!(true, an_event.is_some());
    }
}
