#[cfg(test)]
pub mod some_secured_query_for_test {
    use std::fmt::{Debug, Formatter};
    use dddk_macro::Query;
    use crate::dddk::security::query::secured_query::SecuredQuery;

    #[derive(Query)]
    pub struct AQuery {}

    impl Debug for AQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AQuery")
        }
    }

    impl AQuery {
        pub fn new() -> AQuery {
            AQuery {}
        }
    }

    pub fn get_a_query_secured(roles: Vec<String>) -> SecuredQuery {
        SecuredQuery::new(Box::new(AQuery::new()), roles)
    }

    #[derive(Query)]
    pub struct AnotherQuery {}

    impl Debug for AnotherQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnotherQuery")
        }
    }

    impl AnotherQuery {
        pub fn new() -> AnotherQuery {
            AnotherQuery {}
        }
    }
}