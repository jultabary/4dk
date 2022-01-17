#[cfg(test)]
pub mod some_response_for_test {
    use std::any::Any;
    use crate::dddk::query::response::Response;

    pub struct AResponse {}

    impl Response for AResponse {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherResponse {}

    impl Response for AnotherResponse {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}