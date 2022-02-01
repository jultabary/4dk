#[macro_use]
extern crate diesel;

use std::cell::RefCell;
use std::rc::Rc;
use actix_web::{App, HttpServer};
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use log::LevelFilter;
use crate::infrastructure::api::routes::{get_all_foo, post_foo};
use crate::infrastructure::database::database_repository::{establish_connection, FooRepositoryAdapter};
use crate::logger::SimpleLogger;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommandHandler;
use crate::usecases::events::foo_created_event::PrintThatFooHasBeenCreatedEventHandler;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQueryHandler;

pub mod usecases;
pub mod infrastructure;
pub mod logger;
pub mod domain;
pub mod schema;

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

        let bus = Bus::new(command_handlers, event_handlers, query_handlers, Vec::new());
        Context {
            bus
        }
    }

    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }
}

static LOGGER: SimpleLogger = SimpleLogger {};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    HttpServer::new(
        || {
            let context = RefCell::new(Context::new());
            App::new()
                .service(get_all_foo)
                .service(post_foo)
                .data(context)
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}