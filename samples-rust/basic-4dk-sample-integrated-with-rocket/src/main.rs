#[macro_use]
extern crate rocket;
extern crate dddk_core;

use dddk_core::dddk::command::bus::command_bus_injected_with_box::CommandBusParent;
use dddk_core::dddk::command::bus::command_dispatcher_with_box::CommandDispatcher;
use dddk_core::dddk::command::command_handler::CommandHandleInBus;
use crate::infrastructure::api::get_all_foo;
use crate::usecases::a_command_handler::ACommandHandler;

pub mod infrastructure;
pub mod domain;
pub mod usecases;

#[rocket::main]
async fn main() {
        let command_handler = ACommandHandler {};
        let mut command_handlers= Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
        command_handlers.push(Box::new(command_handler));
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let command_bus = CommandBusParent::new(Box::new(command_dispatcher));

     let _server = rocket::build()
            .manage(command_bus)
            .mount("/", routes![get_all_foo]).launch().await;
}