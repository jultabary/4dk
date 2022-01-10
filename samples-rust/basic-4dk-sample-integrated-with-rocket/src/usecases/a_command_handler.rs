use std::any::{Any, TypeId};
use better_any::{Tid, TidAble};
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use crate::domain::foo::FooRepository;

pub struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Tid)]
pub struct ACommandHandler<'a> {
    foo_repository: &'a dyn FooRepository
}

trait CommandHandlerFake {
    fn as_tid_fake(&self)-> &dyn Tid;
}

impl<'a> ACommandHandler<'a> {
    pub fn new(foo_repository: &'a dyn FooRepository) -> ACommandHandler {
        ACommandHandler {
            foo_repository
        }
    }
}

impl <'a>CommandHandler<ACommand> for ACommandHandler<'a> {
    fn handle(&self, _command: &ACommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        return Vec::new();
    }
}

impl<'a> CommandHandleInBus for ACommandHandler<'a> {
    fn handle_from_bus(&self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_tid(&self) -> &'a dyn Tid {
        self
    }
}
