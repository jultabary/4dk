#[cfg(test)]
pub mod some_secured_command_handler_for_test {
    use std::sync::Arc;
    use dddk_core::dddk::aliases::Events;
    use dddk_core::dddk::bus::Bus;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use dddk_core::dddk::event::event::Event;
    use dddk_macro::CommandHandlerInBus;
    use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
    use crate::dddk::security::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_permission_1;
    use crate::dddk::security::test_tools::some_secured_command_for_test::some_secured_command_for_test::{ACommand, AnotherCommand};

    #[derive(CommandHandlerInBus)]
    pub struct ACommandHandler {}

    impl ACommandHandler {
        pub fn new() -> ACommandHandler {
            ACommandHandler {}
        }
    }

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle(&self, _command: &ACommand, _bus_opt: Option<&dyn Bus>) -> Events {
            let event = Arc::new(AnEvent::new()) as Arc<dyn Event>;
            Ok(vec![event])
        }
    }

    pub fn get_a_command_handler_secured() -> SecuredCommandHandler {
        SecuredCommandHandler::new(
            Box::new(ACommandHandler::new()),
            get_permission_1(),
        )
    }

    #[derive(CommandHandlerInBus)]
    pub struct AnotherCommandHandler {}

    impl AnotherCommandHandler {
        pub fn new() -> AnotherCommandHandler {
            AnotherCommandHandler {}
        }
    }

    impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
        fn handle(&self, _command: &AnotherCommand, _bus_opt: Option<&dyn Bus>) -> Events {
            let event = Arc::new(AnotherEvent::new()) as Arc<dyn Event>;
            Ok(vec![event])
        }
    }

}