#[macro_use]
extern crate rocket;
extern crate dddk_core;

use std::any::{Any, TypeId};
use dddk_core::dddk::command::bus::command_bus_injected_with_box::CommandBusParent;
use dddk_core::dddk::command::bus::command_dispatcher_with_box::CommandDispatcher;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::{CommandHandleInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use rocket::State;

struct ValueToInject {
    value: bool,
}

unsafe impl Send for ValueToInject {}

unsafe impl Sync for ValueToInject {}

struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ACommandHandler {}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle<'a>(&mut self, _command: &'a ACommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        return Vec::new();
    }
}

impl CommandHandleInBus for ACommandHandler {
    fn handle_from_bus<'a>(&mut self, command: &'a dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[get("/")]
fn index(value: &State<ValueToInject>, command_bus: &mut State<CommandBusParent>) -> String {
    let command = ACommand {};
    let response = String::from("Hello, world! ") + &value.value.to_string();
    return response;
}

#[rocket::main]
async fn main() {
        let mut command_handler: ACommandHandler = ACommandHandler {};
        let mut command_handlers= Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
        command_handlers.push(Box::new(command_handler));
        let  mut command_dispatcher = CommandDispatcher::new(command_handlers);
        let mut command_bus = CommandBusParent::new(Box::new(command_dispatcher));


        let value = ValueToInject { value: true };
        let _server = rocket::build()
            .manage(value)
            .manage(command_bus)
            .mount("/", routes![index]).launch().await;
}