use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event::Event;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
use crate::dddk::security::permission::Permission;

pub struct SecuredCommandDispatcher {
    is_command_handlers_restricted: HashMap<TypeId, Permission>,
    command_handlers: Vec<Box<dyn CommandHandlerInBus>>,
    authorized_strategy: Box<dyn AuthorizedStrategy>,
}

impl SecuredCommandDispatcher {
    pub fn new(command_handlers: Vec<Box<dyn CommandHandlerInBus>>, authorized_strategy: Box<dyn AuthorizedStrategy>) -> SecuredCommandDispatcher {
        let mut is_command_handlers_restricted = HashMap::new();
        command_handlers.iter().for_each(|command_handler| {
           if let Some(secured_command_handler) = command_handler.as_any().downcast_ref::<SecuredCommandHandler>(){
               let associated_command_type_id = secured_command_handler.get_associated_command_from_bus();
               let associated_permission = secured_command_handler.get_associated_permission();
               is_command_handlers_restricted.insert(associated_command_type_id, associated_permission);
           }
        });
        SecuredCommandDispatcher {
            is_command_handlers_restricted,
            command_handlers,
            authorized_strategy
        }
    }

    pub fn is_command_handler_restricted(&self, command_type_id: TypeId) -> bool {
        self.is_command_handlers_restricted.get(&command_type_id).is_some()
    }
}

impl CommandBus for SecuredCommandDispatcher {
    fn dispatch<'b>(&self, _command: &'b dyn Command) -> Vec<Arc<dyn Event>> {
        todo!()
    }
}
