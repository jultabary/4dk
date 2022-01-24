#[cfg(test)]
pub mod some_event_for_test {
    use std::any::Any;
    use std::fmt::{Debug, Formatter};
    use crate::dddk::event::event::Event;

    pub struct AnEvent {
        pub id: i32,
    }

    impl AnEvent {
        pub fn new(id: i32) -> AnEvent {
            AnEvent { id }
        }
    }

    impl Debug for AnEvent {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnEvent")
        }
    }

    impl Event for AnEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_event_name(&self) -> String {
            "AnEvent".to_string()
        }
    }

    unsafe impl Send for AnEvent {}

    pub struct AnotherEvent {
        pub id: i32,
    }

    impl AnotherEvent {
        pub fn new(id: i32) -> AnotherEvent {
            AnotherEvent { id }
        }
    }

    impl Debug for AnotherEvent {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnEvent")
        }
    }

    impl Event for AnotherEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_event_name(&self) -> String {
            "AnotherEvent".to_string()
        }
    }

    unsafe impl Send for AnotherEvent {}
}