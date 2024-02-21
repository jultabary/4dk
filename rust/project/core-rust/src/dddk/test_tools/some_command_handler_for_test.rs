#[cfg(test)]
pub mod some_command_handler_for_test {
    use std::any::{Any, TypeId};
    use std::sync::Arc;
    use crate::dddk::aliases::Events;
    use crate::dddk::bus::Bus;
    use crate::dddk::command::command::Command;
    use crate::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};
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
        fn handle(&self, _command: &ACommand, _bus_opt: Option<& dyn Bus>) -> Events {
            Ok(vec![Arc::new(AnEvent::new(1))])
        }
    }

    impl CommandHandlerInBus for ACommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command, bus_opt: Option<& dyn Bus>) -> Events {
            self.handle_generic_command(command, bus_opt)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            self.get_associated_command()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_handler_name(&self) -> String {
            "ACommandHandler".to_string()
        }
    }

    pub struct AnotherCommandHandler {}

    impl CommandHandlerInBus for AnotherCommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command, bus_opt: Option<& dyn Bus>) -> Events {
            self.handle_generic_command(command, bus_opt)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            self.get_associated_command()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_handler_name(&self) -> String {
            "AnotherCommandHandler".to_string()
        }
    }

    impl AnotherCommandHandler {
        pub fn new() -> AnotherCommandHandler {
            AnotherCommandHandler {}
        }
    }

    impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
        fn handle(&self, _command: &AnotherCommand, _bus_opt: Option<& dyn Bus>) -> Events {
            Ok(vec![Arc::new(AnotherEvent::new(2))])
        }
    }
}
