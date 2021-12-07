use crate::dddk::command::command::Command;
use crate::dddk::event::event::Event;

pub trait CommandBus {
    fn dispatch(&mut self, command: Box<dyn Command>) -> Vec<Box<dyn Event>>;
}
