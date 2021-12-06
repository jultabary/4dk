use std::any::{Any, TypeId};
use crate::dddk::command::command::{ACommand, Command};
use crate::dddk::event::event::{AEvent, Event};

pub trait CommandHandler<C: Sized + Any + Command> {

    fn handle_from_bus(&self, command: Box<dyn Command>) -> Option<Box<dyn Event>> {
        let cast_command = command.as_any().downcast_ref::<C>();
        if cast_command.is_some() {
            return Option::Some(self.handle(cast_command.unwrap()));
        }
        return Option::None;
    }

    fn handle(&self, command: &C) -> Box<dyn Event>;

    fn get_associated_command(&self) -> TypeId {
        return TypeId::of::<C>()
    }

}

pub struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler{

    fn handle(&self, _command: &ACommand) -> Box<dyn Event> {
        println!("Returning an event");
        return Box::new(AEvent{});
    }

}
