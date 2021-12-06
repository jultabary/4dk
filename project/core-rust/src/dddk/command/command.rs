use std::any::Any;

pub trait Command {
    fn as_any (&self) -> &dyn Any;
}

pub struct ACommand {}
impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
