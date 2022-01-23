#[macro_use]
extern crate rocket;
extern crate dddk_core;
#[macro_use]
extern crate diesel;

use std::rc::Rc;
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use log::LevelFilter;
use crate::infrastructure::api::{get_all_foo, post_foo};
use crate::infrastructure::database::{establish_connection, FooRepositoryAdapter};
use crate::logger::SimpleLogger;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommandHandler;
use crate::usecases::events::foo_created_event::PrintThatFooHasBeenCreatedEventHandler;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQueryHandler;

pub mod infrastructure;
pub mod domain;
pub mod usecases;
pub mod schema;
pub mod logger;

pub struct Context {
    bus: Bus,
}

impl Context {
    pub fn new() -> Context {
        // clone a Rc smart pointer doesn't copy the value, it creates a new pointer. See Rc and Arc documentation for more detail
        let connection = Rc::new(establish_connection());

        let foo_repository = Rc::new(FooRepositoryAdapter::new(connection.clone()));

        let a_command_handler = CreateFooCommandHandler::new(foo_repository.clone());
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));

        let an_event_handler = Box::new(PrintThatFooHasBeenCreatedEventHandler {});
        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
        event_handlers.push(an_event_handler);

        let a_query_handler = WhatAreAllTheFoosQueryHandler::new(foo_repository.clone());
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler));

        let bus = Bus::new(command_handlers, event_handlers, query_handlers);
        Context {
            bus
        }
    }

    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }
}

unsafe impl Sync for Context {}

unsafe impl Send for Context {}

static LOGGER: SimpleLogger = SimpleLogger {};

#[rocket::main]
async fn main() {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    let context = Context::new();
    let _server = rocket::build()
        .manage(context)
        .mount("/", routes![get_all_foo, post_foo]).launch().await;
}
