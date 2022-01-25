use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use dddk_core::dddk::aliases::Events;
use dddk_macro::Command;
use dddk_core::dddk::command::command::Command;
use dddk_macro::CommandHandlerInBus;
use dddk_core::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};

#[derive(Command)]
pub struct CreateFooCommand {
}

impl Debug for CreateFooCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateFooCommand")
    }
}

#[derive(CommandHandlerInBus)]
pub struct CreateFooCommandHandler {}

impl CommandHandler<CreateFooCommand> for CreateFooCommandHandler {
    fn handle(&self, command: &CreateFooCommand) -> Events {
        println!("receive command: [{:?}]", command);
        Ok(Vec::new())
    }
}