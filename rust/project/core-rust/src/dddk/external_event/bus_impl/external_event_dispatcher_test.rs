#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use crate::dddk::errors::NoPolicyHandlerRegisterForGivenExternalEvent;
    use crate::dddk::external_event::bus_impl::external_event_dispatcher::ExternalEventDispatcher;
    use crate::dddk::external_event::external_event_bus::ExternalEventBus;
    use crate::dddk::external_event::policy_handler::PolicyHandlerInBus;
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::ACommand;
    use crate::dddk::test_tools::some_external_event_for_test::some_external_event_for_test::{AnExternalEvent, AnotherExternalEvent};
    use crate::dddk::test_tools::some_policy_handler_for_test::some_policy_handler_for_test::{AnotherPolicyHandler, APolicyHandler};


    #[test]
    #[should_panic]
    fn it_should_panic_when_register_two_external_event_handler_for_a_same_external_event() {
        // Given
        let a_external_event_handler_1 = APolicyHandler::new();
        let a_external_event_handler_2 = APolicyHandler::new();

        let mut policy_handlers = Vec::new() as Vec<Box<dyn PolicyHandlerInBus>>;
        policy_handlers.push(Box::new(a_external_event_handler_1));
        policy_handlers.push(Box::new(a_external_event_handler_2));

        // When
        ExternalEventDispatcher::new(policy_handlers);

        // Then
        // should panic
    }

    #[test]
    fn it_should_be_handled_by_correct_handler_when_dispatch_external_event() {
        // Given
        let a_external_event_handler = APolicyHandler::new();
        let another_external_event_handler = AnotherPolicyHandler::new();

        let mut policy_handlers = Vec::new() as Vec<Box<dyn PolicyHandlerInBus>>;
        policy_handlers.push(Box::new(a_external_event_handler));
        policy_handlers.push(Box::new(another_external_event_handler));

        let external_event_bus = ExternalEventDispatcher::new(policy_handlers);
        let a_external_event = AnExternalEvent {};

        // When
        let events = external_event_bus.dispatch(&a_external_event);

        // Then
        assert_eq!(true, events.is_ok());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        let an_event = event.as_ref().as_any().downcast_ref::<ACommand>();
        assert_eq!(true, an_event.is_some());
    }

    #[test]
    fn it_should_return_an_error_when_dispatch_a_external_event_with_no_register_handler() {
        // Given
        let a_external_event_handler = APolicyHandler::new();

        let mut policy_handlers = Vec::new() as Vec<Box<dyn PolicyHandlerInBus>>;
        policy_handlers.push(Box::new(a_external_event_handler));

        let external_event_bus = ExternalEventDispatcher::new(policy_handlers);
        let another_external_event = AnotherExternalEvent {};

        // When
        let responses = external_event_bus.dispatch(&another_external_event);

        // Then
        assert_eq!(true, responses.is_err());
        assert_eq!(true, responses.err().unwrap().downcast_ref::<NoPolicyHandlerRegisterForGivenExternalEvent>().is_some());
    }

    #[test]
    fn it_should_return_external_event_handler_which_is_associated_to_the_given_external_event() {
        // Given
        let a_external_event_handler = APolicyHandler::new();
        let another_external_event_handler = AnotherPolicyHandler::new();

        let mut policy_handlers = Vec::new() as Vec<Box<dyn PolicyHandlerInBus>>;
        policy_handlers.push(Box::new(a_external_event_handler));
        policy_handlers.push(Box::new(another_external_event_handler));

        let external_event_bus = ExternalEventDispatcher::new(policy_handlers);

        // When
        let policy_handler_opt = external_event_bus.get_policy_handler_by_its_external_event(TypeId::of::<AnotherExternalEvent>());

        // Then
        assert_eq!(true, policy_handler_opt.is_some());
        let another_external_event_handler = policy_handler_opt.unwrap()
            .as_any().downcast_ref::<AnotherPolicyHandler>();
        assert_eq!(true, another_external_event_handler.is_some());
    }
}
