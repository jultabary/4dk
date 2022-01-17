use std::any::{TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandlerInBus;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher {
    command_handlers: HashMap<TypeId, Box<dyn CommandHandlerInBus>>,
}

impl CommandDispatcher {
    pub fn new(command_handler_values: Vec<Box<dyn CommandHandlerInBus>>) -> CommandDispatcher {
        let mut map = HashMap::new();
        command_handler_values.into_iter().for_each(|item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return CommandDispatcher {
            command_handlers: map
        };
    }

    pub fn get_command_handler_by_its_command(&self, type_id: TypeId) -> Option<&Box<dyn CommandHandlerInBus>> {
        if let Some(command_handler) = self.command_handlers.get(&type_id) {
            return Some(command_handler);
        }
        None
    }
}

impl CommandBus for CommandDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Arc<dyn Event>> {
        if let Option::Some(command_handler) = self.command_handlers.get(&command.as_any().type_id()) {
            let events = command_handler.handle_from_bus(command);
            return events;
        }
        return Vec::new();
    }
}
