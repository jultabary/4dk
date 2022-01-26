use std::cell::RefCell;
use std::str::FromStr;
use actix_web::{Responder, web, get, post, Result};
use uuid::Uuid;
use crate::Context;
use crate::infrastructure::api::api_model::FooApi;
use crate::infrastructure::api::error_handling::CustomHttpError;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
use crate::usecases::events::foo_created_event::FooCreatedEvent;
use crate::usecases::queries::foos_response::FoosResponse;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;

#[post("/foo")]
pub async fn post_foo(foo_api: web::Json<FooApi>, context: web::Data<RefCell<Context>>) -> Result<impl Responder, CustomHttpError> {
    let id_result = Uuid::from_str(foo_api.id.as_str());
    if id_result.is_err() {
        return Err(CustomHttpError::BadRequest);
    }
    let command = CreateFooCommand::new(
        id_result.unwrap(), foo_api.title.clone(),
    );
    let result = context.get_ref().borrow().bus.dispatch_command(&command);
    if result.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    let id = result.unwrap().get(0).unwrap()
        .as_any().downcast_ref::<FooCreatedEvent>().unwrap().id.to_string();
    Ok(id)
}

#[get("/foo")]
pub async fn get_all_foo(context: web::Data<RefCell<Context>>) -> Result<impl Responder, CustomHttpError> {
    let query = WhatAreAllTheFoosQuery {};
    let result = context.get_ref().borrow().bus.dispatch_query(&query);
    if result.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    let response = result.unwrap();
    let foos = response.as_any().downcast_ref::<FoosResponse>().unwrap();
    let api_reponse: Vec<FooApi> = foos.get_foos()
        .iter().map(|foo| { FooApi::from_domain(foo) })
        .collect();
    Ok(web::Json(api_reponse))
}
