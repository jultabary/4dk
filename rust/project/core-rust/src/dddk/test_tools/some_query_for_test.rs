#[cfg(test)]
pub mod some_query_for_test {
    use std::any::Any;
    use crate::dddk::query::query::Query;

    pub struct AQuery {}

    impl Query for AQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherQuery {}

    impl Query for AnotherQuery {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}