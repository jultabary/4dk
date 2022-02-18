#[cfg(test)]
pub mod external_event_macro_tests {
    use std::any::{Any, TypeId};
    use dddk_core::dddk::aliases::Commands;
    use dddk_core::dddk::external_event::policy_handler::PolicyHandler;
    use dddk_core::dddk::external_event::external_event::ExternalEvent;
    use dddk_core::dddk::external_event::policy_handler::PolicyHandlerInBus;
    use crate::macro_tests::command_macro_test::command_macro_tests::ACommand;
    use dddk_macro::{ExternalEvent, PolicyHandlerInBus};

    #[derive(ExternalEvent, Debug)]
    pub struct AnExternalEvent {}

    #[derive(PolicyHandlerInBus)]
    pub struct APolicyHandler {}

    impl PolicyHandler<AnExternalEvent> for APolicyHandler {
        fn handle(&self, _external_event: &AnExternalEvent) -> Commands {
            Ok(vec![Box::new(ACommand {})])
        }
    }

    #[test]
    fn it_should_impl_default_behaviour_of_external_event_trait_when_using_derive_macro() {
        // Given
        let an_external_event = AnExternalEvent {};

        // When
        let external_event_name = an_external_event.get_external_event_name();
        let event: &dyn Any = an_external_event.as_any();

        // Then
        assert_eq!(external_event_name, "AnExternalEvent");
        assert_eq!(event.downcast_ref::<AnExternalEvent>().is_some(), true);
    }

    #[test]
    fn it_should_impl_default_behaviour_of_policy_handler_trait_when_using_derive_macro() {
        // Given
        let a_policy_handler = APolicyHandler {};
        let an_external_event = AnExternalEvent {};

        // When
        let associated_external_event = a_policy_handler.get_associated_external_event_from_bus();
        let policy_handler: &dyn Any = a_policy_handler.as_any();
        let policy_handler_name = a_policy_handler.get_policy_handler_name();
        let commands: Commands = a_policy_handler.handle_from_bus(&an_external_event);

        // Then
        assert_eq!(policy_handler_name, "APolicyHandler");
        assert_eq!(associated_external_event, TypeId::of::<AnExternalEvent>());
        assert_eq!(policy_handler.downcast_ref::<APolicyHandler>().is_some(), true);
        assert_eq!(commands.is_ok(), true);
        let commands = commands.unwrap();
        let command = commands.get(0).unwrap();
        assert_eq!(command.as_any().downcast_ref::<ACommand>().is_some(), true);
    }
}