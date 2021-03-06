use std::str::FromStr;
use dddk_core::dddk::query::response::Response;
use dddk_security::dddk::security::command::secured_command::SecuredCommand;
use dddk_security::dddk::security::query::secured_query::SecuredQuery;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use crate::Context;
use crate::infrastructure::http::error_handling::catch_error_from_bus;
use crate::infrastructure::http::guard::Token;
use crate::infrastructure::http::model::FooModelApi;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
use crate::usecases::events::foo_created_event::FooCreatedEvent;
use crate::usecases::queries::foos_response::FoosResponse;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;


#[get("/foo")]
pub fn get_all_foo(token: Token, context: &State<Context>) -> Result<Json<Vec<FooModelApi>>, Status> {
    let what_are_all_the_foos = SecuredQuery::new(
        Box::new(WhatAreAllTheFoosQuery {}),
        token.get_user_authorization().get_roles().clone(),
    );
    let foos_as_response = context.get_bus().dispatch_query(&what_are_all_the_foos);
    if foos_as_response.is_err() {
        return Err(catch_error_from_bus(foos_as_response.err().unwrap()));
    }
    let responses = convert_response_to_foo_model_api(foos_as_response.unwrap());
    Ok(Json(responses))
}

#[post("/foo", format = "json", data = "<raw_foo>")]
pub fn post_foo(token: Token, raw_foo: Json<FooModelApi>, context: &State<Context>) -> String {
    let command = SecuredCommand::new(
        Box::new(CreateFooCommand::new(Uuid::from_str(&raw_foo.id).unwrap(), raw_foo.title.clone())),
        token.get_user_authorization().get_roles().clone(),
    );
    let events = context.get_bus().dispatch_command(&command).unwrap();
    events.get(0)
        .unwrap().as_any().downcast_ref::<FooCreatedEvent>()
        .unwrap()
        .id.to_string()
}

fn convert_response_to_foo_model_api(mut foos_as_response: Box<dyn Response>) -> Vec<FooModelApi> {
    let foos = foos_as_response.as_any().downcast_ref::<FoosResponse>().unwrap();
    foos.get_foos()
        .iter().map(|foo| { FooModelApi::from_domain(foo) })
        .collect()
}
