use dddk_core::dddk::query::query_bus::QueryBus;
use dddk_core::dddk::query::response::Response;
use rocket::serde::json::Json;
use rocket::State;
use crate::Context;
use crate::domain::foo::Foo;
use crate::infrastructure::response::FooResponse;
use crate::usecases::queries::a_query_handler::WhatAreAllTheFoosQuery;

#[get("/")]
pub fn get_all_foo(context: &State<Context>) -> Json<Vec<FooResponse>> {
    let what_are_all_the_foos = WhatAreAllTheFoosQuery {};
    let foos_as_response = context.dispatch(&what_are_all_the_foos);
    let mut responses = Vec::new();
    foos_as_response
        .iter()
        .for_each(|response: &Box<dyn Response>| {
            let foo= response.as_any().downcast_ref::<Foo>().unwrap();
            responses.push(FooResponse::from_domain(foo))
        });
    Json(responses)
}