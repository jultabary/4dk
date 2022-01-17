use std::sync::Arc;
use crate::dddk::command::command::Command;
use crate::dddk::event::event::Event;

pub trait CommandBus {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Arc<dyn Event>>;
}
