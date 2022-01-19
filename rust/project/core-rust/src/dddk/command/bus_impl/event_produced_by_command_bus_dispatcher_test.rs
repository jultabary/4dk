#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::sync::Arc;
    use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
    use crate::dddk::command::bus_impl::event_produced_by_command_bus_dispatcher::EventsProducedByCommandBusDispatcher;
    use crate::dddk::command::command_bus::CommandBus;
    use crate::dddk::command::command_handler::CommandHandlerInBus;
    use crate::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
    use crate::dddk::event::event_handler::EventHandlerInBus;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::ACommand;
    use crate::dddk::test_tools::some_command_handler_for_test::some_command_handler_for_test::ACommandHandler;
    use crate::dddk::test_tools::some_event_for_test::some_event_for_test::AnEvent;
    use crate::dddk::test_tools::some_event_handler_for_test::some_event_handler_for_test::{AnEventHandler, AnotherEventHandler, AThirdEventHandler, EventHandlerForTest};

    #[test]
    fn it_should_return_expected_events_from_bus_when_command_is_dispatched() {
        // Given
        let a_command_handler = ACommandHandler::new();
        let expected_event_id_returned = a_command_handler.get_event_id_returned();
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));
        let command_dispatcher = CommandDispatcher::new(command_handlers);

        let an_event_handler = AnEventHandler::new();
        let a_third_event_handler = AThirdEventHandler::new();
        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
        event_handlers.push(Box::new(an_event_handler));
        event_handlers.push(Box::new(a_third_event_handler));
        let event_bus = Arc::new(EventDispatcher::new(event_handlers));

        let command_bus = EventsProducedByCommandBusDispatcher::new(
            Box::new(command_dispatcher),
            event_bus.clone(),
            false
        );

        let a_command = ACommand { };

        // When
        let events = command_bus.dispatch(&a_command);

        // Then
        assert_eq!(true, events.is_ok());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<AnEvent>().is_some());
        let event_handlers_opt = event_bus.get_event_handlers_by_its_events(TypeId::of::<AnEvent>());
        event_handlers_opt.unwrap().iter()
            .for_each(|event_handler| {
                if let Some(an_event_handler) = event_handler.as_any().downcast_ref::<AnEventHandler>() {
                    assert_eq!(true, an_event_handler.has_event_been_handled(expected_event_id_returned));
                } else if let Some(a_third_event_handler) = event_handler.as_any().downcast_ref::<AnotherEventHandler>() {
                    assert_eq!(true, a_third_event_handler.has_event_been_handled(expected_event_id_returned));
                }
            });
    }
}