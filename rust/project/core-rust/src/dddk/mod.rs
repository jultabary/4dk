pub mod aliases;
pub mod errors;
pub mod command {
    pub mod command_bus;
    pub mod bus_impl {
        pub mod command_dispatcher;
        mod command_dispatcher_test;
        pub mod event_produced_by_command_bus_dispatcher;
        mod event_produced_by_command_bus_dispatcher_test;
    }
    pub mod command;
    pub mod command_handler;
    mod command_handler_test;
}
pub mod event {
    pub mod bus_impl {
        pub mod event_dispatcher;
        mod event_dispatcher_test;
    }
    pub mod event;
    pub mod event_bus;
    pub mod event_handler;
    mod event_handler_test;
}
pub mod query {
    pub mod query;
    pub mod response;
    pub mod query_handler;
    pub mod query_bus;
    mod query_handler_test;
    pub mod bus_impl {
        pub mod query_dispatcher;
        mod query_dispatcher_test;
    }
}
mod test_tools {
    pub mod some_command_for_test;
    pub mod some_command_handler_for_test;
    pub mod some_event_for_test;
    pub mod some_event_handler_for_test;
    pub mod some_query_for_test;
    pub mod some_response_for_test;
    pub mod some_query_handler_for_test;
}
mod test_macro {
    mod command_macro_test;
    mod event_macro_test;
    mod query_macro_test;
}