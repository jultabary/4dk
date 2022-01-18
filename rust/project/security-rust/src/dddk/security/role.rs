use crate::dddk::security::permission::Permission;

#[derive(Eq, Hash)]
pub struct Role {
    name: String,
    associated_permissions: Vec<Permission>,
}

impl Role {
    pub fn new(name: String, associated_permissions: Vec<Permission>) -> Role {
        Role {
            name,
            associated_permissions,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_associated_permissions(&self) -> &Vec<Permission> {
        &self.associated_permissions
    }
}

impl Clone for Role {
    fn clone(&self) -> Self {
        let name = self.get_name().clone();
        let associated_permissions = self.get_associated_permissions().clone();
        Role::new(name, associated_permissions)
    }
}

impl PartialEq<Self> for Role {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() &&
            self.associated_permissions == other.associated_permissions
    }
}
