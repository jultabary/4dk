#[cfg(test)]
pub mod some_policy_handler_for_test {
    use std::any::{Any, TypeId};
    use crate::dddk::aliases::Commands;
    use crate::dddk::external_event::external_event::ExternalEvent;
    use crate::dddk::external_event::policy_handler::{PolicyHandler, PolicyHandlerInBus};
    use crate::dddk::test_tools::some_command_for_test::some_command_for_tests::{ACommand, AnotherCommand};
    use crate::dddk::test_tools::some_external_event_for_test::some_external_event_for_test::{AnExternalEvent, AnotherExternalEvent};

    pub struct APolicyHandler {}

    impl APolicyHandler {
        pub fn new() -> APolicyHandler {
            APolicyHandler {}
        }
    }

    impl PolicyHandler<AnExternalEvent> for APolicyHandler {
        fn handle(&self, _external_event: &AnExternalEvent) -> Commands {
            Ok(vec![Box::new(ACommand {})])
        }
    }

    impl PolicyHandlerInBus for APolicyHandler {
        fn handle_from_bus<'a>(&self, external_event: &'a dyn ExternalEvent) -> Commands {
            self.handle_generic_external_event(external_event)
        }

        fn get_associated_external_event_from_bus(&self) -> TypeId {
            self.get_associated_external_event()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_policy_handler_name(&self) -> String {
            "APolicyHandler".to_string()
        }
    }

    pub struct AnotherPolicyHandler {}

    impl AnotherPolicyHandler {
        pub fn new() -> AnotherPolicyHandler {
            AnotherPolicyHandler {}
        }
    }

    impl PolicyHandler<AnotherExternalEvent> for AnotherPolicyHandler {
        fn handle(&self, _external_event: &AnotherExternalEvent) -> Commands {
            Ok(vec![Box::new(AnotherCommand {})])
        }
    }

    impl PolicyHandlerInBus for AnotherPolicyHandler {
        fn handle_from_bus<'a>(&self, external_event: &'a dyn ExternalEvent) -> Commands {
            self.handle_generic_external_event(external_event)
        }

        fn get_associated_external_event_from_bus(&self) -> TypeId {
            self.get_associated_external_event()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_policy_handler_name(&self) -> String {
            "AnotherPolicyHandler".to_string()
        }
    }
}