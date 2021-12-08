#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::dddk::command::bus::command_dispatcher::CommandDispatcher;
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

    impl <'a>CommandHandler<'a, ACommand> for ACommandHandler {
        fn handle(&mut self, _command: &ACommand) -> Vec<Box<dyn Event>> {
            self.called();
            return Vec::new();
        }
    }

    impl <'a>CommandHandleInBus<'a> for ACommandHandler {
        fn handle_from_bus(&mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
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
        let mut command_handler = ACommandHandler { has_been_called: false };
        let mut command_handlers = Vec::new() as Vec<&mut dyn CommandHandleInBus>;
        command_handlers.push(&mut command_handler);
        let mut command_bus = CommandDispatcher::new(command_handlers);
        let a_command = ACommand { };

        // When
        command_bus.dispatch(&a_command);

        // Then
        assert_eq!(command_handler.has_been_called(), true);
    }
}
