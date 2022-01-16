use dddk_core::dddk::query::query_bus::QueryBus;
use rocket::State;
use crate::Context;
use crate::usecases::queries::a_query_handler::WhatAreAllTheFoosQuery;

#[get("/")]
pub fn get_all_foo(context: &State<Context>) -> String {
    let what_are_all_the_foos = WhatAreAllTheFoosQuery {};
    let _foos_as_response = context.dispatch(&what_are_all_the_foos);
    return String::from("Hello world !!!");
}