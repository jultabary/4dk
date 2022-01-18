#[cfg(test)]
pub mod fake_role_repository {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use crate::dddk::security::authorized_strategy_impl::role_read_repository::RoleReadRepository;
    use crate::dddk::security::permission::Permission;
    use crate::dddk::security::role::Role;

    pub struct FakeRoleReadRepository {
        store: RefCell<Vec<Role>>,
    }

    impl FakeRoleReadRepository {
        pub fn new(store: Vec<Role>) -> FakeRoleReadRepository {
            FakeRoleReadRepository {
                store: RefCell::new(store)
            }
        }

        pub fn save(&self, role: Role) {
            self.store.borrow_mut().push(role);
        }
    }

    impl RoleReadRepository for FakeRoleReadRepository {
        fn find_all_roles(&self) -> Vec<Role> {
            self.store.borrow().clone()
        }

        fn find_all_permissions(&self) -> Vec<Permission> {
            let mut permissions = HashSet::new();
            self.store.borrow().iter()
                .for_each(|role| {
                    role.get_associated_permissions().iter()
                        .for_each(|permission| { permissions.insert(permission.clone()); })
                });
            let mut roles_vec = Vec::new();
            permissions.into_iter().for_each(|role| { roles_vec.push(role.clone()) });
            roles_vec
        }

        fn find_role_by_name(&self, name: String) -> Option<Role> {
            if let Some(role) = self.store.borrow().iter()
                .find(|&role| { role.get_name() == &name }) {
                return Some(role.clone());
            }
            None
        }

        fn find_permissions_for_given_role(&self, name: String) -> Vec<Permission> {
            if let Some(role) = self.store.borrow().iter()
                .find(|&role| { role.get_name() == &name }) {
                return role.get_associated_permissions().clone();
            }
            Vec::new()
        }
    }
}