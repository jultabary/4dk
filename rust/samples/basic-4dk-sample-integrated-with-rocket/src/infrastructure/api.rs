use std::str::FromStr;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::query::query_bus::QueryBus;
use dddk_core::dddk::query::response::Response;
use rocket::serde::json::{Json};
use rocket::State;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Context;
use crate::domain::foo::Foo;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;

#[derive(Serialize, Deserialize)]
pub struct FooModelApi {
    id: String,
    title: String,
}

impl FooModelApi {
    pub fn from_domain(foo: &Foo) -> FooModelApi {
        FooModelApi {
            id: foo.get_id().to_string(),
            title: foo.get_title().clone(),
        }
    }
}

#[get("/foo")]
pub fn get_all_foo(context: &State<Context>) -> Json<Vec<FooModelApi>> {
    let what_are_all_the_foos = WhatAreAllTheFoosQuery {};
    let foos_as_response = context.query_bus.dispatch(&what_are_all_the_foos);
    let mut responses = Vec::new();
    foos_as_response
        .iter()
        .for_each(|response: &Box<dyn Response>| {
            let foo = response.as_any().downcast_ref::<Foo>().unwrap();
            responses.push(FooModelApi::from_domain(foo))
        });
    Json(responses)
}

#[post("/foo", format = "json", data = "<raw_foo>")]
pub fn post_foo(raw_foo: Json<FooModelApi>, context: &State<Context>) -> String {
    let command = CreateFooCommand::new(
        Uuid::from_str(&raw_foo.id).unwrap(),
        raw_foo.title.clone(),
    );
    context.command_bus.dispatch(&command);
    String::from("OK")
}