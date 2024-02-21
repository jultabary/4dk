use crate::CommandBus;
use crate::dddk::aliases::Commands;
use crate::dddk::external_event::external_event::ExternalEvent;
use crate::dddk::external_event::external_event_bus::ExternalEventBus;

pub struct CommandProducedByExternalEventBusDispatcher {
    external_event_bus: Box<dyn ExternalEventBus>,
    command_bus: Box<dyn CommandBus>,
}

impl CommandProducedByExternalEventBusDispatcher {
    pub fn new(external_event_bus: Box<dyn ExternalEventBus>,
               command_bus: Box<dyn CommandBus>) -> CommandProducedByExternalEventBusDispatcher {
        CommandProducedByExternalEventBusDispatcher {
            external_event_bus,
            command_bus,
        }
    }

    pub fn get_command_bus(&self) -> &Box<dyn CommandBus> {
        &self.command_bus
    }
}

impl ExternalEventBus for CommandProducedByExternalEventBusDispatcher {
    fn dispatch(&self, external_event: &dyn ExternalEvent) -> Commands {
        let commands = self.external_event_bus.dispatch(external_event);
        if commands.is_err() {
            return commands;
        }
        let commands = commands.unwrap();
        commands.iter()
            .for_each(|command| {
                let _events = self.command_bus.dispatch(command.as_ref(), None);
            });
        return Ok(commands);
    }

}