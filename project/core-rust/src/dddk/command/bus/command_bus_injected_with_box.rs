use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event::Event;

pub struct CommandBusParent {
    command_bus: Box<dyn CommandBus>
}

impl CommandBusParent {
    pub fn new(command_bus: Box<dyn CommandBus>) -> CommandBusParent {
        return CommandBusParent {
          command_bus
        };
    }
}

impl CommandBus for CommandBusParent {
    fn dispatch(&mut self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.command_bus.dispatch(command);
    }
}

unsafe impl Sync for CommandBusParent { }

unsafe impl Send for CommandBusParent { }