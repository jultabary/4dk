#[cfg(test)]
pub mod command_macro_tests {
    use std::any::{Any, TypeId};
    use std::fmt::{Debug, Formatter};
    use std::sync::Arc;
    use dddk_core::dddk::aliases::Events;
    use dddk_core::dddk::bus::Bus;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
    use dddk_core::dddk::command::command::Command;
    use dddk_macro::{Command, CommandHandlerInBus};
    use crate::macro_tests::event_macro_test::event_macro_tests::AnEvent;

    #[derive(Command)]
    pub struct ACommand {}

    impl Debug for ACommand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ACommand")
        }
    }

    #[derive(CommandHandlerInBus)]
    struct ACommandHandler {}

    impl CommandHandler<ACommand> for ACommandHandler {
        fn handle(&self, _command: &ACommand, _bus_opt: Option<& dyn Bus>) -> Events {
            Ok(vec![Arc::new(AnEvent {})])
        }
    }

    #[test]
    fn it_should_impl_default_behaviour_of_command_trait_when_using_derive_macro() {
        // Given
        let a_command = ACommand {};

        // When
        let command_name = a_command.get_command_name();
        let as_any: &dyn Any = a_command.as_any();

        // Then
        assert_eq!("ACommand".to_string(), command_name);
        assert_eq!(true, as_any.downcast_ref::<ACommand>().is_some());
    }

    #[test]
    fn it_should_impl_default_behaviour_of_command_handler_in_bus_trait_when_using_derive_macro() {
        // Given
        let a_command = ACommand {};
        let a_command_handler = ACommandHandler {};

        // When
        let events: Events = a_command_handler.handle_from_bus(&a_command, None);
        let command_handler_name = a_command_handler.get_command_handler_name();
        let command_type_id = a_command_handler.get_associated_command_from_bus();

        // Then
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<AnEvent>().is_some());
        assert_eq!("ACommandHandler".to_string(), command_handler_name);
        assert_eq!(TypeId::of::<ACommand>(), command_type_id);
    }
}
