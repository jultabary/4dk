use std::any::Any;
use std::fmt::{Debug, Formatter};
use dddk_core::dddk::command::command::Command;

pub struct SecuredCommand {
    command: Box<dyn Command>,
    role_names: Vec<String>,
}

impl SecuredCommand {
    pub fn new(command: Box<dyn Command>, role_names: Vec<String>) -> SecuredCommand {
        SecuredCommand {
            command,
            role_names,
        }
    }

    pub fn get_command(&self) -> &dyn Command {
        self.command.as_ref()
    }

    pub fn get_roles_names(&self) -> &Vec<String> {
        &self.role_names
    }
}

impl Debug for SecuredCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.command.fmt(f)
    }
}

impl Command for SecuredCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_command_name(&self) -> String {
        self.command.get_command_name()
    }
}
