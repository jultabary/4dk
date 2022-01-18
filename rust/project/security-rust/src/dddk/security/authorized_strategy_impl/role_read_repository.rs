use crate::dddk::security::permission::Permission;
use crate::dddk::security::role::Role;

pub trait RoleReadRepository {
    fn find_all_roles(&self) -> Vec<Role>;

    fn find_all_permissions(&self) -> Vec<Permission>;

    fn find_role_by_name(&self, name: String) -> Option<Role>;

    fn find_permissions_for_given_role(&self, name: String) -> Vec<Permission>;
}