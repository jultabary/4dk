use std::any::Any;
use std::fmt::Debug;

pub trait Command: Debug {
    fn as_any (&self) -> &dyn Any;

    fn get_command_name(&self) -> String;
}