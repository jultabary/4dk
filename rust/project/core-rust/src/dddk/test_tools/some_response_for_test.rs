#[cfg(test)]
pub mod some_response_for_test {
    use std::any::Any;
    use crate::dddk::query::response::Response;

    pub struct AResponse {}

    impl Response for AResponse {
        fn as_any(&mut self) -> &mut dyn Any {
            self
        }

        fn get_response_name(&self) -> String {
            "AResponse".to_string()
        }
    }

    pub struct AnotherResponse {}

    impl Response for AnotherResponse {
        fn as_any(&mut self) -> &mut dyn Any {
            self
        }

        fn get_response_name(&self) -> String {
            "AnotherResponse".to_string()
        }
    }
}