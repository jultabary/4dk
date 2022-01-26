use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::{Events, ResponseFromHandler};
use dddk_core::dddk::command::bus_impl::command_logging_middleware::CommandLoggingMiddleware;
use dddk_core::dddk::command::bus_impl::event_produced_by_command_bus_dispatcher::EventsProducedByCommandBusDispatcher;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::command::secured_command_dispatcher::SecuredCommandDispatcher;
use crate::dddk::security::query::secured_query_dispatcher::SecuredQueryDispatcher;
use dddk_core::dddk::event::event_handler_logger::encapsulated_event_handler_with_logger;
use dddk_core::dddk::query::bus_impl::query_logging_middleware::QueryLoggingMiddleware;
use dddk_core::dddk::query::query_bus::QueryBus;


pub mod dddk {
    pub mod security;
}

pub struct SecuredBus {
    command_bus: EventsProducedByCommandBusDispatcher,
    query_bus: QueryLoggingMiddleware,
}

impl SecuredBus {
    pub fn new(command_handlers: Vec<Box<dyn CommandHandlerInBus>>,
               event_handlers: Vec<Box<dyn EventHandlerInBus>>,
               query_handlers: Vec<Box<dyn QueryHandlerInBus>>,
               authorized_strategy: Rc<dyn AuthorizedStrategy>) -> SecuredBus {
        let command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            authorized_strategy.clone(),
        );
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let event_handlers_logger = encapsulated_event_handler_with_logger(event_handlers);
        let event_dispatcher = EventDispatcher::new(event_handlers_logger);

        let events_produced_by_command_bus_dispatcher = EventsProducedByCommandBusDispatcher::new(
            Box::new(command_logging_middleware),
            Arc::new(event_dispatcher),
            true,
        );

        let query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            authorized_strategy,
        );
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));
        SecuredBus {
            command_bus: events_produced_by_command_bus_dispatcher,
            query_bus: query_logging_middleware,
        }
    }

    pub fn dispatch_command(&self, command: &dyn Command) -> Events {
        self.command_bus.dispatch(command)
    }

    pub fn dispatch_query(&self, query: &dyn Query) -> ResponseFromHandler {
        self.query_bus.dispatch(query)
    }
}
