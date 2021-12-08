use std::any::{TypeId};
use std::collections::HashMap;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandleInBus;
use crate::dddk::event::event::Event;

pub struct CommandDispatcher<'a> {
    command_handlers: HashMap<TypeId, &'a mut dyn CommandHandleInBus<'a>>
}

impl <'a>CommandDispatcher<'a> {
    pub fn new(command_handlers_vec: Vec<&'a mut dyn CommandHandleInBus<'a>>) -> Box<CommandDispatcher <'a>> {
        let mut map = HashMap::new();
        command_handlers_vec.into_iter().for_each(| item| {
            map.insert(item.get_associated_command_from_bus(), item);
        });
        return Box::new(CommandDispatcher {
            command_handlers: map
        });
    }

}


impl <'a>CommandBus<'a> for CommandDispatcher<'a> {
    fn dispatch(&mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        if let Some(command_handler) = self.command_handlers.get_mut(&command.as_any().type_id()) {
            let events = command_handler.handle_from_bus(command);
            return events;
        }
        return Vec::new();
    }

}
