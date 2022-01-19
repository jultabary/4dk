use crate::dddk::security::authorization::Authorization;
use crate::dddk::security::permission::Permission;

pub trait AuthorizedStrategy {
    fn is_authorized(&self, expected_permission: Permission, given_role_names: Vec<String>) -> Authorization;
}
