#[cfg(test)]
pub mod some_secured_query_for_test {
    use std::any::Any;
    use dddk_core::dddk::query::query::Query;
    use crate::dddk::security::query::secured_query::SecuredQuery;

    pub struct AQuery {}

    impl AQuery {
        pub fn new() -> AQuery {
            AQuery {}
        }
    }

    impl Query for AQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub fn get_a_query_secured(roles: Vec<String>) -> SecuredQuery {
        SecuredQuery::new(Box::new(AQuery::new()), roles)
    }

    pub struct AnotherQuery {}

    impl AnotherQuery {
        pub fn new() -> AnotherQuery {
            AnotherQuery {}
        }
    }

    impl Query for AnotherQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}