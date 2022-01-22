use std::any::{Any, TypeId};
use dddk_core::dddk::aliases::Responses;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use crate::dddk::security::permission::Permission;

pub struct SecuredQueryHandler {
    query_handler: Box<dyn QueryHandlerInBus>,
    associated_permission: Permission,
}

impl SecuredQueryHandler {
    pub fn new(command_handler: Box<dyn QueryHandlerInBus>, associated_permission: Permission) -> SecuredQueryHandler {
        SecuredQueryHandler {
            query_handler: command_handler,
            associated_permission,
        }
    }

    pub fn get_associated_permission(&self) -> Permission {
        self.associated_permission.clone()
    }

    pub fn get_query_handler(&self) -> &dyn QueryHandlerInBus {
        self.query_handler.as_ref()
    }
}

impl QueryHandlerInBus for SecuredQueryHandler {
    fn handle_from_bus<'a>(&self, command: &'a dyn Query) -> Responses {
        self.query_handler.handle_from_bus(command)
    }

    fn get_associated_query_from_bus(&self) -> TypeId {
        self.query_handler.get_associated_query_from_bus()
    }

    fn get_query_handler_name(&self) -> String {
        self.query_handler.get_query_handler_name()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}