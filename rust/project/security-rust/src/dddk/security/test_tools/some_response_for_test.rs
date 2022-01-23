#[cfg(test)]
pub mod some_response_for_test {
    use std::any::Any;
    use dddk_core::dddk::query::response::Response;

    pub struct AResponse {}

    impl AResponse {
        pub fn new() -> AResponse {
            AResponse {}
        }
    }

    impl Response for AResponse {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_response_name(&self) -> String {
            "AResponse".to_string()
        }
    }

    pub struct AnotherResponse {}

    impl AnotherResponse {
        pub fn new() -> AnotherResponse {
            AnotherResponse {}
        }
    }

    impl Response for AnotherResponse {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_response_name(&self) -> String {
            "AnotherResponse".to_string()
        }
    }
}