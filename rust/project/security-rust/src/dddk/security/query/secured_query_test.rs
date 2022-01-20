#[cfg(test)]
mod test {
    use dddk_core::dddk::query::query::Query;
    use crate::dddk::security::query::secured_query::SecuredQuery;
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_a_role;
    use crate::dddk::security::test_tools::some_secured_query_for_test::some_secured_query_for_test::{AQuery, get_a_query_secured};

    #[test]
    fn it_should_return_sub_query_when_call_get_query() {
        // Given
        let user_roles = vec![get_a_role().get_name().clone()];
        let a_query_secured = get_a_query_secured(user_roles);

        // When
        let query = a_query_secured.get_query();
        let a_query_opt = query.as_any().downcast_ref::<AQuery>();

        // Then
        assert_eq!(true, a_query_opt.is_some());
        assert_eq!(&vec![get_a_role().get_name().clone()], a_query_secured.get_roles_names());
    }

    #[test]
    fn it_should_downcast_ref_secured_query_when_receiving_it_as_query() {
        // Given
        let user_roles = vec![get_a_role().get_name().clone()];
        let a_query_secured = get_a_query_secured(user_roles);
        let query = &a_query_secured as &dyn Query;

        // When
        let a_secured_query_opt = query.as_any().downcast_ref::<SecuredQuery>();

        // Then
        assert_eq!(true, a_secured_query_opt.is_some());
        let a_query_secured = a_secured_query_opt.unwrap();
        let a_query_opt = a_query_secured.get_query().as_any().downcast_ref::<AQuery>();
        assert_eq!(true, a_query_opt.is_some());
    }
}