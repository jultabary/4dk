use log::{error, info};
use crate::dddk::aliases::Commands;
use crate::dddk::external_event::external_event::ExternalEvent;
use crate::dddk::external_event::external_event_bus::ExternalEventBus;

pub struct ExternalEventLoggingMiddleware {
    external_event_bus: Box<dyn ExternalEventBus>,
}

impl ExternalEventLoggingMiddleware {
    pub fn new(external_event_bus: Box<dyn ExternalEventBus>) -> ExternalEventLoggingMiddleware {
        ExternalEventLoggingMiddleware {
            external_event_bus
        }
    }
}

impl ExternalEventBus for ExternalEventLoggingMiddleware {
    fn dispatch<'b>(&self, external_event: &'b dyn ExternalEvent) -> Commands {
        info!("Dispatching an external event [{}] [{:?}].",
            external_event.get_external_event_name(),
            external_event);
        let commands = self.external_event_bus.dispatch(external_event);
        if commands.is_err() {
            let error = commands.err().unwrap();
            let error_message = error.to_string();
            error!(
                "An error has occurred when dispatching external event [{}] [{:?}]: {}",
                external_event.get_external_event_name(),
                external_event,
                error_message
            );
            return Err(error);
        }
        let commands = commands.unwrap();
        let mut command_names = String::new();
        commands.iter()
            .for_each(|command| {
                command_names.push_str(command.get_command_name().as_str());
                command_names.push_str(" ");
            });
        info!(
            "ExternalEvent[{}] [{:?}] has been handled and has produced [{}] commands [{}].",
            external_event.get_external_event_name(),
            external_event,
            commands.len(),
            command_names
        );
        return Ok(commands);
    }
}

