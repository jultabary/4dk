use std::any::{TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandleInBus;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher {
    command_handlers: HashMap<TypeId, Box<dyn CommandHandleInBus>>,
}

impl CommandDispatcher {
    pub fn new(command_handler_values: Vec<Box<dyn CommandHandleInBus>>) -> CommandDispatcher {
        let mut map = HashMap::new();
        command_handler_values.into_iter().for_each(|item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return CommandDispatcher {
            command_handlers: map
        };
    }

    pub fn get_command_handler_by_its_command(&self, type_id: TypeId) -> Option<&Box<dyn CommandHandleInBus>> {
        if let Some(command_handler) = self.command_handlers.get(&type_id) {
            return Some(command_handler);
        }
        None
    }
}

impl CommandBus for CommandDispatcher {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        if let Option::Some(command_handler) = self.command_handlers.get(&command.as_any().type_id()) {
            let events = command_handler.handle_from_bus(command);
            return events;
        }
        return Vec::new();
    }
}

#[derive(Debug)]
pub struct CommandHandlerNotFound {}

impl Display for CommandHandlerNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandHandler Not Found")
    }
}

impl Error for CommandHandlerNotFound {}