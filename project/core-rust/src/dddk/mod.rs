pub mod command {
    pub mod command_bus;
    pub mod bus {
        pub mod command_dispatcher_with_box;
        pub mod command_dispatcher_with_box_test;
        pub mod command_dispatcher_with_ref;
        pub mod command_dispatcher_with_ref_test;
        pub mod command_bus_injected_with_ref;
        pub mod command_bus_injected_with_box;
    }
    pub mod command;
    pub mod command_handler;
}
pub mod event {
    pub mod event;
}