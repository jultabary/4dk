pub mod command {
    pub mod command_bus;
    pub mod bus_impl {
        pub mod error {
            pub mod unknown_command_error;
        }
        pub mod command_dispatcher;
        pub mod command_dispatcher_test;
        pub mod command_bus_shared_btw_threads;
    }
    pub mod command;
    pub mod command_handler;
}
pub mod event {
    pub mod event;
}