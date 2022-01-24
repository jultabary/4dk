#[cfg(test)]
pub mod some_event_for_test {
    use std::any::Any;
    use std::fmt::{Debug, Formatter};
    use dddk_core::dddk::event::event::Event;
    use dddk_macro::Event;

    #[derive(Event)]
    pub struct AnEvent {}

    impl Debug for AnEvent {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnEvent")
        }
    }

    impl AnEvent {
        pub fn new() -> AnEvent {
            AnEvent {}
        }
    }

    #[derive(Event)]
    pub struct AnotherEvent {}

    impl Debug for AnotherEvent {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnotherEvent")
        }
    }

    impl AnotherEvent {
        pub fn new() -> AnotherEvent {
            AnotherEvent {}
        }
    }
}