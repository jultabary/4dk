use std::any::TypeId;
use std::collections::HashMap;
use crate::{CommandBus, CommandHandleInBus};
use crate::dddk::command::command::Command;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher {
    command_handlers: HashMap<TypeId, Box<dyn CommandHandleInBus>>
}

impl CommandDispatcher {
    pub fn new(command_handlers: Vec<Box<dyn CommandHandleInBus>>) -> Box<CommandDispatcher> {
        let mut map = HashMap::new();
        command_handlers.into_iter().for_each(|item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return Box::new(CommandDispatcher {
            command_handlers: map
        });
    }
}


impl CommandBus for CommandDispatcher {
    fn dispatch(&self, command: Box<dyn Command>) -> Vec<Box<dyn Event>> {
        let command_handler_opt = self.command_handlers.get(&command.as_any().type_id());
        if command_handler_opt.is_some() {
            return command_handler_opt.unwrap().handle_from_bus(command);
        }
        return Vec::new();
    }
}
