#[cfg(test)]
mod test {
    use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
    use crate::dddk::security::query::secured_query_handler::SecuredQueryHandler;
    use crate::dddk::security::test_tools::some_secured_query_handler_for_test::some_secured_query_handler_for_test::{AQueryHandler, get_a_query_handler_secured};

    #[test]
    fn it_should_return_sub_query_handler_when_call_get_query_handler() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();

        // When
        let query_handler = a_query_handler_secured.get_query_handler();
        let a_query_handler_opt = query_handler.as_any().downcast_ref::<AQueryHandler>();

        // Then
        assert_eq!(true, a_query_handler_opt.is_some());
    }

    #[test]
    fn it_should_downcast_ref_secured_query_handler_when_receiving_it_as_query_handler() {
        // Given
        let a_query_handler_secured = get_a_query_handler_secured();
        let query_handler = &a_query_handler_secured as &dyn QueryHandlerInBus;

        // When
        let a_secured_query_handler_opt = query_handler.as_any()
            .downcast_ref::<SecuredQueryHandler>();

        // Then
        assert_eq!(true, a_secured_query_handler_opt.is_some());
        let query_handler = a_secured_query_handler_opt.unwrap().get_query_handler();
        let a_query_handler = query_handler.as_any().downcast_ref::<AQueryHandler>();
        assert_eq!(true, a_query_handler.is_some());
    }
}