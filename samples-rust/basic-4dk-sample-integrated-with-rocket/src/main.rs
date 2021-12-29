#[macro_use]
extern crate rocket;
extern crate dddk_core;

use std::any::{Any, TypeId};
use dddk_core::dddk::command::bus::command_dispatcher::CommandDispatcher;
use dddk_core::dddk::command::bus::command_bus_injected::CommandBusParent;
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
}

#[get("/")]
fn index(value: &State<ValueToInject>) -> String {
    let response = String::from("Hello, world! ") + &value.value.to_string();
    return response;
}

#[rocket::main]
async fn main() {
        let mut command_handler: ACommandHandler = ACommandHandler {};
        let mut command_handlers: Vec<&mut dyn CommandHandleInBus> = Vec::new() as Vec<&mut dyn CommandHandleInBus>;
        let command_handler_ref: &mut ACommandHandler = &mut command_handler;
        command_handlers.push(command_handler_ref);
        let  mut command_dispatcher: CommandDispatcher = CommandDispatcher::new(command_handlers);
        let mut command_bus: CommandBusParent = CommandBusParent::new(&mut command_dispatcher);

        let command_bus_ref: &mut CommandBusParent = &mut command_bus;

        let value = ValueToInject { value: true };
        let _server = rocket::build()
            .manage(value)
            .manage(command_bus_ref)
            .mount("/", routes![index]).launch().await;
}