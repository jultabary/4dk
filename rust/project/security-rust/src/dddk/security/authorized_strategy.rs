use crate::dddk::security::authorization::Authorization;
use crate::dddk::security::permission::Permission;
use crate::dddk::security::role::Role;

pub trait AuthorizedStrategy {
    fn is_authorized(&self, expected_permission: &Permission, given_roles: &Vec<Role>) -> Authorization;
}
