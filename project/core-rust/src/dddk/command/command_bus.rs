use crate::dddk::command::command::Command;
use crate::dddk::event::event::Event;

pub trait CommandBus {
    fn dispatch<'b>(&mut self, command: &'b dyn Command) -> Vec<Box<dyn Event>>;
}