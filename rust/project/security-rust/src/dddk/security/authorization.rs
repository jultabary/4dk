use crate::dddk::security::permission::Permission;

pub struct Authorization {
    is_authorized: bool,
    permissions: Vec<Permission>,
}

impl Authorization {
    pub fn new(is_authorized: bool, permissions: Vec<Permission>) -> Authorization {
        Authorization {
            is_authorized,
            permissions,
        }
    }

    pub fn is_authorized(&self) -> bool {
        self.is_authorized
    }

    pub fn get_permissions(&self) -> &Vec<Permission> {
        &self.permissions
    }
}