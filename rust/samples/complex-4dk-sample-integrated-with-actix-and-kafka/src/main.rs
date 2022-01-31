#[macro_use]
extern crate diesel;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use actix_web::{App, HttpServer};
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::external_event::policy_handler::PolicyHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use log::LevelFilter;
use crate::infrastructure::api::routes::{get_all_news_paper, post_one_news_paper, submit_article_to_existing_news_paper};
use crate::infrastructure::database::database_repository::{establish_connection, NewsPaperRepositoryAdapter};
use crate::infrastructure::kafka::article_review_consumer::consume_article_review_event;
use crate::infrastructure::kafka::config::KafkaConfig;
use crate::infrastructure::kafka::generic_consumer::consume_messages;
use crate::logger::SimpleLogger;
use crate::usecases::commands::create_news_paper_command_handler::CreateNewsPaperCommandHandler;
use crate::usecases::commands::publish_article_command_handler::PublishArticleCommandHandler;
use crate::usecases::commands::submit_article_command_handler::SubmitArticleCommandHandler;
use crate::usecases::policies::publish_article_if_validated_policy_handler::PublishArticleIfValidatedPolicyHandler;
use crate::usecases::queries::what_are_news_papers_query_handler::WhatAreNewsPaperQueryHandler;

mod domain;
mod infrastructure;
mod usecases;
mod logger;
pub mod schema;

static LOGGER: SimpleLogger = SimpleLogger {};

pub struct Context {
    bus: Bus,
}

impl Context {
    pub fn new() -> Context {
        // clone a Rc smart pointer doesn't copy the value, it creates a new pointer. See Rc and Arc documentation for more detail
        let connection = Rc::new(establish_connection());
        let news_paper_repository = Rc::new(NewsPaperRepositoryAdapter::new(connection));

        let create_news_paper_command_handler = CreateNewsPaperCommandHandler::new(news_paper_repository.clone());
        let submit_article_command_handler = SubmitArticleCommandHandler::new(news_paper_repository.clone());
        let publish_article_command_handler = PublishArticleCommandHandler::new(news_paper_repository.clone());
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(create_news_paper_command_handler));
        command_handlers.push(Box::new(publish_article_command_handler));
        command_handlers.push(Box::new(submit_article_command_handler));

        let what_are_news_paper_query_handler = WhatAreNewsPaperQueryHandler::new(news_paper_repository.clone());
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(what_are_news_paper_query_handler));

        let event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;

        let publish_article_if_it_has_been_reviewed = PublishArticleIfValidatedPolicyHandler {};
        let mut policy_handlers = Vec::new() as Vec<Box<dyn PolicyHandlerInBus>>;
        policy_handlers.push(Box::new(publish_article_if_it_has_been_reviewed));

        let bus = Bus::new(command_handlers, event_handlers, query_handlers, policy_handlers);
        Context {
            bus
        }
    }
    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }
}

unsafe impl Sync for Context {}

unsafe impl Send for Context {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    // I prefer to copy middleware rather share all the bus between the two contexts (Actix and Kafka)
    // Bus is stateless and copy it does not cos a lot
    // In the other hand, i have shared the database context in both with an Arc.
    let kafka_context = Arc::new(Context::new());
    thread::spawn(|| {
        let kafka_config = KafkaConfig::from_var_env();
        let _error = consume_messages(kafka_config,
                         "article.review".to_string(),
                         kafka_context,
                         consume_article_review_event,
        );
    });
    HttpServer::new(
        || {
            let actix_context = RefCell::new(Context::new());
            App::new()
                .service(get_all_news_paper)
                .service(post_one_news_paper)
                .service(submit_article_to_existing_news_paper)
                .data(actix_context)
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
