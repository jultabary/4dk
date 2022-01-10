#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use better_any::{Tid, TidAble};
    use crate::dddk::command::bus::command_dispatcher_with_ref::CommandDispatcher;
    use crate::dddk::command::command::Command;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
    use crate::dddk::event::event::Event;

    static mut HAS_BEEN_CALLED: bool = false;

    struct ACommand { }
    impl Command for ACommand {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Tid)]
    struct ACommandHandler {
    }

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle<'a>(&self, _command: &'a ACommand) -> Vec<Box<dyn Event>> {
            unsafe {
                HAS_BEEN_CALLED = true;
            }
            return Vec::new();
        }
    }

    impl CommandHandleInBus for ACommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
            return self.handle_command(command);
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            return self.get_associated_command();
        }

        fn as_tid(&self) -> &dyn Tid {
            self
        }
    }

    #[test]
    fn it_should_handle_correct_handler() {
        // Given
        let mut command_handler = ACommandHandler { };
        let mut command_handlers = Vec::new() as Vec<&mut dyn CommandHandleInBus>;
        command_handlers.push(&mut command_handler);
        let command_bus = CommandDispatcher::new(command_handlers);
        let a_command = ACommand { };

        // When
        command_bus.dispatch(&a_command);

        // Then
        unsafe {
            assert_eq!(HAS_BEEN_CALLED, true);
        }
    }

    #[test]
    fn it_should_create_command_bus_and_move_command_handler_in_it() {
        // Given
        let mut command_handler = ACommandHandler { };
        let mut command_handlers = Vec::new() as Vec<&mut dyn CommandHandleInBus>;
        command_handlers.push(&mut command_handler);

        // When
        CommandDispatcher::new(command_handlers);

    }
}
