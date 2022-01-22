#[macro_use]
extern crate rocket;
extern crate dddk_security;
#[macro_use]
extern crate diesel;

use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::command::bus_impl::event_produced_by_command_bus_dispatcher::EventsProducedByCommandBusDispatcher;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use dddk_security::dddk::security::authorized_strategy_impl::role_based_strategy::RoleBasedStrategy;
use dddk_security::dddk::security::command::secured_command_dispatcher::SecuredCommandDispatcher;
use dddk_security::dddk::security::command::secured_command_handler::SecuredCommandHandler;
use dddk_security::dddk::security::permission::Permission;
use dddk_security::dddk::security::query::secured_query_dispatcher::SecuredQueryDispatcher;
use dddk_security::dddk::security::query::secured_query_handler::SecuredQueryHandler;
use dddk_security::dddk::security::role::Role;
use crate::infrastructure::database::foo_database::{establish_connection, FooRepositoryAdapter};
use crate::infrastructure::http::routes::{get_all_foo, post_foo};
use crate::infrastructure::http::error_handling::{forbidden, un_authorized};
use crate::infrastructure::rbac::role_fake_database::RoleRepository;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommandHandler;
use crate::usecases::events::foo_created_event::PrintThatFooHasBeenCreatedEventHandler;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQueryHandler;


pub mod infrastructure;
pub mod domain;
pub mod usecases;
pub mod schema;

pub struct Context {
    command_bus: EventsProducedByCommandBusDispatcher,
    query_bus: SecuredQueryDispatcher,
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
        let secured_command_dispatcher = SecuredCommandDispatcher::new(
            command_handlers,
            role_based_strategy.clone(),
        );

        let a_query_handler = SecuredQueryHandler::new(
            Box::new(WhatAreAllTheFoosQueryHandler::new(foo_repository.clone())),
            Permission::new(String::from("permission")),
        );
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(a_query_handler));
        let secured_query_dispatcher = SecuredQueryDispatcher::new(
            query_handlers,
            role_based_strategy.clone(),
        );

        let an_event_handler = Box::new(PrintThatFooHasBeenCreatedEventHandler {});
        let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
        event_handlers.push(an_event_handler);
        let event_dispatcher = EventDispatcher::new(event_handlers);

        let context = Context {
            command_bus: EventsProducedByCommandBusDispatcher::new(
                Box::new(secured_command_dispatcher),
                Arc::new(event_dispatcher),
                true,
            ),
            query_bus: secured_query_dispatcher,
        };
        return context;
    }
}

unsafe impl Sync for Context {}

unsafe impl Send for Context {}

#[rocket::main]
async fn main() {
    let context = Context::new();
    let _server = rocket::build()
        .manage(context)
        .mount("/", routes![get_all_foo, post_foo])
        .register("/", catchers![forbidden, un_authorized])
        .launch().await;
}
