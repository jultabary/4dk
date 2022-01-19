pub mod permission;
pub mod authorization;
pub mod authorized_strategy;
pub mod role;
pub mod errors;
pub mod authorized_strategy_impl {
    pub mod no_security_strategy;
    mod no_security_strategy_test;
    pub mod role_based_strategy;
    mod role_based_strategy_test;
    pub mod role_read_repository;
}
mod test_tools {
    pub mod fake_role_repository;
    pub mod some_secured_command_for_test;
    pub mod some_secured_command_handler_for_test;
    pub mod some_event_for_test;
    pub mod some_role_and_permission_for_test;
}
pub mod command {
    pub mod secured_command;
    mod secured_command_test;
    pub mod secured_command_handler;
    mod secured_command_handler_test;
    pub mod secured_command_dispatcher;
    pub mod secured_command_dispatcher_test;
}