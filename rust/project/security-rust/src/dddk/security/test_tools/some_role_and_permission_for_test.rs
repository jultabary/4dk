#[cfg(test)]
pub mod some_role_and_permission_for_test {
    use crate::dddk::security::permission::Permission;
    use crate::dddk::security::role::Role;

    pub fn get_permission_1() -> Permission {
        Permission::new(String::from("1"))
    }

    pub fn get_permission_2() -> Permission {
        Permission::new(String::from("2"))
    }

    pub fn get_permission_3() -> Permission {
        Permission::new(String::from("3"))
    }

    pub fn get_a_role() -> Role {
        Role::new(String::from("a_role"),
                  vec![get_permission_1(), get_permission_2()])
    }

    pub fn get_another_role() -> Role {
        Role::new(String::from("another_role"),
                  vec![get_permission_3()])
    }
}