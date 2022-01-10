#[macro_use]
extern crate rocket;
extern crate dddk_core;
#[macro_use]
extern crate diesel;

use dddk_core::dddk::command::bus::command_bus_injected_with_box::CommandBusParent;
use dddk_core::dddk::command::bus::command_dispatcher_with_box::CommandDispatcher;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandleInBus;
use dddk_core::dddk::event::event::Event;
use diesel::PgConnection;
use crate::infrastructure::api::get_all_foo;
use crate::infrastructure::database::{establish_connection, FooRepositoryAdapter};
use crate::usecases::a_command_handler::ACommandHandler;

pub mod infrastructure;
pub mod domain;
pub mod usecases;
pub mod schema;

struct App<'a> {
    pg_connection: PgConnection,
    foo_repository: Option<FooRepositoryAdapter<'a>>,
    command_bus: Option<CommandBusParent>
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let pg_connection = establish_connection();
        let mut app = App {
            pg_connection,
            foo_repository: None,
            command_bus: None
        };
        app.init();
        return app;
    }

    fn init(&mut self) {
        let foo_repository = FooRepositoryAdapter::new(&self.pg_connection);
        self.foo_repository = Some(foo_repository);

        // let command_handler = ACommandHandler::new(&foo_repository);
        // let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
        // command_handlers.push(Box::new(command_handler));
        // let command_dispatcher = CommandDispatcher::new(command_handlers);
        // self.command_bus = Option::Some(CommandBusParent::new(Box::new(command_dispatcher)));

    }
}

impl<'a> CommandBus for App<'a> {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        self.command_bus.as_ref().unwrap().dispatch(command)
    }
}

unsafe impl<'a> Sync for App<'a> { }

unsafe impl<'a> Send for App<'a> { }

#[rocket::main]
async fn main() {
    let app = App::new();
    let _server = rocket::build()
        .manage(app)
        .mount("/", routes![get_all_foo]).launch().await;
}
