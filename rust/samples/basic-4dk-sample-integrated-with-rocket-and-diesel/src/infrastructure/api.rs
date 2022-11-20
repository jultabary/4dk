use std::str::FromStr;
use rocket::serde::json::{Json};
use rocket::State;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Context;
use crate::domain::foo::Foo;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
use crate::usecases::events::foo_created_event::FooCreatedEvent;
use crate::usecases::queries::foos_response::FoosResponse;
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
    let mut foos_as_response = context.get_bus().dispatch_query(&what_are_all_the_foos).unwrap();
    let foos = foos_as_response.as_any().downcast_ref::<FoosResponse>().unwrap();
    let api_reponse: Vec<FooModelApi> = foos.get_foos()
        .iter().map(|foo| { FooModelApi::from_domain(foo) })
        .collect();
    Json(api_reponse)
}

#[post("/foo", format = "json", data = "<raw_foo>")]
pub fn post_foo(raw_foo: Json<FooModelApi>, context: &State<Context>) -> String {
    let command = CreateFooCommand::new(
        Uuid::from_str(&raw_foo.id).unwrap(),
        raw_foo.title.clone(),
    );
    let events = context.get_bus().dispatch_command(&command).unwrap();
    events.get(0)
        .unwrap().as_any().downcast_ref::<FooCreatedEvent>()
        .unwrap()
        .id.to_string()
}