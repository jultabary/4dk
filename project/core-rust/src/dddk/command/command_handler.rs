use std::any::{Any, TypeId};
use crate::dddk::command::command::{ACommand, AnotherCommand, Command};
use crate::dddk::event::event::{AEvent, Event};

pub trait CommandHandleInBus {
    fn handle_from_bus(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>>;

    fn get_associated_command_from_bus(&self) -> TypeId;
}

pub trait CommandHandler<C: Sized + Any + Command> {
    fn handle_command(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>> {
        let cast_command = command.as_any().downcast_ref::<C>();
        if cast_command.is_some() {
            return Option::Some(self.handle(cast_command.unwrap()));
        }
        return Option::None;
    }

    fn handle(&self, command: &C) -> Box<dyn Event>;

    fn get_associated_command(&self) -> TypeId {
        return TypeId::of::<C>();
    }
}

pub struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle(&self, _command: &ACommand) -> Box<dyn Event> {
        return Box::new(AEvent {});
    }
}

impl CommandHandleInBus for ACommandHandler {
    fn handle_from_bus(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }
}

pub struct AnotherCommandHandler {}

impl CommandHandler<AnotherCommand> for AnotherCommandHandler {
    fn handle(&self, _command: &AnotherCommand) -> Box<dyn Event> {
        println!("Returning an event from AnotherCommandHandler");
        return Box::new(AEvent {});
    }
}

impl CommandHandleInBus for AnotherCommandHandler {
    fn handle_from_bus(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }
}