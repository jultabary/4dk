use std::sync::Arc;
use crate::dddk::aliases::{Commands, Events, ResponseFromHandler};
use crate::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
use crate::dddk::command::bus_impl::command_logging_middleware::CommandLoggingMiddleware;
use crate::dddk::command::bus_impl::event_produced_by_command_bus_dispatcher::EventsProducedByCommandBusDispatcher;
use crate::dddk::command::command::Command;
use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::command::command_handler::CommandHandlerInBus;
use crate::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
use crate::dddk::event::event_handler::EventHandlerInBus;
use crate::dddk::event::event_handler_logger::encapsulated_event_handler_with_logger;
use crate::dddk::external_event::bus_impl::command_produced_by_external_event_bus_dispatcher::CommandProducedByExternalEventBusDispatcher;
use crate::dddk::external_event::bus_impl::external_event_dispatcher::ExternalEventDispatcher;
use crate::dddk::external_event::bus_impl::external_event_logging_middleware::ExternalEventLoggingMiddleware;
use crate::dddk::external_event::external_event::ExternalEvent;
use crate::dddk::external_event::external_event_bus::ExternalEventBus;
use crate::dddk::external_event::policy_handler::PolicyHandlerInBus;
use crate::dddk::query::bus_impl::query_dispatcher::QueryDispatcher;
use crate::dddk::query::bus_impl::query_logging_middleware::QueryLoggingMiddleware;
use crate::dddk::query::query::Query;
use crate::dddk::query::query_bus::QueryBus;
use crate::dddk::query::query_handler::QueryHandlerInBus;

pub mod dddk;

pub struct Bus {
    external_event_bus: CommandProducedByExternalEventBusDispatcher,
    query_bus: QueryLoggingMiddleware,
}

impl Bus {
    pub fn new(command_handlers: Vec<Box<dyn CommandHandlerInBus>>,
               event_handlers: Vec<Box<dyn EventHandlerInBus>>,
               query_handlers: Vec<Box<dyn QueryHandlerInBus>>,
               policy_handlers: Vec<Box<dyn PolicyHandlerInBus>>) -> Bus {
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let event_handlers_logger = encapsulated_event_handler_with_logger(event_handlers);
        let event_dispatcher = EventDispatcher::new(event_handlers_logger);

        let events_produced_by_command_bus_dispatcher = EventsProducedByCommandBusDispatcher::new(
            Box::new(command_logging_middleware),
            Arc::new(event_dispatcher),
            true,
        );

        let external_event_dispatcher = ExternalEventDispatcher::new(policy_handlers);
        let external_events_bus_logging_middleware = ExternalEventLoggingMiddleware::new(Box::new(external_event_dispatcher));
        let command_produced_by_policy_dispatcher = CommandProducedByExternalEventBusDispatcher::new(
            Box::new(external_events_bus_logging_middleware),
            Box::new(events_produced_by_command_bus_dispatcher)
        );

        let query_dispatcher = QueryDispatcher::new(query_handlers);
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));
        Bus {
            external_event_bus: command_produced_by_policy_dispatcher,
            query_bus: query_logging_middleware,
        }
    }

    pub fn dispatch_external_event(&self, external_event: &dyn ExternalEvent) -> Commands {
        self.external_event_bus.dispatch(external_event)
    }

    pub fn dispatch_command(&self, command: &dyn Command) -> Events {
        self.external_event_bus.get_command_bus().dispatch(command)
    }

    pub fn dispatch_query(&self, query: &dyn Query) -> ResponseFromHandler {
        self.query_bus.dispatch(query)
    }
}

