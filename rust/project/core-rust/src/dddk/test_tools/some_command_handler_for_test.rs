#[cfg(test)]
pub mod some_command_handler_for_test {
    use std::any::{Any, TypeId};
    use std::sync::Arc;
    use crate::dddk::command::command::Command;
    use crate::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};
    use crate::dddk::event::event::Event;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::{ACommand, AnotherCommand};
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};

    pub struct ACommandHandler {
        event_id_returned: i32,
    }

    impl ACommandHandler {
        pub fn new() -> ACommandHandler {
            ACommandHandler { event_id_returned: 1 }
        }
        pub fn get_event_id_returned(&self) -> i32 {
            self.event_id_returned
        }
    }

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle(&self, _command: &ACommand) -> Vec<Arc<dyn Event>> {
            vec![Arc::new(AnEvent::new(1))]
        }
    }

    impl CommandHandlerInBus for ACommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Arc<dyn Event>> {
            self.handle_generic_command(command)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            self.get_associated_command()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherCommandHandler {}

    impl CommandHandlerInBus for AnotherCommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Arc<dyn Event>> {
            self.handle_generic_command(command)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            self.get_associated_command()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl AnotherCommandHandler {
        pub fn new() -> AnotherCommandHandler {
            AnotherCommandHandler {}
        }
    }

    impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
        fn handle(&self, _command: &AnotherCommand) -> Vec<Arc<dyn Event>> {
            vec![Arc::new(AnotherEvent::new(2))]
        }
    }
}
