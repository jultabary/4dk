use crate::dddk::command::command::Command;
use crate::dddk::event::event::Event;

pub trait CommandBus<'a> {
    fn dispatch(&'a mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>>;
}
