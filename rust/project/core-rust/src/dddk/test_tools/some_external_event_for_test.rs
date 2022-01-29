#[cfg(test)]
pub mod some_external_event_for_test {
    use std::any::Any;
    use crate::dddk::external_event::external_event::ExternalEvent;

    pub struct AnExternalEvent {}

    impl ExternalEvent for AnExternalEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_external_event_name(&self) -> String {
            "AnExternalEvent".to_string()
        }
    }

    pub struct AnotherExternalEvent {}

    impl ExternalEvent for AnotherExternalEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_external_event_name(&self) -> String {
            "AnotherExternalEvent".to_string()
        }
    }

}
