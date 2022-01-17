#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::sync::Arc;
    use crate::dddk::event::event_handler::EventHandler;
    use crate::dddk::test::some_event_for_test::{AnEvent, AnotherEvent};
    use crate::dddk::test::some_event_handler_for_test::{AnEventHandler, has_event_been_handled_by_handler, reset_event_handled};

    #[test]
    fn it_should_handle_event_when_event_is_associated_to_this_handler() {
        // Given
        let mut an_event_handler = AnEventHandler::new();
        let an_event = Arc::new(AnEvent::new(1));

        // When
        an_event_handler.handle_generic_event(an_event.clone());

        // Then
        unsafe {
            assert_eq!(true, has_event_been_handled_by_handler(an_event, TypeId::of::<AnEventHandler>()));
        }
    }

    #[test]
    fn it_should_not_handle_event_when_event_is_not_associated_to_this_handler() {
        // Given
        let mut an_event_handler = AnEventHandler::new();
        let another_event = Arc::new(AnotherEvent::new(2));

        // When
        an_event_handler.handle_generic_event(another_event.clone());

        // Then
        unsafe {
            assert_eq!(false, has_event_been_handled_by_handler(another_event, TypeId::of::<AnEventHandler>()));
            reset_event_handled();
        }
    }
}