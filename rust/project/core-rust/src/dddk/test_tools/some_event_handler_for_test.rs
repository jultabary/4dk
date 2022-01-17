#[cfg(test)]
pub mod some_event_handler_for_test {
    use std::any::{Any, TypeId};
    use std::cell::RefCell;
    use std::sync::Arc;
    use crate::dddk::event::event::Event;
    use crate::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};

    fn push_event_id_to_store(event_ids_handled: &RefCell<Vec<i32>>, id: i32) {
        event_ids_handled.borrow_mut().push(id);
    }

    pub trait EventHandlerForTest<E: Event + Any>: EventHandlerInBus + EventHandler<E> {
        fn get_handled_events(&self) -> &RefCell<Vec<i32>>;

        fn has_event_been_handled(&self, id: i32) -> bool {
            self.get_handled_events().borrow_mut().contains(&id)
        }
    }

    pub struct AnEventHandler {
        handled_events: RefCell<Vec<i32>>,
    }

    impl AnEventHandler {
        pub fn new() -> AnEventHandler {
            AnEventHandler { handled_events: RefCell::new(Vec::new()) }
        }
    }

    impl EventHandlerForTest<AnEvent> for AnEventHandler {
        fn get_handled_events(&self) -> &RefCell<Vec<i32>> {
            &self.handled_events
        }
    }

    impl EventHandler<AnEvent> for AnEventHandler {
        fn handle(&self, event: &AnEvent) {
            push_event_id_to_store(&self.handled_events, event.id);
        }
    }

    impl EventHandlerInBus for AnEventHandler {
        fn handle_from_bus(&self, event: Arc<dyn Event>) {
            self.handle_generic_event(event);
        }

        fn get_associated_event_from_bus(&self) -> TypeId {
            self.get_associated_event()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub struct AnotherEventHandler {
        handled_events: RefCell<Vec<i32>>,
    }

    impl AnotherEventHandler {
        pub fn new() -> AnotherEventHandler {
            AnotherEventHandler { handled_events: RefCell::new(Vec::new()) }
        }
    }

    impl EventHandlerForTest<AnotherEvent> for AnotherEventHandler {
        fn get_handled_events(&self) -> &RefCell<Vec<i32>> {
            &self.handled_events
        }
    }

    impl EventHandler<AnotherEvent> for AnotherEventHandler {
        fn handle(&self, event: &AnotherEvent) {
            push_event_id_to_store(&self.handled_events, event.id);
        }
    }

    impl EventHandlerInBus for AnotherEventHandler {
        fn handle_from_bus(&self, event: Arc<dyn Event>) {
            self.handle_generic_event(event);
        }

        fn get_associated_event_from_bus(&self) -> TypeId {
            self.get_associated_event()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }


    pub struct AThirdEventHandler {
        handled_events: RefCell<Vec<i32>>,
    }

    impl EventHandlerForTest<AnEvent> for AThirdEventHandler {
        fn get_handled_events(&self) -> &RefCell<Vec<i32>> {
            &self.handled_events
        }
    }

    impl EventHandler<AnEvent> for AThirdEventHandler {
        fn handle(&self, event: &AnEvent) {
            push_event_id_to_store(&self.handled_events, event.id);
        }
    }

    impl AThirdEventHandler {
        pub fn new() -> AThirdEventHandler {
            AThirdEventHandler { handled_events: RefCell::new(Vec::new()) }
        }
    }

    impl EventHandlerInBus for AThirdEventHandler {
        fn handle_from_bus(&self, event: Arc<dyn Event>) {
            self.handle_generic_event(event);
        }

        fn get_associated_event_from_bus(&self) -> TypeId {
            self.get_associated_event()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}