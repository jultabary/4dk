use crate::dddk::command::command::{ACommand};
use crate::dddk::command::command_bus::{CommandBus, CommandDispatcher};
use crate::dddk::command::command_handler::{ACommandHandler, AnotherCommandHandler, CommandHandleInBus};

mod dddk;


fn main() {
    let handler = ACommandHandler {};
    let another_handler = AnotherCommandHandler {};

    let box_handler = Box::new(handler);
    let box_another_handler = Box::new(another_handler);

    let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandleInBus>>;
    command_handlers.push(box_handler);
    command_handlers.push(box_another_handler);

    let another_command = Box::new(ACommand {});
    let command_bus = CommandDispatcher::new(command_handlers);
    command_bus.dispatch(another_command);
}
