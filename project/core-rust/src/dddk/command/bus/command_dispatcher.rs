use std::any::{TypeId};
use std::collections::HashMap;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandleInBus;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher {
    command_handlers: HashMap<TypeId, Box<dyn CommandHandleInBus>>
}

impl<'a> CommandDispatcher {
    pub fn new(command_handlers: Vec<Box<dyn CommandHandleInBus>>) -> Box<CommandDispatcher> {
        let mut map = HashMap::new();
        command_handlers.into_iter().for_each(| item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return Box::new(CommandDispatcher {
            command_handlers: map
        });
    }
    pub fn get_associated_command_handler(&'a self, type_id: &TypeId) ->  &'a Box<dyn CommandHandleInBus>  {
        return  self.command_handlers.get(type_id).unwrap();
    }

}


impl CommandBus for CommandDispatcher {
    fn dispatch(&mut self, command: Box<dyn Command>) -> Vec<Box<dyn Event>> {
        if let Some(command_handler) = self.command_handlers.get_mut(&command.as_any().type_id()) {
            let events = command_handler.handle_from_bus(command);
            return events;
        }
        return Vec::new();
    }

}
