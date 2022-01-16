pub mod command {
    pub mod command_bus;
    pub mod bus_impl {
        pub mod error {
            pub mod unknown_command_error;
        }
        pub mod command_dispatcher;
        mod command_dispatcher_test;
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
    pub mod query_handler_test;
    pub mod bus_impl {
        pub mod query_dispatcher;
        mod query_dispatcher_test;
    }
}
mod test {
    pub mod some_command_for_test;
    pub mod some_command_handler_for_test;
    pub mod some_event_for_test;
    pub mod some_event_handler_for_test;
    pub mod some_query_for_test;
    pub mod some_response_for_test;
    pub mod some_query_handler_for_test;
}