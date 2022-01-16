#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use crate::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
    use crate::dddk::event::event_bus::EventBus;
    use crate::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
    use crate::dddk::test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::test::some_event_handler_for_test::{AnEventHandler, AnotherEventHandler, has_event_been_handled_by_handler};

    #[test]
    fn it_should_dispatch_event_to_its_correct_handler() {
        // Given
        let an_event_handler = AnEventHandler::new();
        let another_event_handler = AnotherEventHandler::new();

        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
        event_handlers.push(Box::new(an_event_handler));
        event_handlers.push(Box::new(another_event_handler));

        let event_bus = EventDispatcher::new(event_handlers);
        let an_event = AnEvent::new(1);

        // When
        event_bus.dispatch(&an_event);

        // Then
        unsafe {
            assert_eq!(true, has_event_been_handled_by_handler(&an_event, TypeId::of::<AnEventHandler>()));
        }
    }
}