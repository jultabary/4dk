use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Forbidden {}

impl Display for Forbidden {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command does not have the right permission to execute handler !")
    }
}

impl Error for Forbidden {}

#[derive(Debug)]
pub struct TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {}

impl Display for TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Try to execute a secured command handler with an unsecured command !")
    }
}

impl Error for TryToExecuteASecuredCommandHandlerWithAnUnSecuredCommand {}

#[derive(Debug)]
pub struct TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery {}

impl Display for TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Try to execute a secured query handler with an unsecured query !")
    }
}

impl Error for TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery {}
