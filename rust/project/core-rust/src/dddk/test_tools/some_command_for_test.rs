#[cfg(test)]
pub mod some_command_for_tests {
    use std::any::Any;
    use crate::dddk::command::command::Command;

    pub struct ACommand {}

    impl Command for ACommand {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_name(&self) -> String {
            "ACommand".to_string()
        }
    }

    pub struct AnotherCommand {}

    impl Command for AnotherCommand {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_command_name(&self) -> String {
            "AnotherCommand".to_string()
        }
    }
}