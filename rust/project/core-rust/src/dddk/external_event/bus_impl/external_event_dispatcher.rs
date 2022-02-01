use std::any::TypeId;
use std::collections::HashMap;
use log::debug;
use crate::dddk::aliases::Commands;
use crate::dddk::errors::NoPolicyHandlerRegisterForGivenExternalEvent;
use crate::dddk::external_event::external_event::ExternalEvent;
use crate::dddk::external_event::external_event_bus::ExternalEventBus;
use crate::dddk::external_event::policy_handler::PolicyHandlerInBus;

pub struct ExternalEventDispatcher {
    policy_handlers: HashMap<TypeId, Box<dyn PolicyHandlerInBus>>,
}

impl ExternalEventDispatcher {
    pub fn new(policy_handler_values: Vec<Box<dyn PolicyHandlerInBus>>) -> ExternalEventDispatcher {
        let mut map = HashMap::new();
        policy_handler_values.into_iter().for_each(|item| {
            if let Some(_) = map.get(&item.get_associated_external_event_from_bus()) {
                panic!("A PolicyHandler has already been registered for this external event");
            }
            debug!("[ExternalEventDispatcher]: register handler {}", item.get_policy_handler_name());
            map.insert(item.get_associated_external_event_from_bus(), item);
        });
        return ExternalEventDispatcher {
            policy_handlers: map
        };
    }

    pub fn get_policy_handler_by_its_external_event(&self, type_id: TypeId) -> Option<&Box<dyn PolicyHandlerInBus>> {
        if let Some(policy_handler) = self.policy_handlers.get(&type_id) {
            return Some(policy_handler);
        }
        None
    }

}

impl ExternalEventBus for ExternalEventDispatcher {
    fn dispatch<'b>(&self, external_event: &'b dyn ExternalEvent) -> Commands {
        if let Option::Some(policy_handler) = self.policy_handlers.get(&external_event.as_any().type_id()) {
            let commands = policy_handler.handle_from_bus(external_event);
            return commands;
        }
        Err(Box::new(NoPolicyHandlerRegisterForGivenExternalEvent {}))
    }
}
