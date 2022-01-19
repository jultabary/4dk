use crate::dddk::aliases::Events;
use crate::dddk::command::command::Command;

pub trait CommandBus {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Events;
}
