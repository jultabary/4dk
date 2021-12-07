pub mod command {
    pub mod command_bus;
    pub mod bus {
        pub mod command_dispatcher;
        pub mod command_dispatcher_test;
    }
    pub mod command;
    pub mod command_handler;
}
pub mod event {
    pub mod event;
}