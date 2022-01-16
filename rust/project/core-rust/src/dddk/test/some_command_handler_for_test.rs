use std::any::{Any, TypeId};
use crate::dddk::command::command::Command;
use crate::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};
use crate::dddk::event::event::Event;
use crate::dddk::test::some_command_for_test::{ACommand, AnotherCommand};
use crate::dddk::test::some_event_for_test::{AnEvent, AnotherEvent};

pub struct ACommandHandler { }
impl ACommandHandler {
    pub fn new() -> ACommandHandler {
        ACommandHandler { }
    }
}
impl CommandHandler<ACommand> for ACommandHandler {
    fn handle(&self, _command: &ACommand) -> Vec<Box<dyn Event>> {
        vec![Box::new(AnEvent::new())]
    }
}
impl CommandHandlerInBus for ACommandHandler {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        self.handle_generic_command(command)
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        self.get_associated_command()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AnotherCommandHandler { }
impl CommandHandlerInBus for AnotherCommandHandler {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        self.handle_generic_command(command)
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        self.get_associated_command()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl AnotherCommandHandler {
    pub fn new() -> AnotherCommandHandler {
        AnotherCommandHandler { }
    }
}
impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
    fn handle(&self, _command: &AnotherCommand) -> Vec<Box<dyn Event>> {
        vec![Box::new(AnotherEvent::new())]
    }
}