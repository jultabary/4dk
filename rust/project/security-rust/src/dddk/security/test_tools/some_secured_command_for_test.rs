#[cfg(test)]
pub mod some_secured_command_for_test {
    use std::any::Any;
    use dddk_core::dddk::command::command::Command;
    use crate::dddk::security::command::secured_command::SecuredCommand;

    pub struct ACommand {}

    impl ACommand {
        pub fn new() -> ACommand {
            ACommand {}
        }
    }

    impl Command for ACommand {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub fn get_a_command_secured(roles: Vec<String>) -> SecuredCommand {
        SecuredCommand::new(Box::new(ACommand::new()), roles)
    }

    pub struct AnotherCommand {}

    impl AnotherCommand {
        pub fn new() -> AnotherCommand {
            AnotherCommand {}
        }
    }

    impl Command for AnotherCommand {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}