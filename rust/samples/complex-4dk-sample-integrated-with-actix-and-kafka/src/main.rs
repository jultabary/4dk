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
use bus_config::Context;
use crate::infrastructure::api::routes::{admin_get_all_news_paper, get_all_news_paper, post_one_news_paper, submit_article_to_existing_news_paper};
use crate::infrastructure::database::database_query_repository::NewsPaperQueryRepositoryAdapter;
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
use crate::usecases::queries::what_are_news_papers_query_handler_even_with_unpublished_articles::WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler;

mod domain;
mod infrastructure;
mod usecases;
mod logger;
mod bus_config;
pub mod schema;

static LOGGER: SimpleLogger = SimpleLogger {};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    // I prefer to copy middleware rather share all the bus between the two contexts (Actix and Kafka)
    // Bus is stateless and copy it does not cost a lot
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
                .service(admin_get_all_news_paper)
                .data(actix_context)
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
