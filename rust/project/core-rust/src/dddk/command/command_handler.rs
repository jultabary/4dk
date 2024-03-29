use std::any::{Any, TypeId};
use crate::dddk::aliases::Events;
use crate::dddk::bus::Bus;
use crate::dddk::command::command::Command;
use crate::dddk::errors::CommandIsNotAssociatedWithHandler;


pub trait CommandHandlerInBus {
    fn handle_from_bus<'a>(&self, command: &'a dyn Command, bus_opt: Option<&'a dyn Bus>) -> Events;

    fn get_associated_command_from_bus(&self) -> TypeId;

    fn as_any(&self) -> &dyn Any;

    fn get_command_handler_name(&self) -> String;
}

pub trait CommandHandler<C: Sized + Any + Command> {
    fn handle_generic_command<'a>(&self, command: &'a dyn Command, bus_opt: Option<&'a dyn Bus>) -> Events {
        let cast_command = command.as_any().downcast_ref::<C>();
        if cast_command.is_some() {
            return self.handle(cast_command.unwrap(), bus_opt);
        }
        return Err(Box::new(CommandIsNotAssociatedWithHandler {}));
    }

    fn handle(&self, command: &C, bus_opt: Option<&dyn Bus>) -> Events;

    fn get_associated_command(&self) -> TypeId {
        return TypeId::of::<C>();
    }
}