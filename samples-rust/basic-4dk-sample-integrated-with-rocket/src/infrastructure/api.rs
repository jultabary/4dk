use dddk_core::dddk::command::command_bus::CommandBus;
use rocket::State;
use crate::{Context};
use crate::usecases::a_command_handler::ACommand;

#[get("/")]
pub fn get_all_foo(command_bus: &State<Context>) -> String {
    let command = ACommand {};
    command_bus.dispatch(&command);
    String::from("Hello, world!")
}