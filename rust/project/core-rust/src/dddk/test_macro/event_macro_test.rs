#[cfg(test)]
pub mod event_macro_tests {
    use std::any::{Any, TypeId};
    use std::cell::RefCell;
    use crate::dddk::event::event::Event;
    use dddk_macro::Event;
    use std::sync::Arc;
    use dddk_macro::EventHandlerInBus;
    use crate::dddk::aliases::GenericError;
    use crate::dddk::event::event_handler::EventHandlerInBus;
    use crate::dddk::event::event_handler::EventHandler;

    #[derive(Event)]
    struct AnEvent {}

    #[derive(EventHandlerInBus)]
    struct AnEventHandler {
        has_been_called: RefCell<bool>,
    }

    impl EventHandler<AnEvent> for AnEventHandler {
        fn handle(&self, _event: &AnEvent) -> Result<(), GenericError> {
            self.has_been_called.replace(true);
            Ok(())
        }
    }

    #[test]
    fn it_should_impl_default_behaviour_of_event_trait_when_using_derive_macro() {
        // Given
        let an_event = AnEvent {};

        // When
        let event_name = an_event.get_event_name();
        let as_any: &dyn Any = an_event.as_any();

        // Then
        assert_eq!("AnEvent".to_string(), event_name);
        assert_eq!(true, as_any.downcast_ref::<AnEvent>().is_some());
    }

    #[test]
    fn it_should_impl_default_behaviour_of_query_handler_in_bus_trait_when_using_derive_macro() {
        // Given
        let an_event = AnEvent {};
        let a_event_handler = AnEventHandler { has_been_called: RefCell::new(false) };

        // When
        a_event_handler.handle_from_bus(Arc::new(an_event));
        let event_handler_name = a_event_handler.get_event_handler_name();
        let event_type_id = a_event_handler.get_associated_event_from_bus();

        // Then
        assert_eq!(true, a_event_handler.has_been_called.take());
        assert_eq!("AnEventHandler".to_string(), event_handler_name);
        assert_eq!(TypeId::of::<AnEvent>(), event_type_id);
    }
}