#[cfg(test)]
mod command_produced_by_external_bus_dispatcher_test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{Command, CommandBus, Events};
    use crate::dddk::external_event::bus_impl::command_produced_by_external_event_bus_dispatcher::CommandProducedByExternalEventBusDispatcher;
    use crate::dddk::external_event::bus_impl::external_event_dispatcher::ExternalEventDispatcher;
    use crate::dddk::external_event::external_event_bus::ExternalEventBus;
    use crate::dddk::test_tools::some_external_event_for_test::some_external_event_for_test::AnExternalEvent;
    use crate::dddk::test_tools::some_policy_handler_for_test::some_policy_handler_for_test::APolicyHandler;

    struct FakeCommandBus {
        has_been_called: Rc<RefCell<bool>>,
    }

    impl CommandBus for FakeCommandBus {
        fn dispatch<'b>(&self, _command: &'b dyn Command) -> Events {
            self.has_been_called.replace(true);
            Ok(vec![])
        }
    }

    #[test]
    fn it_should_dispatch_produced_command_to_command_bus() {
        // Given
        let policy_handler = APolicyHandler::new();
        let external_event_bus = ExternalEventDispatcher::new(vec![Box::new(policy_handler)]);
        let has_been_called = Rc::new(RefCell::new(false));
        let command_bus = FakeCommandBus { has_been_called: has_been_called.clone() };
        let command_produced_by_external_event_bus_dispatcher = CommandProducedByExternalEventBusDispatcher::new(
            Box::new(external_event_bus),
            Box::new(command_bus),
        );
        let external_event = AnExternalEvent {};

        // When
        let _commands = command_produced_by_external_event_bus_dispatcher.dispatch(&external_event);

        // Then
        assert_eq!(has_been_called.take(), true);
    }
}