use crate::dddk::command::command::{Command};
use crate::dddk::command::command_handler::CommandHandler;
use crate::dddk::command::command_handler::ACommandHandler;
use crate::dddk::event::event::Event;

pub trait CommandBus {
    fn dispatch(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>>;
}

pub struct CommandDispatcher<'a>{
    command_handler: &'a ACommandHandler
}

impl<'a> CommandDispatcher<'a> {
    pub fn new(command_handler: &'a ACommandHandler) -> Box<CommandDispatcher> {
        return Box::new(CommandDispatcher {
            command_handler
        });
    }
}
impl<'a> CommandBus for CommandDispatcher<'a> {
    fn dispatch(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>> {
        return self.command_handler.handle_from_bus(command);
    }
}
