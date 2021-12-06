use std::any::TypeId;
use crate::dddk::command::command::{ACommand, Command};
use crate::dddk::command::command_bus::{CommandBus, CommandDispatcher};
use crate::dddk::command::command_handler::{ACommandHandler, CommandHandler};

mod dddk;


fn main() {
    let handler = ACommandHandler {};
    let type_of = handler.get_associated_command();
    if type_of == TypeId::of::<ACommand>() {
        println!("it is equal");
    } else {
        println!("{:?}", type_of);
        println!("{:?}", TypeId::of::<ACommand>());
        println!("{:?}", TypeId::of::<dyn Command>());
    }
    let command_bus = CommandDispatcher::new(&handler);
    let a_command = Box::new(ACommand {});
    command_bus.dispatch(a_command);
}
