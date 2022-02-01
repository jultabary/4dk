use std::cell::RefCell;
use actix_web::{get, post, Responder, web};
use crate::bus_config::Context;
use crate::domain::response::news_paper_response::NewsPapersResponse;
use crate::infrastructure::api::api_model::{NewsPaperBodyRequest, SubmitArticleRequest};
use crate::infrastructure::api::error_handling::CustomHttpError;
use crate::usecases::commands::create_news_paper_command_handler::CreateNewsPaperCommand;
use crate::usecases::commands::submit_article_command_handler::SubmitArticleCommand;
use crate::usecases::queries::what_are_news_papers_query_handler::WhatAreNewsPaperQuery;
use crate::usecases::queries::what_are_news_papers_query_handler_even_with_unpublished_articles::WhatAreNewsPaperEventWithUnpublishedArticlesQuery;

#[get("/news_papers")]
pub async fn get_all_news_paper(context: web::Data<RefCell<Context>>) -> Result<impl Responder, CustomHttpError> {
    let query = WhatAreNewsPaperQuery {};
    let responses = context.get_ref().borrow().get_bus().dispatch_query(&query);
    if responses.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    let mut responses = responses.unwrap();
    let news_papers_response = responses.as_mut().as_any().downcast_mut::<NewsPapersResponse>().unwrap();
    Ok(web::Json(news_papers_response.move_news_papers()))
}

#[get("/admin/news_papers")]
pub async fn admin_get_all_news_paper(context: web::Data<RefCell<Context>>) -> Result<impl Responder, CustomHttpError> {
    let query = WhatAreNewsPaperEventWithUnpublishedArticlesQuery {};
    let responses = context.get_ref().borrow().get_bus().dispatch_query(&query);
    if responses.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    let mut responses = responses.unwrap();
    let news_papers_response = responses.as_mut().as_any().downcast_mut::<NewsPapersResponse>().unwrap();
    Ok(web::Json(news_papers_response.move_news_papers()))
}


#[post("/news_paper")]
pub async fn post_one_news_paper(body: web::Json<NewsPaperBodyRequest>, context: web::Data<RefCell<Context>>) -> Result<String, CustomHttpError>{
    let command = CreateNewsPaperCommand { name: body.name.clone() };
    let events = context.get_ref().borrow().get_bus().dispatch_command(&command);
    if events.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    Ok("ok".to_string())
}

#[post("/news_paper/{name}/articles")]
pub async fn submit_article_to_existing_news_paper(
    web::Path(news_paper_name): web::Path<String>,
    body: web::Json<SubmitArticleRequest>,
    context: web::Data<RefCell<Context>>) -> Result<String, CustomHttpError>{
    let command = SubmitArticleCommand {
        title: body.title.clone(),
        body: body.body.clone(),
        news_paper_name
    };
    let events = context.get_ref().borrow().get_bus().dispatch_command(&command);
    if events.is_err() {
        return Err(CustomHttpError::InternalServerError);
    }
    Ok("ok".to_string())
}