use std::collections::HashSet;
use std::rc::Rc;
use crate::dddk::security::authorization::Authorization;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::authorized_strategy_impl::role_read_repository::RoleReadRepository;
use crate::dddk::security::permission::Permission;

pub struct RoleBasedStrategy {
    role_repository: Rc<dyn RoleReadRepository>,
}

impl RoleBasedStrategy {
    pub fn new(role_repository: Rc<dyn RoleReadRepository>) -> RoleBasedStrategy {
        RoleBasedStrategy {
            role_repository
        }
    }
}

impl AuthorizedStrategy for RoleBasedStrategy {
    fn is_authorized(&self, expected_permission: Permission, given_roles: Vec<String>) -> Authorization {
        let mut user_permissions = HashSet::new();
        given_roles.iter().for_each(|role| {
            if let Some(role) = self.role_repository.find_role_by_name(role.clone()) {
                role.get_associated_permissions().iter().for_each(
                    |permission| { user_permissions.insert(permission.clone()); }
                );
            }
        });
        if user_permissions.contains(&expected_permission) {
            return Authorization::new(true, Vec::from_iter(user_permissions));
        }
        Authorization::new(false, Vec::new())
    }
}