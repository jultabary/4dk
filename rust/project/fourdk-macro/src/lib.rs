extern crate proc_macro;

mod impl_command_macro_function;
mod impl_query_macro_function;
mod impl_event_macro_function;
mod impl_external_event_macro_function;

use proc_macro::TokenStream;
use crate::impl_command_macro_function::{impl_command, impl_command_handler_in_bus};
use crate::impl_event_macro_function::{impl_event, impl_event_handler_in_bus};
use crate::impl_external_event_macro_function::{impl_external_event, impl_policy_handler_in_bus};
use crate::impl_query_macro_function::{impl_query_handler_in_bus, impl_query, impl_response};

#[proc_macro_derive(CommandHandlerInBus)]
pub fn command_handler_in_bus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_command_handler_in_bus(&ast)
}

#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_command(&ast)
}

#[proc_macro_derive(QueryHandlerInBus)]
pub fn query_handler_in_bus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_query_handler_in_bus(&ast)
}

#[proc_macro_derive(Query)]
pub fn query_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_query(&ast)
}

#[proc_macro_derive(Response)]
pub fn response_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_response(&ast)
}

#[proc_macro_derive(EventHandlerInBus)]
pub fn event_handler_in_bus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_event_handler_in_bus(&ast)
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_event(&ast)
}

#[proc_macro_derive(PolicyHandlerInBus)]
pub fn policy_handler_in_bus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_policy_handler_in_bus(&ast)
}

#[proc_macro_derive(ExternalEvent)]
pub fn external_event_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_external_event(&ast)
}