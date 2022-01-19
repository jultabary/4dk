use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event::Event;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::command::secured_command::SecuredCommand;
use crate::dddk::security::command::secured_command_handler::SecuredCommandHandler;
use crate::dddk::security::permission::Permission;

pub struct SecuredCommandDispatcher {
    secured_handler_expected_permission: HashMap<TypeId, Permission>,
    command_handlers: HashMap<TypeId, Box<dyn CommandHandlerInBus>>,
    authorized_strategy: Box<dyn AuthorizedStrategy>,
}

impl SecuredCommandDispatcher {
    pub fn new(given_command_handlers: Vec<Box<dyn CommandHandlerInBus>>, authorized_strategy: Box<dyn AuthorizedStrategy>) -> SecuredCommandDispatcher {
        let mut secured_handler_expected_permission = HashMap::new();
        let mut command_handlers = HashMap::new() as HashMap<TypeId, Box<dyn CommandHandlerInBus>>;
        given_command_handlers.into_iter().for_each(|command_handler| {
            if let Some(secured_command_handler) = command_handler.as_any().downcast_ref::<SecuredCommandHandler>() {
                let associated_command_type_id = secured_command_handler.get_associated_command_from_bus();
                let associated_permission = secured_command_handler.get_associated_permission();
                secured_handler_expected_permission.insert(associated_command_type_id.clone(), associated_permission);
                command_handlers.insert(associated_command_type_id, command_handler);
            } else {
                let associated_command_type_id = command_handler.get_associated_command_from_bus();
                command_handlers.insert(associated_command_type_id, command_handler);
            }
        });
        SecuredCommandDispatcher {
            secured_handler_expected_permission,
            command_handlers,
            authorized_strategy,
        }
    }

    pub fn is_command_handler_restricted(&self, command_type_id: TypeId) -> bool {
        self.secured_handler_expected_permission.get(&command_type_id).is_some()
    }

    pub fn get_command_handler_associated_to_command(&self, command_type_id: TypeId) -> Option<&Box<dyn CommandHandlerInBus>> {
        self.command_handlers.get(&command_type_id)
    }

    fn get_command_handler_from_secured_command(&self, secured_command: &SecuredCommand) -> &dyn CommandHandlerInBus {
        self.get_command_handler_from_command(secured_command.get_command())
    }

    fn get_command_handler_from_command(&self, command: &dyn Command) -> &dyn CommandHandlerInBus {
        let command_type_id = command.as_any().type_id();
        if let Some(command_handler) = self.command_handlers.get(&command_type_id) {
            return command_handler.as_ref();
        }
        panic!("No Handler found");
    }

    fn get_handler_expected_permission(&self, secured_command: &SecuredCommand) -> Permission {
        let command_type_id = secured_command.get_command().as_any().type_id();
        self.secured_handler_expected_permission.get(&command_type_id).unwrap().clone()
    }
}

impl CommandBus for SecuredCommandDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Arc<dyn Event>> {
        return if let Some(secured_command) = command.as_any().downcast_ref::<SecuredCommand>() {
            let command_handler = self.get_command_handler_from_secured_command(secured_command);
            let permission = self.get_handler_expected_permission(secured_command);
            let authorization = self.authorized_strategy.is_authorized(
                permission,
                secured_command.get_roles_names()
            );
            if authorization.is_authorized() {
                return command_handler.handle_from_bus(secured_command.get_command());
            }
            return Vec::new();
        } else {
            let command_handler = self.get_command_handler_from_command(command);
            if let Some(_) = command_handler.as_any().downcast_ref::<SecuredCommandHandler>() {
                return Vec::new();
            }
            command_handler.handle_from_bus(command)
        };
    }
}
