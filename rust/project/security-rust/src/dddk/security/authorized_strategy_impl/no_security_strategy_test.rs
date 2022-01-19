#[cfg(test)]
pub mod test {
    use std::rc::Rc;
    use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
    use crate::dddk::security::authorized_strategy_impl::no_security_strategy::NoSecurityStrategy;
    use crate::dddk::security::permission::Permission;
    use crate::dddk::security::role::Role;
    use crate::dddk::security::test_tools::fake_role_role_repository::fake_role_repository::FakeRoleReadRepository;

    #[test]
    fn it_should_authorized_and_return_all_permissions_whatever_param_given() {
        // Given
        let permission_1 = Permission::new(String::from("1"));
        let permission_2 = Permission::new(String::from("2"));
        let permission_3 = Permission::new(String::from("3"));
        let a_role = Role::new(String::from("a_role"), vec![permission_1, permission_2]);
        let another_role = Role::new(String::from("a_role"), vec![permission_3]);
        let fake_repo = Rc::new(FakeRoleReadRepository::new(vec![a_role, another_role]));
        let no_strategy = NoSecurityStrategy::new(fake_repo.clone());

        // When
        let authorization = no_strategy
            .is_authorized(Permission::new(String::from("0")),
                           Vec::new());

        // Then
        assert_eq!(true, authorization.is_authorized());
        assert_eq!(true, authorization.get_permissions().contains(&Permission::new(String::from("1"))));
        assert_eq!(true, authorization.get_permissions().contains(&Permission::new(String::from("2"))));
        assert_eq!(true, authorization.get_permissions().contains(&Permission::new(String::from("3"))));
    }
}
