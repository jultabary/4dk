use dddk_core::dddk::command::command_bus::CommandBus;
use rocket::State;
use crate::App;
use crate::usecases::a_command_handler::ACommand;

#[get("/")]
pub fn get_all_foo(command_bus: &State<App>) -> String {
    let command = ACommand {};
    command_bus.dispatch(&command);
    let response = String::from("Hello, world!");
    return response;
}