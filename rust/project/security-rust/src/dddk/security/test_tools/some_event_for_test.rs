#[cfg(test)]
pub mod some_event_for_test {
    use std::any::Any;
    use dddk_core::dddk::event::event::Event;

    pub struct AnEvent {}

    impl AnEvent {
        pub fn new() -> AnEvent {
            AnEvent {}
        }
    }

    impl Event for AnEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherEvent {}

    impl AnotherEvent {
        pub fn new() -> AnotherEvent {
            AnotherEvent {}
        }
    }

    impl Event for AnotherEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}