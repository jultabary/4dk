use std::cell::RefCell;
use std::collections::HashSet;
use dddk_security::dddk::security::authorized_strategy_impl::role_read_repository::RoleReadRepository;
use dddk_security::dddk::security::permission::Permission;
use dddk_security::dddk::security::role::Role;

pub struct RoleRepository {
    store: RefCell<Vec<Role>>,
}

impl RoleRepository {
    pub fn new(store: Vec<Role>) -> RoleRepository {
        RoleRepository {
            store: RefCell::new(store)
        }
    }

    pub fn save(&self, role: Role) {
        self.store.borrow_mut().push(role);
    }

}

impl RoleReadRepository for RoleRepository {
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
        Vec::from_iter(permissions)
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

#[test]
fn it_should_find_all_permissions_stored() {
    // Given
    let permission_1 = Permission::new(String::from("1"));
    let permission_2 = Permission::new(String::from("2"));
    let permission_3 = Permission::new(String::from("3"));
    let a_role = Role::new(String::from("a_role"), vec![permission_1, permission_2]);
    let another_role = Role::new(String::from("a_role"), vec![permission_3]);
    let fake_repo = RoleRepository::new(vec![a_role, another_role]);

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
    let fake_repo = RoleRepository::new(Vec::new());

    // When
    fake_repo.save(a_role.clone());

    // Then
    assert_eq!(true, fake_repo.find_role_by_name(String::from("a_role")).is_some());
}
