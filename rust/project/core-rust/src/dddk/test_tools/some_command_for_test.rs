#[cfg(test)]
pub mod some_command_for_tests {
    use std::any::Any;
    use std::fmt::{Debug, Formatter};
    use crate::dddk::command::command::Command;

    pub struct ACommand {}

    impl Debug for ACommand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ACommand")
        }
    }

    impl Command for ACommand {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_name(&self) -> String {
            "ACommand".to_string()
        }
    }

    pub struct AnotherCommand {}

    impl Debug for AnotherCommand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnotherCommand")
        }
    }

    impl Command for AnotherCommand {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_name(&self) -> String {
            "AnotherCommand".to_string()
        }
    }
}