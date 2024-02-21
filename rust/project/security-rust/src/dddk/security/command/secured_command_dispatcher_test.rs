#[cfg(test)]
pub mod test {
    use std::any::TypeId;
    use std::rc::Rc;
    use dddk_core::dddk::command::command_bus::CommandBus;
    use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::security::authorized_strategy_impl::no_security_strategy::NoSecurityStrategy;
    use crate::dddk::security::authorized_strategy_impl::role_based_strategy::RoleBasedStrategy;
    use crate::dddk::security::command::secured_command::SecuredCommand;
    use crate::dddk::security::command::secured_command_dispatcher::SecuredCommandDispatcher;
    use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
    use crate::dddk::security::errors::{Forbidden, TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand};
    use crate::dddk::security::test_tools::fake_role_repository::fake_role_repository::get_fake_repository_with_filled_with_roles;
    use crate::dddk::security::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::{get_a_role, get_another_role};
    use crate::dddk::security::test_tools::some_secured_command_for_test::some_secured_command_for_test::{ACommand, AnotherCommand, get_a_command_secured};
    use crate::dddk::security::test_tools::some_secured_command_handler_for_test::some_secured_command_handler_for_test::{ACommandHandler, AnotherCommandHandler, get_a_command_handler_secured};

    #[test]
    #[should_panic]
    fn it_should_panic_when_register_two_command_handler_for_a_same_command() {
        // Given
        let a_command_handler_1 = ACommandHandler::new();
        let a_command_handler_2 = ACommandHandler::new();

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_1));
        command_handlers.push(Box::new(a_command_handler_2));

        // When
        SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );

        // Then
        // should panic
    }

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
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );

        // Then
        assert_eq!(true, secured_command_dispatcher.is_command_handler_restricted(TypeId::of::<ACommand>()));
        assert_eq!(false, secured_command_dispatcher.is_command_handler_restricted(TypeId::of::<AnotherCommand>()));
    }

    #[test]
    fn it_should_return_command_handler_associated_to_its_command_when_calling_getter() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();
        let another_command_handler = AnotherCommandHandler::new();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));
        command_handlers.push(Box::new(another_command_handler));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        // When
        let a_command_handler_opt = secured_command_dispatcher
            .get_command_handler_associated_to_command(TypeId::of::<ACommand>());
        let another_command_handler_opt = secured_command_dispatcher
            .get_command_handler_associated_to_command(TypeId::of::<AnotherCommand>());

        // Then
        assert_eq!(true, a_command_handler_opt.is_some());
        let secured_command_handler_opt = a_command_handler_opt.
            unwrap().as_any().downcast_ref::<SecuredCommandHandler>();
        assert_eq!(true, secured_command_handler_opt.is_some());
        let another_command_handler =
            another_command_handler_opt.unwrap().as_any().downcast_ref::<AnotherCommandHandler>();
        assert_eq!(true, another_command_handler.is_some());
    }

    #[test]
    fn it_should_not_dispatch_unsecured_command_to_a_secured_command_handler() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let unsecured_command = ACommand::new();

        // When
        let events = secured_command_dispatcher.dispatch(&unsecured_command, None);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        let error_opt = error.downcast_ref::<TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand>();
        assert_eq!(true, error_opt.is_some());
    }

    #[test]
    fn it_should_dispatch_secured_command_to_its_secured_command_handler() {
        // Given
        let a_command_handler_secured = get_a_command_handler_secured();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let secured_command = get_a_command_secured(vec![String::from("a_role")]);

        // When
        let events = secured_command_dispatcher.dispatch(&secured_command, None);

        // Then
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<AnEvent>().is_some());
    }

    #[test]
    fn it_should_dispatch_command_to_its_command_handler() {
        // Given
        let another_command_handler = AnotherCommandHandler::new();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(another_command_handler));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let command = AnotherCommand::new();

        // When
        let events = secured_command_dispatcher.dispatch(&command, None);

        // Then
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<AnotherEvent>().is_some());
    }

    #[test]
    fn it_should_not_dispatch_command_when_its_roles_has_not_correct_permissions() {
        // Given
        let not_enough_privilege_role = get_another_role().get_name().clone();
        let repository = get_fake_repository_with_filled_with_roles();
        let a_command_handler_secured = get_a_command_handler_secured();
        let another_command_handler = AnotherCommandHandler::new();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));
        command_handlers.push(Box::new(another_command_handler));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(RoleBasedStrategy::new(repository)),
        );
        let command = SecuredCommand::new(Box::new(ACommand::new()), vec![not_enough_privilege_role]);

        // When
        let events = secured_command_dispatcher.dispatch(&command, None);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        let error_opt = error.downcast_ref::<Forbidden>();
        assert_eq!(true, error_opt.is_some());
    }

    #[test]
    fn it_should_not_dispatch_command_when_its_roles_has_correct_permissions() {
        // Given
        let user_role = get_a_role().get_name().clone();
        let repository = get_fake_repository_with_filled_with_roles();
        let a_command_handler_secured = get_a_command_handler_secured();
        let another_command_handler = AnotherCommandHandler::new();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler_secured));
        command_handlers.push(Box::new(another_command_handler));

        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            Rc::new(RoleBasedStrategy::new(repository)),
        );
        let command = SecuredCommand::new(Box::new(ACommand::new()), vec![user_role]);

        // When
        let events = secured_command_dispatcher.dispatch(&command, None);

        // Then
        let events = events.unwrap();
        assert_eq!(1, events.len());
    }
}
