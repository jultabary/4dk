use std::rc::Rc;
use crate::dddk::security::authorization::Authorization;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::authorized_strategy_impl::role_read_repository::RoleReadRepository;
use crate::dddk::security::permission::Permission;

pub struct NoSecurityStrategy {
    role_read_repository: Rc<dyn RoleReadRepository>,
}

impl NoSecurityStrategy {
    pub fn new(role_read_repository: Rc<dyn RoleReadRepository>) -> NoSecurityStrategy {
        NoSecurityStrategy {
            role_read_repository
        }
    }
}

impl AuthorizedStrategy for NoSecurityStrategy{
    fn is_authorized(&self, _expected_permission: Permission, _given_roles: Vec<String>) -> Authorization {
        let permissions = self.role_read_repository.find_all_permissions();
        Authorization::new(true, permissions)
    }
}