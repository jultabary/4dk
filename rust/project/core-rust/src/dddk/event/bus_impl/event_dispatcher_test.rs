#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::sync::Arc;
    use crate::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
    use crate::dddk::event::event_bus::EventBus;
    use crate::dddk::event::event_handler::EventHandlerInBus;
    use crate::dddk::test::some_event_for_test::AnEvent;
    use crate::dddk::test::some_event_handler_for_test::{AnEventHandler, AnotherEventHandler, AThirdEventHandler, has_event_been_handled_by_handler, reset_event_handled};

    #[test]
    fn it_should_dispatch_event_to_its_correct_handlers() {
        // Given
        let an_event_handler = AnEventHandler::new();
        let another_event_handler = AnotherEventHandler::new();
        let a_third_event_handler = AThirdEventHandler::new();

        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;

        event_handlers.push(Box::new(an_event_handler));
        event_handlers.push(Box::new(another_event_handler));
        event_handlers.push(Box::new(a_third_event_handler));

        let mut event_bus = EventDispatcher::new(event_handlers);
        let an_event = Arc::new(AnEvent::new(1));

        // When
        event_bus.dispatch(an_event.clone());

        // Then
        unsafe {
            assert_eq!(true, has_event_been_handled_by_handler(an_event.clone(), TypeId::of::<AnEventHandler>()));
            assert_eq!(false, has_event_been_handled_by_handler(an_event.clone(), TypeId::of::<AnotherEventHandler>()));
            assert_eq!(true, has_event_been_handled_by_handler(an_event.clone(), TypeId::of::<AThirdEventHandler>()));
            reset_event_handled();
        }
    }
}