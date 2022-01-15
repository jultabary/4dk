use std::any::{Any, TypeId};
use crate::dddk::command::command::Command;
use crate::dddk::event::event::{Event};


pub trait CommandHandlerInBus: Send {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>>;

    fn get_associated_command_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;

}

pub trait CommandHandler<C: Sized + Any + Command> {
    fn handle_generic_command<'a>(&self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        let mut events = Vec::new() as Vec<Box<dyn Event>>;
        let cast_command = command.as_any().downcast_ref::<C>();
        if cast_command.is_some() {
            events = self.handle(cast_command.unwrap());
        }
        return events;
    }

    fn handle(&self, command: &C) -> Vec<Box<dyn Event>>;

    fn get_associated_command(&self) -> TypeId {
        return TypeId::of::<C>();
    }
}