#[cfg(test)]
mod tests {
    use crate::dddk::errors::ExternalEventIsNotAssociatedWithThisPolicyHandler;
    use crate::dddk::external_event::policy_handler::PolicyHandler;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::ACommand;
    use crate::dddk::test_tools::some_external_event_for_test::some_external_event_for_test::{AnExternalEvent, AnotherExternalEvent};
    use crate::dddk::test_tools::some_policy_handler_for_test::some_policy_handler_for_test::APolicyHandler;

    #[test]
    pub fn it_should_handle_external_event_when_external_event_is_associated_to_this_handler() {
        // Given
        let a_policy_handler = APolicyHandler::new();
        let an_external_event = AnExternalEvent { };

        // When
        let commands = a_policy_handler.handle_generic_external_event(&an_external_event);

        // Then
        assert_eq!(true, commands.is_ok());
        let commands = commands.unwrap();
        assert_eq!(1, commands.len());
        let command = commands.get(0).unwrap();
        let a_command = command.as_ref().as_any().downcast_ref::<ACommand>();
        assert_eq!(true, a_command.is_some());
    }

    #[test]
    fn it_should_not_handle_external_event_when_external_event_is_not_associated_to_this_handler() {
        // Given
        let a_external_event_handler = APolicyHandler::new();
        let another_external_event = AnotherExternalEvent { };

        // When
        let commands = a_external_event_handler.handle_generic_external_event(&another_external_event);

        // Then
        assert_eq!(true, commands.is_err());
        assert_eq!(true, commands.err().unwrap().downcast_ref::<ExternalEventIsNotAssociatedWithThisPolicyHandler>().is_some());
    }
}