use std::any::Any;
use crate::dddk::command::command::Command;

pub struct ACommand { }
impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AnotherCommand { }
impl Command for AnotherCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}