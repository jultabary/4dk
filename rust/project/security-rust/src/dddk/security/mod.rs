pub mod permission;
pub mod authorization;
pub mod authorized_strategy;
pub mod role;
pub mod authorized_strategy_impl {
    pub mod no_security_strategy;
    mod no_security_strategy_test;
    pub mod role_based_strategy;
    mod role_based_strategy_test;
    pub mod role_read_repository;
}
mod test_tools {
    pub mod fake_role_role_repository;
}