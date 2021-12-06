use std::any::{Any, TypeId};
use crate::dddk::command::bus::command_dispatcher::CommandDispatcher;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use crate::dddk::event::event::{Event};

mod dddk;


pub struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle(&self, _command: &ACommand) -> Vec<Box<dyn Event>> {
        println!("Returning an event from ACommandHandler");
        let mut events = Vec::new();
        events.push(Box::new(AEvent {}) as Box<dyn Event>);
        return events;
    }
}

impl CommandHandleInBus for ACommandHandler {
    fn handle_from_bus(&self, command: Box<dyn Command>) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }
}

pub struct AnotherCommandHandler {}

impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
    fn handle(&self, _command: &AnotherCommand) -> Vec<Box<dyn Event>> {
        println!("Returning an event from AnotherCommandHandler");
        let mut events = Vec::new();
        events.push(Box::new(AEvent {}) as Box<dyn Event>);
        return events;
    }
}

impl CommandHandleInBus for AnotherCommandHandler {
    fn handle_from_bus(&self, command: Box<dyn Command>) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }
}

pub struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AnotherCommand {}

impl Command for AnotherCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AEvent {}

impl Event for AEvent {

}

fn main() {
    let handler = ACommandHandler {};
    let another_handler = AnotherCommandHandler {};

    let box_handler = Box::new(handler);
    let box_another_handler = Box::new(another_handler);

    let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
    command_handlers.push(box_handler);
    command_handlers.push(box_another_handler);

    let another_command = Box::new(ACommand {});
    let command_bus = CommandDispatcher::new(command_handlers);
    command_bus.dispatch(another_command);
}
