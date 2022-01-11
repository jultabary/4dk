use std::any::{Any, TypeId};
use better_any::{Tid, TidAble};
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use crate::domain::foo::FooRepository;

pub struct AnotherCommand {}

impl Command for AnotherCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Tid)]
pub struct AnotherCommandHandler {
    foo_repository: Box<dyn FooRepository>
}

impl AnotherCommandHandler {
    pub fn new(foo_repository: Box<dyn FooRepository>) -> AnotherCommandHandler {
        AnotherCommandHandler {
            foo_repository
        }
    }
}

impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
    fn handle(&self, _command: &AnotherCommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        return Vec::new();
    }
}

impl CommandHandleInBus for AnotherCommandHandler {
    fn handle_from_bus(&self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_tid(&self) -> &dyn Tid {
        self
    }
}
