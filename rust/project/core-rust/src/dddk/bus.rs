use crate::dddk::command::command_bus::CommandBus;
use crate::dddk::query::query_bus::QueryBus;

pub trait Bus: CommandBus + QueryBus {}