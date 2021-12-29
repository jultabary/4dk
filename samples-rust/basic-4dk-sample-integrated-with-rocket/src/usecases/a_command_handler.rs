use std::any::{Any, TypeId};
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;

pub struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle<'a>(&self, _command: &'a ACommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        return Vec::new();
    }
}

impl CommandHandleInBus for ACommandHandler {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
