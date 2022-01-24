#[cfg(test)]
pub mod event_handler_logger_test {
    use std::any::TypeId;
    use std::sync::Arc;
    use log::Log;
    use crate::dddk::event::event_handler::EventHandlerInBus;
    use crate::dddk::event::event_handler_logger::EventHandlerLogger;
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::AnEvent;
    use crate::dddk::test_tools::some_event_handler_for_test::some_event_handler_for_test::{AnEventHandler, EventHandlerForTest};
    use crate::dddk::test_tools::some_logger_for_test::some_logger_for_test::MockLogger;

    #[test]
    pub fn it_should_instantiate_event_handling_with_its_inner_handler_correctly() {
        // Given
        let event_handler = AnEventHandler::new();

        // When
        let event_handler_logger = EventHandlerLogger::new(Box::new(event_handler) as Box<dyn EventHandlerInBus>);

        // Then
        assert_eq!("AnEventHandler".to_string(), event_handler_logger.get_event_handler_name());
        assert_eq!(TypeId::of::<AnEvent>(), event_handler_logger.get_associated_event_from_bus());
        assert_eq!(true, event_handler_logger.as_any().downcast_ref::<AnEventHandler>().is_some());
    }

    #[test]
    pub fn it_should_dispatch_event_handled_to_its_inner_event_handler() {
        // Given
        let event_handler = AnEventHandler::new();
        let event_handler_logger = EventHandlerLogger::new(Box::new(event_handler) as Box<dyn EventHandlerInBus>);
        let an_event = Arc::new(AnEvent::new(1));

        // When
        let _result = event_handler_logger.handle_from_bus(an_event);

        // Then
        assert_eq!(true, event_handler_logger.as_any().downcast_ref::<AnEventHandler>().is_some());
        let an_event_handler = event_handler_logger.as_any().downcast_ref::<AnEventHandler>().unwrap();
        assert_eq!(true, an_event_handler.has_event_been_handled(1));
    }

    pub fn it_should_log_event_handling_when_there_is_an_handler_for_event(logger: &MockLogger) {
        // Given
        let event_handler = AnEventHandler::new();
        let event_handler_logger = EventHandlerLogger::new(Box::new(event_handler) as Box<dyn EventHandlerInBus>);
        let an_event = Arc::new(AnEvent::new(1));

        // When
        let _result = event_handler_logger.handle_from_bus(an_event);

        // Then
        assert_eq!(2, logger.messages.borrow().len());
        let first_log = "INFO_Handling an event [AnEvent] by [AnEventHandler].".to_string();
        let second_log = "INFO_Event[AnEvent] has been handled by [AnEventHandler].".to_string();
        assert_eq!(&first_log, logger.messages.borrow().get(0).unwrap());
        assert_eq!(&second_log, logger.messages.borrow().get(1).unwrap());
        logger.flush();
    }
}
