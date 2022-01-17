#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::sync::Arc;
    use crate::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
    use crate::dddk::event::event_bus::EventBus;
    use crate::dddk::event::event_handler::EventHandlerInBus;
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::test_tools::some_event_handler_for_test::some_event_handler_for_test::{AnEventHandler, AnotherEventHandler, AThirdEventHandler, EventHandlerForTest};

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
        let event_id = 1;
        let an_event = Arc::new(AnEvent::new(event_id));

        // When
        event_bus.dispatch(an_event.clone());

        // Then
        let an_event_handlers = event_bus.get_event_handlers_by_its_events(TypeId::of::<AnEvent>()).unwrap();
        an_event_handlers.iter()
            .for_each(|event_handler| {
                if let Some(an_event_handler) = event_handler.as_any().downcast_ref::<AnEventHandler>() {
                    assert_eq!(true, an_event_handler.has_event_been_handled(event_id));
                } else if let Some(a_third_event_handler) = event_handler.as_any().downcast_ref::<AnotherEventHandler>() {
                    assert_eq!(true, a_third_event_handler.has_event_been_handled(event_id));
                }
            });

        let another_event_handler = event_bus
            .get_event_handlers_by_its_events(TypeId::of::<AnotherEvent>())
            .unwrap()
            .get(0)
            .unwrap()
            .as_any()
            .downcast_ref::<AnotherEventHandler>()
            .unwrap();
        assert_eq!(false, another_event_handler.has_event_been_handled(event_id));
    }
}