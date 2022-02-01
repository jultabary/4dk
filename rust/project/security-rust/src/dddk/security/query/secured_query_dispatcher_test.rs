#[cfg(test)]
pub mod test {
    use std::any::TypeId;
    use std::rc::Rc;
    use dddk_core::dddk::query::query_bus::QueryBus;
    use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
    use crate::dddk::security::authorized_strategy_impl::no_security_strategy::NoSecurityStrategy;
    use crate::dddk::security::authorized_strategy_impl::role_based_strategy::RoleBasedStrategy;
    use crate::dddk::security::errors::{Forbidden, TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery};
    use crate::dddk::security::query::secured_query::SecuredQuery;
    use crate::dddk::security::query::secured_query_dispatcher::SecuredQueryDispatcher;
    use crate::dddk::security::query::secured_query_handler::SecuredQueryHandler;
    use crate::dddk::security::test_tools::fake_role_repository::fake_role_repository::get_fake_repository_with_filled_with_roles;
    use crate::dddk::security::test_tools::some_response_for_test::some_response_for_test::{AnotherResponse, AResponse};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::{get_a_role, get_another_role};
    use crate::dddk::security::test_tools::some_secured_query_for_test::some_secured_query_for_test::{AnotherQuery, AQuery, get_a_query_secured};
    use crate::dddk::security::test_tools::some_secured_query_handler_for_test::some_secured_query_handler_for_test::{AnotherQueryHandler, AQueryHandler, get_a_query_handler_secured};

    #[test]
    #[should_panic]
    fn it_should_panic_when_register_two_query_handler_for_a_same_query() {
        // Given
        let a_query_handler_1 = AQueryHandler::new();
        let a_query_handler_2 = AQueryHandler::new();

        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_1));
        query_handlers.push(Box::new(a_query_handler_2));

        // When
        SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );

        // Then
        // should panic
    }

    #[test]
    fn it_should_return_is_restricted_when_asking_if_a_secured_query_handler_is_restricted() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();
        let another_query_handler = AnotherQueryHandler::new();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));
        query_handlers.push(Box::new(another_query_handler));

        // When
        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );

        // Then
        assert_eq!(true, secured_query_dispatcher.is_query_handler_restricted(TypeId::of::<AQuery>()));
        assert_eq!(false, secured_query_dispatcher.is_query_handler_restricted(TypeId::of::<AnotherQuery>()));
    }

    #[test]
    fn it_should_return_query_handler_associated_to_its_query_when_calling_getter() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();
        let another_query_handler = AnotherQueryHandler::new();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));
        query_handlers.push(Box::new(another_query_handler));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        // When
        let a_query_handler_opt = secured_query_dispatcher
            .get_query_handler_associated_to_query(TypeId::of::<AQuery>());
        let another_query_handler_opt = secured_query_dispatcher
            .get_query_handler_associated_to_query(TypeId::of::<AnotherQuery>());

        // Then
        assert_eq!(true, a_query_handler_opt.is_some());
        let secured_query_handler_opt = a_query_handler_opt.
            unwrap().as_any().downcast_ref::<SecuredQueryHandler>();
        assert_eq!(true, secured_query_handler_opt.is_some());
        let another_query_handler =
            another_query_handler_opt.unwrap().as_any().downcast_ref::<AnotherQueryHandler>();
        assert_eq!(true, another_query_handler.is_some());
    }

    #[test]
    fn it_should_not_dispatch_unsecured_query_to_a_secured_query_handler() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let unsecured_query = AQuery::new();

        // When
        let events = secured_query_dispatcher.dispatch(&unsecured_query);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        let error_opt = error.downcast_ref::<TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery>();
        assert_eq!(true, error_opt.is_some());
    }

    #[test]
    fn it_should_dispatch_secured_query_to_its_secured_query_handler() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let secured_query = get_a_query_secured(vec![String::from("a_role")]);

        // When
        let responses = secured_query_dispatcher.dispatch(&secured_query);

        // Then
        let mut responses = responses.unwrap();
        assert_eq!(true, responses.as_any().downcast_ref::<AResponse>().is_some());
    }

    #[test]
    fn it_should_dispatch_query_to_its_query_handler() {
        // Given
        let another_query_handler = AnotherQueryHandler::new();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(another_query_handler));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(NoSecurityStrategy::new(get_fake_repository_with_filled_with_roles())),
        );
        let query = AnotherQuery::new();

        // When
        let responses = secured_query_dispatcher.dispatch(&query);

        // Then
        let mut responses = responses.unwrap();
        assert_eq!(true, responses.as_any().downcast_ref::<AnotherResponse>().is_some());
    }

    #[test]
    fn it_should_not_dispatch_query_when_its_roles_has_not_correct_permissions() {
        // Given
        let not_enough_privilege_role = get_another_role().get_name().clone();
        let repository = get_fake_repository_with_filled_with_roles();
        let a_query_handler_secured = get_a_query_handler_secured();
        let another_query_handler = AnotherQueryHandler::new();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));
        query_handlers.push(Box::new(another_query_handler));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(RoleBasedStrategy::new(repository)),
        );
        let query = SecuredQuery::new(Box::new(AQuery::new()), vec![not_enough_privilege_role]);

        // When
        let events = secured_query_dispatcher.dispatch(&query);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        let error_opt = error.downcast_ref::<Forbidden>();
        assert_eq!(true, error_opt.is_some());
    }

    #[test]
    fn it_should_not_dispatch_query_when_its_roles_has_correct_permissions() {
        // Given
        let user_role = get_a_role().get_name().clone();
        let repository = get_fake_repository_with_filled_with_roles();
        let a_query_handler_secured = get_a_query_handler_secured();
        let another_query_handler = AnotherQueryHandler::new();
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler_secured));
        query_handlers.push(Box::new(another_query_handler));

        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            Rc::new(RoleBasedStrategy::new(repository)),
        );
        let query = SecuredQuery::new(Box::new(AQuery::new()), vec![user_role]);

        // When
        let response = secured_query_dispatcher.dispatch(&query);

        // Then
        let mut response = response.unwrap();
        assert_eq!(true, response.as_any().downcast_ref::<AResponse>().is_some());
    }
}
