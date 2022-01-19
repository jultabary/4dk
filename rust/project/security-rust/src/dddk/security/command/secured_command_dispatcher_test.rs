#[cfg(test)]
pub mod test {
    use std::any::TypeId;
    use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::security::authorized_strategy_impl::no_security_strategy::NoSecurityStrategy;
    use crate::dddk::security::command::secured_command_dispatcher::SecuredCommandDispatcher;
    use crate::dddk::security::test_tools::fake_role_repository::fake_role_repository::get_fake_repository_with_filled_with_roles;
    use crate::dddk::security::test_tools::some_secured_command_for_test::some_secured_command_for_test::{ACommand, AnotherCommand};
    use crate::dddk::security::test_tools::some_secured_command_handler_for_test::some_secured_command_handler_for_test::{AnotherCommandHandler, get_a_command_handler_secured};

    #[test]
    fn it_should_return_is_restricted_when_asking_if_a_secured_command_handler_is_restricted() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();
        let another_command_handler = AnotherCommandHandler::new();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));
        command_handlers.push(Box::new(another_command_handler));

        // When
        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Box::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );

        // Then
        assert_eq!(true, secured_command_dispatcher.is_command_handler_restricted(TypeId::of::<ACommand>()));
        assert_eq!(false, secured_command_dispatcher.is_command_handler_restricted(TypeId::of::<AnotherCommand>()));
    }
}
