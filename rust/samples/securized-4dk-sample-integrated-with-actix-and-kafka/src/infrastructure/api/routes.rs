use std::cell::RefCell;
use actix_web::{get, post, Responder, web};
use crate::Context;
use crate::infrastructure::api::api_model::NewsPaperBodyRequest;
use crate::infrastructure::api::error_handling::CustomHttpError;
use crate::usecases::commands::open_news_paper_command_handler::OpenNewsPaperCommand;
use crate::usecases::queries::what_are_opened_news_papers_query_handler::{NewsPapersResponse, WhatAreOpenedNewsPaperQuery};

#[get("/news_papers")]
pub async fn get_all_news_paper(context: web::Data<RefCell<Context>>) -> Result<impl Responder, CustomHttpError> {
    let query = WhatAreOpenedNewsPaperQuery {};
    let responses = context.get_ref().borrow().bus.dispatch_query(&query);
    if responses.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    let mut responses = responses.unwrap();
    let news_papers_response = responses.as_mut().as_any().downcast_mut::<NewsPapersResponse>().unwrap();
    Ok(web::Json(news_papers_response.move_news_papers()))
}

#[post("/news_paper")]
pub async fn post_one_news_paper(body: web::Json<NewsPaperBodyRequest>, context: web::Data<RefCell<Context>>) -> Result<String, CustomHttpError>{
    let command = OpenNewsPaperCommand { name: body.name.clone() };
    let events = context.get_ref().borrow().bus.dispatch_command(&command);
    if events.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    Ok("ok".to_string())
}