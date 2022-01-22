#[cfg(test)]
pub mod some_event_for_test {
    use std::any::Any;
    use dddk_core::dddk::event::event::Event;
    use dddk_macro::Event;

    #[derive(Event)]
    pub struct AnEvent {}

    impl AnEvent {
        pub fn new() -> AnEvent {
            AnEvent {}
        }
    }

    #[derive(Event)]
    pub struct AnotherEvent {}

    impl AnotherEvent {
        pub fn new() -> AnotherEvent {
            AnotherEvent {}
        }
    }
}