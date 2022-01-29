use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CommandIsNotAssociatedWithHandler {}

impl Error for CommandIsNotAssociatedWithHandler {}

impl Display for CommandIsNotAssociatedWithHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given command is not associated with the handler !")
    }
}

#[derive(Debug)]
pub struct NoCommandHandlerRegisterForGivenCommand {}

impl Error for NoCommandHandlerRegisterForGivenCommand {}

impl Display for NoCommandHandlerRegisterForGivenCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No CommandHandler is registered for given command !")
    }
}

#[derive(Debug)]
pub struct QueryIsNotAssociatedWithHandler {}

impl Error for QueryIsNotAssociatedWithHandler {}

impl Display for QueryIsNotAssociatedWithHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given query is not associated with the handler !")
    }
}

#[derive(Debug)]
pub struct NoQueryHandlerRegisterForGivenQuery {}

impl Error for NoQueryHandlerRegisterForGivenQuery {}

impl Display for NoQueryHandlerRegisterForGivenQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No QueryHandler is registered for given query !")
    }
}

#[derive(Debug)]
pub struct ExternalEventIsNotAssociatedWithThisPolicyHandler {}

impl Error for ExternalEventIsNotAssociatedWithThisPolicyHandler {}

impl Display for ExternalEventIsNotAssociatedWithThisPolicyHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "External Event is not associated with this PolicyHandler !")
    }
}

#[derive(Debug)]
pub struct NoPolicyHandlerRegisterForGivenExternalEvent {}

impl Error for NoPolicyHandlerRegisterForGivenExternalEvent {}

impl Display for NoPolicyHandlerRegisterForGivenExternalEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No PolicyHandler is registered for given external event !")
    }
}