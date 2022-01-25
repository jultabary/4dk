use std::cell::RefCell;
use actix_web::{Responder, web, get};
use crate::Context;
use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;

#[get("/foo")]
pub async fn get_all_foo(context: web::Data<RefCell<Context>>) -> impl Responder {
    let query = WhatAreAllTheFoosQuery {};
    let _result = context.get_ref().borrow().bus.dispatch_query(&query);
    format!("Hello World !!")
}
