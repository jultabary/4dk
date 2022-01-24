#[cfg(test)]
pub mod some_query_for_test {
    use std::any::Any;
    use std::fmt::{Debug, Formatter};
    use crate::dddk::query::query::Query;

    pub struct AQuery {}

    impl Query for AQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_query_name(&self) -> String {
            "AQuery".to_string()
        }
    }

    impl Debug for AQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AQuery")
        }
    }

    pub struct AnotherQuery {}

    impl Debug for AnotherQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnotherQuery")
        }
    }

    impl Query for AnotherQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_query_name(&self) -> String {
            "AnotherQuery".to_string()
        }
    }
}