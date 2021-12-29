use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event::Event;

pub struct CommandBusParent<'a> {
    command_bus: &'a mut dyn CommandBus
}

impl <'a>CommandBusParent<'a> {
    pub fn new(command_bus: &'a mut dyn CommandBus) -> CommandBusParent {
        return CommandBusParent {
          command_bus
        };
    }
}

impl <'a>CommandBus for CommandBusParent<'a> {
    fn dispatch<'b>(&mut self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        return self.command_bus.dispatch(command);
    }
}

unsafe impl <'a>Sync for CommandBusParent<'a> { }

unsafe impl <'a>Send for CommandBusParent<'a> { }