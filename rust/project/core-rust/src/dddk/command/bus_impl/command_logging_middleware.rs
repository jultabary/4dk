use log::{error, info};
use crate::dddk::aliases::Events;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;

pub struct CommandLoggingMiddleware {
    command_bus: Box<dyn CommandBus>,
}

impl CommandLoggingMiddleware {
    pub fn new(command_bus: Box<dyn CommandBus>) -> CommandLoggingMiddleware {
        CommandLoggingMiddleware {
            command_bus
        }
    }
}

impl CommandBus for CommandLoggingMiddleware {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Events {
        info!("Dispatching a command [{}] [{:?}].",
            command.get_command_name(),
            command);
        let events = self.command_bus.dispatch(command);
        if events.is_err() {
            let error = events.err().unwrap();
            let error_message = error.to_string();
            error!(
                "An error has occurred when dispatching command [{}] [{:?}]: {}",
                command.get_command_name(),
                command,
                error_message
            );
            return Err(error);
        }
        let events = events.unwrap();
        let mut event_names = String::new();
        events.iter()
            .for_each(|event| {
                event_names.push_str(event.get_event_name().as_str());
                event_names.push_str(" ");
            });
        info!(
            "Command[{}] [{:?}] has been handled and has produced [{}] events [{}].",
            command.get_command_name(),
            command,
            events.len(),
            event_names
        );
        return Ok(events);
    }
}

