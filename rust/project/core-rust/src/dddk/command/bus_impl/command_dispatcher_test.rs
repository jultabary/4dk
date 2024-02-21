#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::errors::NoCommandHandlerRegisterForGivenCommand;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::{ACommand, AnotherCommand};
    use crate::dddk::test_tools::some_command_handler_for_test::some_command_handler_for_test::{ACommandHandler, AnotherCommandHandler};
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::AnEvent;


    #[test]
    #[should_panic]
    fn it_should_panic_when_register_two_command_handler_for_a_same_command() {
        // Given
        let a_command_handler_1 = ACommandHandler::new();
        let a_command_handler_2 = ACommandHandler::new();

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_1));
        command_handlers.push(Box::new(a_command_handler_2));

        // When
        CommandDispatcher::new(command_handlers);

        // Then
        // should panic
    }

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
        let events = command_bus.dispatch(&a_command, None);

        // Then
        assert_eq!(true, events.is_ok());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        let an_event = event.as_ref().as_any().downcast_ref::<AnEvent>();
        assert_eq!(true, an_event.is_some());
    }

    #[test]
    fn it_should_return_an_error_when_dispatch_a_command_with_no_register_handler() {
        // Given
        let a_command_handler = ACommandHandler::new();

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));

        let command_bus = CommandDispatcher::new(command_handlers);
        let another_command = AnotherCommand {};

        // When
        let responses = command_bus.dispatch(&another_command, None);

        // Then
        assert_eq!(true, responses.is_err());
        assert_eq!(true, responses.err().unwrap().downcast_ref::<NoCommandHandlerRegisterForGivenCommand>().is_some());
    }

    #[test]
    fn it_should_return_command_handler_which_is_associated_to_the_given_command() {
        // Given
        let a_command_handler = ACommandHandler::new();
        let another_command_handler = AnotherCommandHandler::new();

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));
        command_handlers.push(Box::new(another_command_handler));

        let command_bus = CommandDispatcher::new(command_handlers);

        // When
        let command_handler_opt = command_bus.get_command_handler_by_its_command(TypeId::of::<AnotherCommand>());

        // Then
        assert_eq!(true, command_handler_opt.is_some());
        let another_command_handler = command_handler_opt.unwrap()
            .as_any().downcast_ref::<AnotherCommandHandler>();
        assert_eq!(true, another_command_handler.is_some());
    }
}
