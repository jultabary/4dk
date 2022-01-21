use std::str::FromStr;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::query::query_bus::QueryBus;
use dddk_core::dddk::query::response::Response;
use dddk_security::dddk::security::command::secured_command::SecuredCommand;
use dddk_security::dddk::security::query::secured_query::SecuredQuery;
use dddk_security::dddk::security::userinfo::user::{User, UserAuthorization};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{Json};
use rocket::{Request, request, State};
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Context;
use crate::domain::foo::Foo;
use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
use crate::usecases::events::foo_created_event::FooCreatedEvent;
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
pub fn get_all_foo(token: Token, context: &State<Context>) -> Json<Vec<FooModelApi>> {
    let what_are_all_the_foos = SecuredQuery::new(
        Box::new(WhatAreAllTheFoosQuery {}),
        token.get_user_authorization().get_roles().clone(),
    );
    let foos_as_response = context.query_bus.dispatch(&what_are_all_the_foos);
    if foos_as_response.is_err() {
        // todo
        panic!("UnAuthorized");
    }
    let foos_as_response = foos_as_response.unwrap();
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
pub fn post_foo(token: Token, raw_foo: Json<FooModelApi>, context: &State<Context>) -> String {
    let command = SecuredCommand::new(
        Box::new(CreateFooCommand::new(Uuid::from_str(&raw_foo.id).unwrap(), raw_foo.title.clone())),
        token.get_user_authorization().get_roles().clone(),
    );
    let events = context.command_bus.dispatch(&command).unwrap();
    events.get(0)
        .unwrap().as_any().downcast_ref::<FooCreatedEvent>()
        .unwrap()
        .id.to_string()
}


pub struct Token {
    value: UserAuthorization,
}

impl Token {
    pub fn get_user_authorization(&self) -> &UserAuthorization {
        &self.value
    }
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                // check validity
                let user = User::new("login".to_string(), Some("login@email.com".to_string()));
                let user_authorization;
                if token == "token" {
                    user_authorization = UserAuthorization::new(user, vec!["role".to_string()]);
                } else if token == "token_1" {
                    user_authorization = UserAuthorization::new(user, vec!["role_1".to_string()]);
                } else {
                    return Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing));
                }
                Outcome::Success(Token { value: user_authorization })
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}