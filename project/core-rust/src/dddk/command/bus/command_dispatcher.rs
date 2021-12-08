use std::any::{TypeId};
use std::collections::HashMap;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandleInBus;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher<'a> {
    command_handlers: HashMap<TypeId, &'a mut dyn CommandHandleInBus>
}

impl <'a>CommandDispatcher<'a> {
    pub fn new(command_handlers_vec: Vec<&'a mut dyn CommandHandleInBus>) -> CommandDispatcher {
        let mut map = HashMap::new();
        command_handlers_vec.into_iter().for_each(| item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return CommandDispatcher {
            command_handlers: map
        };
    }

}


impl <'a>CommandBus for CommandDispatcher<'a> {
    fn dispatch<'b>(&mut self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        if let Some(command_handler) = self.command_handlers.get_mut(&command.as_any().type_id()) {
            let events = command_handler.handle_from_bus(command);
            return events;
        }
        return Vec::new();
    }

}
