#[cfg(test)]
mod test {
    use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
    use crate::dddk::security::test_tools::some_secured_command_handler_for_test::some_secured_command_handler_for_test::{ACommandHandler, get_a_command_handler_secured};

    #[test]
    fn it_should_return_sub_command_handler_when_call_get_command_handler() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();

        // When
        let command_handler = a_command_handler_secured.get_command_handler();
        let a_command_handler_opt = command_handler.as_any().downcast_ref::<ACommandHandler>();

        // Then
        assert_eq!(true, a_command_handler_opt.is_some());
    }

    #[test]
    fn it_should_downcast_ref_secured_command_handler_when_receiving_it_as_command_handler() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();
        let command_handler = &a_command_handler_secured as &dyn CommandHandlerInBus;

        // When
        let a_secured_command_handler_opt = command_handler.as_any()
            .downcast_ref::<SecuredCommandHandler>();

        // Then
        assert_eq!(true, a_secured_command_handler_opt.is_some());
        let command_handler = a_secured_command_handler_opt.unwrap().get_command_handler();
        let a_command_handler = command_handler.as_any().downcast_ref::<ACommandHandler>();
        assert_eq!(true, a_command_handler.is_some());
    }
}