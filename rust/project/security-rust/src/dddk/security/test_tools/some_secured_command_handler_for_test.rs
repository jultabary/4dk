#[cfg(test)]
pub mod some_secured_command_handler_for_test {
    use std::any::{Any, TypeId};
    use std::sync::Arc;
    use dddk_core::dddk::command::command::Command;
    use dddk_core::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};
    use dddk_core::dddk::event::event::Event;
    use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
    use crate::dddk::security::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_permission_1;
    use crate::dddk::security::test_tools::some_secured_command_for_test::some_secured_command_for_test::{ACommand, AnotherCommand};

    pub struct ACommandHandler {}

    impl ACommandHandler {
        pub fn new() -> ACommandHandler {
            ACommandHandler {}
        }
    }

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle(&self, _command: &ACommand) -> Vec<Arc<dyn Event>> {
            let event = Arc::new(AnEvent::new()) as Arc<dyn Event>;
            vec![event]
        }
    }

    impl CommandHandlerInBus for ACommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Arc<dyn Event>> {
            self.handle_generic_command(command)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            TypeId::of::<ACommand>()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub fn get_a_command_handler_secured() -> SecuredCommandHandler {
        SecuredCommandHandler::new(
            Box::new(ACommandHandler::new()),
            get_permission_1(),
        )
    }

    pub struct AnotherCommandHandler {}

    impl AnotherCommandHandler {
        pub fn new() -> AnotherCommandHandler {
            AnotherCommandHandler {}
        }
    }

    impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
        fn handle(&self, _command: &AnotherCommand) -> Vec<Arc<dyn Event>> {
            let event = Arc::new(AnotherEvent::new()) as Arc<dyn Event>;
            vec![event]
        }
    }

    impl CommandHandlerInBus for AnotherCommandHandler {
        fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Arc<dyn Event>> {
            self.handle_generic_command(command)
        }

        fn get_associated_command_from_bus(&self) -> TypeId {
            TypeId::of::<AnotherCommand>()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

}