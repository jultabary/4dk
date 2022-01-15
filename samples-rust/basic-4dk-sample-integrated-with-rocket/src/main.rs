#[macro_use]
extern crate rocket;
extern crate dddk_core;
#[macro_use]
extern crate diesel;

use std::sync::Arc;
use dddk_core::dddk::command::bus_impl::command_bus_shared_btw_threads::CommandBusSharedBetweenThreads;
use dddk_core::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event::Event;
use crate::infrastructure::api::get_all_foo;
use crate::infrastructure::database::{establish_connection, FooRepositoryAdapter};
use crate::usecases::a_command_handler::ACommandHandler;
use crate::usecases::another_command_handler::AnotherCommandHandler;

pub mod infrastructure;
pub mod domain;
pub mod usecases;
pub mod schema;

pub struct Context {
    command_bus: CommandBusSharedBetweenThreads
}

impl Context {
    fn new() -> Context {
        // clone a Arc Object doesn't copy the value, it creates a new pointer. See Arc documentation for more detail
        let connection = Arc::new(establish_connection());

        let foo_repository = Arc::new(FooRepositoryAdapter::new(connection.clone()));

        let a_command_handler = ACommandHandler::new(foo_repository.clone());
        let another_command_handler = AnotherCommandHandler::new(foo_repository.clone());

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));
        command_handlers.push(Box::new(another_command_handler));
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let context = Context {
            command_bus: CommandBusSharedBetweenThreads::new(Box::new(command_dispatcher))
        };
        return context;
    }

}

impl CommandBus for Context {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        self.command_bus.dispatch(command)
    }
}

unsafe impl Sync for Context { }
unsafe impl Send for Context { }

#[rocket::main]
async fn main() {
    {
        let context = Context::new();
        let _server = rocket::build()
            .manage(context)
            .mount("/", routes![get_all_foo]).launch().await;
    }
}
