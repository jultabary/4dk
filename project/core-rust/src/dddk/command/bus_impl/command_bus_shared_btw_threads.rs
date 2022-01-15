use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::event::event::Event;

pub struct CommandBusSharedBetweenThreads {
    command_bus: Box<dyn CommandBus>
}

impl CommandBusSharedBetweenThreads {
    pub fn new(command_bus: Box<dyn CommandBus>) -> CommandBusSharedBetweenThreads {
        return CommandBusSharedBetweenThreads {
          command_bus
        };
    }
}

impl CommandBus for CommandBusSharedBetweenThreads {
    fn dispatch(&self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.command_bus.dispatch(command);
    }
}

unsafe impl Sync for CommandBusSharedBetweenThreads { }

unsafe impl Send for CommandBusSharedBetweenThreads { }