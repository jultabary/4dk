#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::dddk::command::bus::command_dispatcher_with_box::CommandDispatcher;
    use crate::dddk::command::command::Command;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
    use crate::dddk::event::event::Event;

    struct ACommand { }
    impl Command for ACommand {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    struct ACommandHandler {
        has_been_called: bool
    }

    impl ACommandHandler {
        fn called(&mut self) {
            self.has_been_called = true;
        }

        fn has_been_called(&self) -> bool {
            return self.has_been_called;
        }
    }

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle<'a>(&mut self, _command: &'a ACommand) -> Vec<Box<dyn Event>> {
            self.called();
            return Vec::new();
        }
    }

    impl CommandHandleInBus for ACommandHandler {
        fn handle_from_bus<'a>(&mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
            return self.handle_command(command);
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            return self.get_associated_command();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn it_should_handle_correct_handler() {
        // Given
        let command_handler = ACommandHandler { has_been_called: false };
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
        command_handlers.push(Box::new(command_handler));
        let mut command_bus = CommandDispatcher::new(command_handlers);
        let a_command = ACommand { };

        // When
        command_bus.dispatch(&a_command);

        // Then
        let type_of_a_command = TypeId::of::<ACommand>();
        let command_handler_borrow = command_bus.get_command_handler_by_its_command(type_of_a_command);
        if command_handler_borrow.is_none() {
            panic!("Test fails because command_bus has not found the desired handler");
        }
        let a_command_handler_borrow = command_handler_borrow.unwrap().as_any().downcast_ref::<ACommandHandler>();
        if a_command_handler_borrow.is_none() {
            panic!("Test fails when try to downcast variable command_handler_borrow");
        }
        assert_eq!(a_command_handler_borrow.unwrap().has_been_called(), true);
    }
}
