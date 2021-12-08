use std::any::{Any, TypeId};
use crate::dddk::command::bus::command_dispatcher::CommandDispatcher;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use crate::dddk::event::event::Event;

pub mod dddk;

struct Controller<'a> {
    command_bus: &'a mut dyn CommandBus,
}

impl<'a> Controller<'a> {
    fn receive(&mut self) {
        let command = ACommand {};
        self.command_bus.dispatch(&command);
    }
}

struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle<'a>(&mut self, _command: &'a ACommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        return Vec::new();
    }
}

impl CommandHandleInBus for ACommandHandler {
    fn handle_from_bus<'a>(&mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }
}


fn main() {
    let mut command_handler: ACommandHandler = ACommandHandler {};
    let mut command_handlers = Vec::new() as Vec<&mut dyn CommandHandleInBus>;
    command_handlers.push(&mut command_handler);
    let mut command_dispatcher = CommandDispatcher::new(command_handlers);
    let mut controller = Controller { command_bus: &mut command_dispatcher };
    controller.receive();
}
