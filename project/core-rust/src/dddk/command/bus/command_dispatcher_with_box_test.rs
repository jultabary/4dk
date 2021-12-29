#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::dddk::command::bus::command_dispatcher_with_box::CommandDispatcher;
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

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn it_should_handle_correct_handler() {
        // Given
        let command_handler = ACommandHandler { };
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
        command_handlers.push(Box::new(command_handler));
        let command_bus = CommandDispatcher::new(command_handlers);
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
        unsafe {
            assert_eq!(HAS_BEEN_CALLED, true);
        }
    }
}
