use std::any::Any;
use dddk_core::dddk::query::query::Query;

pub struct SecuredQuery {
    query: Box<dyn Query>,
    role_names: Vec<String>,
}

impl SecuredQuery {
    pub fn new(query: Box<dyn Query>, role_names: Vec<String>) -> SecuredQuery {
        SecuredQuery {
            query,
            role_names,
        }
    }

    pub fn get_query(&self) -> &dyn Query {
        self.query.as_ref()
    }

    pub fn get_roles_names(&self) -> &Vec<String> {
        &self.role_names
    }
}

impl Query for SecuredQuery {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_query_name(&self) -> String {
        self.query.get_query_name()
    }
}
