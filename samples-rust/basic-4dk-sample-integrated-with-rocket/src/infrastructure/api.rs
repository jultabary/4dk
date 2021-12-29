use dddk_core::dddk::command::bus::command_bus_injected_with_box::CommandBusParent;
use dddk_core::dddk::command::command_bus::CommandBus;
use rocket::State;
use crate::usecases::a_command_handler::ACommand;

#[get("/")]
pub fn get_all_foo(command_bus: &State<CommandBusParent>) -> String {
    let command = ACommand {};
    command_bus.dispatch(&command);
    let response = String::from("Hello, world!");
    return response;
}