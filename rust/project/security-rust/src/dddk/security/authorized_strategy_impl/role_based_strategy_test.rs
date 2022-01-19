#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
    use crate::dddk::security::authorized_strategy_impl::role_based_strategy::RoleBasedStrategy;
    use crate::dddk::security::permission::Permission;
    use crate::dddk::security::role::Role;
    use crate::dddk::security::test_tools::fake_role_repository::fake_role_repository::{FakeRoleReadRepository, get_fake_repository_with_filled_with_roles};
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::{get_a_role, get_permission_1, get_permission_2, get_permission_3};

    #[test]
    fn it_should_authorized_when_expected_permission_is_contained_in_given_role() {
        // Given
        let fake_repo = get_fake_repository_with_filled_with_roles();
        let role_based_strategy = RoleBasedStrategy::new(fake_repo.clone());
        let user_roles = vec![get_a_role().get_name().clone()];

        // When
        let authorization = role_based_strategy
            .is_authorized(Permission::new(String::from("1")),
                           &user_roles,
            );

        // Then
        assert_eq!(true, authorization.is_authorized());
        assert_eq!(true, authorization.get_permissions().contains(&get_permission_1()));
        assert_eq!(true, authorization.get_permissions().contains(&get_permission_2()));
        assert_eq!(false, authorization.get_permissions().contains(&get_permission_3()));
    }

    #[test]
    fn it_should_not_authorized_when_expected_permission_is_not_contained_in_given_role() {
        // Given
        let permission_1 = Permission::new(String::from("1"));
        let permission_2 = Permission::new(String::from("2"));
        let permission_3 = Permission::new(String::from("3"));
        let a_role = Role::new(String::from("a_role"),
                               vec![permission_1.clone(), permission_2.clone()]);
        let user_roles = vec![String::from("another_role")];
        let another_role = Role::new(String::from("another_role"),
                                     vec![permission_3.clone()]);
        let fake_repo = Rc::new(FakeRoleReadRepository::new(vec![a_role, another_role]));
        let role_based_strategy = RoleBasedStrategy::new(fake_repo.clone());

        // When
        let authorization = role_based_strategy
            .is_authorized(Permission::new(String::from("1")),
                           &user_roles,
            );

        // Then
        assert_eq!(false, authorization.is_authorized());
        assert_eq!(0, authorization.get_permissions().len());
    }
}