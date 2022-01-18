#[cfg(test)]
pub mod test {
    use crate::dddk::security::authorized_strategy_impl::role_read_repository::RoleReadRepository;
    use crate::dddk::security::permission::Permission;
    use crate::dddk::security::role::Role;
    use crate::dddk::security::test_tools::fake_role_role_repository::fake_role_repository::FakeRoleReadRepository;

    #[test]
    fn it_should_find_all_permissions_stored() {
        // Given
        let permission_1 = Permission::new(String::from("1"));
        let permission_2 = Permission::new(String::from("2"));
        let permission_3 = Permission::new(String::from("3"));
        let a_role = Role::new(String::from("a_role"), vec![permission_1, permission_2]);
        let another_role = Role::new(String::from("a_role"), vec![permission_3]);
        let fake_repo = FakeRoleReadRepository::new(vec![a_role, another_role]);

        // When
        let permissions = fake_repo.find_all_permissions();

        // Then
        assert_eq!(3, permissions.len());
    }

    #[test]
    fn it_should_persist_role_into_store() {
        // Given
        let permission_1 = Permission::new(String::from("1"));
        let a_role = Role::new(String::from("a_role"), vec![permission_1]);
        let fake_repo = FakeRoleReadRepository::new(Vec::new());

        // When
        fake_repo.save(a_role.clone());

        // Then
        assert_eq!(true, fake_repo.find_role_by_name(String::from("a_role")).is_some());
    }
}
