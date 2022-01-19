use std::any::{Any, TypeId};
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use crate::dddk::security::permission::Permission;

pub struct SecuredCommandHandler {
    command_handler: Box<dyn CommandHandlerInBus>,
    associated_permission: Permission,
}

impl SecuredCommandHandler {
    pub fn new(command_handler: Box<dyn CommandHandlerInBus>, associated_permission: Permission) -> SecuredCommandHandler {
        SecuredCommandHandler {
            command_handler,
            associated_permission,
        }
    }

    pub fn get_associated_permission(&self) -> Permission {
        self.associated_permission.clone()
    }

    pub fn get_command_handler(&self) -> &dyn CommandHandlerInBus {
        self.command_handler.as_ref()
    }
}

impl CommandHandlerInBus for SecuredCommandHandler {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Events {
        self.command_handler.handle_from_bus(command)
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        self.command_handler.get_associated_command_from_bus()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}