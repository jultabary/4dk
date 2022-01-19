use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CommandDoesNotHaveTheRightPermission {}

impl Display for CommandDoesNotHaveTheRightPermission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command does not have the right permission to execute handler !")
    }
}

impl Error for CommandDoesNotHaveTheRightPermission {}

#[derive(Debug)]
pub struct TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {}

impl Display for TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Try to execute a secured command handler with an unsecured command !")
    }
}

impl Error for TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {}
