#[macro_use]
extern crate dddk_macro;
#[macro_use]
extern crate rocket;

mod logger;
mod infrastructure;
mod usecases;
mod domain;

use std::env;
use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::{Events, ResponseFromHandler};
use dddk_core::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
use dddk_core::dddk::command::bus_impl::command_logging_middleware::CommandLoggingMiddleware;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::query::bus_impl::query_dispatcher::QueryDispatcher;
use dddk_core::dddk::query::bus_impl::query_logging_middleware::QueryLoggingMiddleware;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_bus::QueryBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use futures::executor::block_on;
use log::LevelFilter;
use rocket::routes;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::domain::task_repository::TaskRepository;
use crate::logger::SimpleLogger;
use crate::infrastructure::api::route_http::{create_task, get_all_task, delete_task, pass_task_to_done};
use crate::infrastructure::api::error_handling::not_found;
use crate::infrastructure::persistence::database_in_memory::DatabaseInMemory;
use crate::infrastructure::persistence::persistence_middleware::PersistenceMiddleware;
use crate::infrastructure::persistence::task_repository_sqlx_impl::TaskRepositorySqlXImpl;
use crate::usecases::command::create_task::CreateTask;
use crate::usecases::command::delete_task::DeleteTask;
use crate::usecases::command::pass_task_to_done::PassTaskToDone;
use crate::usecases::query::what_are_all_the_tasks::WhatAreAllTheTasks;


pub struct CustomBus {
    command_bus: Box<dyn CommandBus>,
    query_bus: QueryLoggingMiddleware,
}

impl CustomBus {
    pub fn new_for_prod(command_handlers: Vec<Box<dyn CommandHandlerInBus>>,
               query_handlers: Vec<Box<dyn QueryHandlerInBus>>,
               pool: Arc<Pool<Postgres>>) -> CustomBus {
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let command_persistence_middleware = PersistenceMiddleware::new(pool, Box::new(command_dispatcher));
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_persistence_middleware));

        let query_dispatcher = QueryDispatcher::new(query_handlers);
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));
        CustomBus {
            command_bus: Box::new(command_logging_middleware),
            query_bus: query_logging_middleware,
        }
    }

    pub fn new(command_handlers: Vec<Box<dyn CommandHandlerInBus>>,
                        query_handlers: Vec<Box<dyn QueryHandlerInBus>>) -> CustomBus {
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let command_logging_middleware = CommandLoggingMiddleware::new(Box::new(command_dispatcher));

        let query_dispatcher = QueryDispatcher::new(query_handlers);
        let query_logging_middleware = QueryLoggingMiddleware::new(Box::new(query_dispatcher));
        CustomBus {
            command_bus: Box::new(command_logging_middleware),
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


pub struct App {
    bus: CustomBus,
    task_repository: Rc<dyn TaskRepository>,
}

impl App {
    fn new(prod: bool) -> App {
        let task_repository = App::init_task_repository(prod);
        App {
            bus: App::init_bus(prod, task_repository.clone()),
            task_repository,
        }
    }
    pub fn bus(&self) -> &CustomBus {
        &self.bus
    }
    pub fn task_repository(&self) -> &Rc<dyn TaskRepository> {
        &self.task_repository
    }

    fn init_bus(prod: bool, task_repository: Rc<dyn TaskRepository>) -> CustomBus {
        match prod {
            true => {
                let repository = task_repository.as_ref().as_any().downcast_ref::<TaskRepositorySqlXImpl>().unwrap();
                let pool = repository.pool();
                let mut command_handlers = Vec::<Box<dyn CommandHandlerInBus>>::new();
                command_handlers.push(Box::new(CreateTask::new(task_repository.clone())));
                command_handlers.push(Box::new(DeleteTask::new(task_repository.clone())));
                command_handlers.push(Box::new(PassTaskToDone::new(task_repository.clone())));
                let mut query_handlers = Vec::<Box<dyn QueryHandlerInBus>>::new();
                query_handlers.push(Box::new(WhatAreAllTheTasks::new(task_repository.clone())));
                CustomBus::new_for_prod(command_handlers, query_handlers, pool)
            }
            false => {
                let mut command_handlers = Vec::<Box<dyn CommandHandlerInBus>>::new();
                command_handlers.push(Box::new(CreateTask::new(task_repository.clone())));
                command_handlers.push(Box::new(DeleteTask::new(task_repository.clone())));
                command_handlers.push(Box::new(PassTaskToDone::new(task_repository.clone())));
                let mut query_handlers = Vec::<Box<dyn QueryHandlerInBus>>::new();
                query_handlers.push(Box::new(WhatAreAllTheTasks::new(task_repository.clone())));
                CustomBus::new(command_handlers, query_handlers)
            }
        }

    }
    fn init_task_repository(prod: bool) -> Rc<dyn TaskRepository> {
        match prod {
            true => {
                let database_url_result = env::var("DATABASE_URL");
                if database_url_result.is_err() {
                    panic!("DATABASE_URL env var has not been set");
                }
                let pool = block_on(PgPoolOptions::new().max_connections(5).connect(&database_url_result.unwrap())).unwrap();
                let pool_arc = Arc::new(pool);
                Rc::new(TaskRepositorySqlXImpl::new(pool_arc.clone())) as Rc<dyn TaskRepository>
            }
            false => {
                Rc::new(DatabaseInMemory::_new()) as Rc<dyn TaskRepository>
            }
        }
    }
}

impl CommandBus for App {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Events {
        self.bus.dispatch_command(command)
    }
}

impl QueryBus for App {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> ResponseFromHandler {
        self.bus.dispatch_query(query)
    }
}

unsafe impl Sync for App {}

unsafe impl Send for App {}

static LOGGER: SimpleLogger = SimpleLogger {};

#[rocket::main]
async fn main() {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    let app = App::new(true);
    let _server = rocket::build()
        .manage(app)
        .register("/", catchers![not_found])
        .mount("/", routes![create_task, get_all_task, delete_task, pass_task_to_done]).launch().await;
}
