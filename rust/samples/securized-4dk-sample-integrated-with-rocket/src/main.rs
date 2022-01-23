#[macro_use]
extern crate rocket;
extern crate dddk_security;
#[macro_use]
extern crate diesel;

use std::rc::Rc;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use dddk_security::dddk::security::authorized_strategy_impl::role_based_strategy::RoleBasedStrategy;
use dddk_security::dddk::security::command::secured_command_handler::SecuredCommandHandler;
use dddk_security::dddk::security::permission::Permission;
use dddk_security::dddk::security::query::secured_query_handler::SecuredQueryHandler;
use dddk_security::dddk::security::role::Role;
use dddk_security::SecuredBus;
use log::LevelFilter;
use crate::infrastructure::database::foo_database::{establish_connection, FooRepositoryAdapter};
use crate::infrastructure::http::routes::{get_all_foo, post_foo};
use crate::infrastructure::http::error_handling::{forbidden, un_authorized};
use crate::infrastructure::rbac::role_fake_database::RoleRepository;
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
    bus: SecuredBus,
}

impl Context {
    fn new() -> Context {
        // clone a Rc smart pointer doesn't copy the value, it creates a new pointer. See Rc and Arc documentation for more detail
        let connection = Rc::new(establish_connection());

        let mut roles = Vec::new();
        let permission = Permission::new(String::from("permission"));
        let role = Role::new(String::from("role"), vec![permission]);
        roles.push(role);
        let role_based_strategy = Rc::new(
            RoleBasedStrategy::new(
                Rc::new(RoleRepository::new(roles))
            )
        );

        let foo_repository = Rc::new(FooRepositoryAdapter::new(connection.clone()));

        let a_command_handler = SecuredCommandHandler::new(
            Box::new(CreateFooCommandHandler::new(foo_repository.clone())),
            Permission::new(String::from("permission")),
        );
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));

        let a_query_handler = SecuredQueryHandler::new(
            Box::new(WhatAreAllTheFoosQueryHandler::new(foo_repository.clone())),
            Permission::new(String::from("permission")),
        );
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler));

        let an_event_handler = Box::new(PrintThatFooHasBeenCreatedEventHandler {});
        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
        event_handlers.push(an_event_handler);

        let secured_bus = SecuredBus::new(command_handlers, event_handlers, query_handlers, role_based_strategy);
        Context {
            bus: secured_bus
        }
    }

    pub fn get_bus(&self) -> &SecuredBus {
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
        .mount("/", routes![get_all_foo, post_foo])
        .register("/", catchers![forbidden, un_authorized])
        .launch().await;
}
