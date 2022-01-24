#[cfg(test)]
pub mod some_secured_command_for_test {
    use std::any::Any;
    use std::fmt::{Debug, Formatter};
    use dddk_core::dddk::command::command::Command;
    use dddk_macro::Command;
    use crate::dddk::security::command::secured_command::SecuredCommand;

    #[derive(Command)]
    pub struct ACommand {}

    impl Debug for ACommand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ACommand")
        }
    }

    impl ACommand {
        pub fn new() -> ACommand {
            ACommand {}
        }
    }

    pub fn get_a_command_secured(roles: Vec<String>) -> SecuredCommand {
        SecuredCommand::new(Box::new(ACommand::new()), roles)
    }

    #[derive(Command)]
    pub struct AnotherCommand {}

    impl Debug for AnotherCommand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnotherCommand")
        }
    }

    impl AnotherCommand {
        pub fn new() -> AnotherCommand {
            AnotherCommand {}
        }
    }
}