use std::rc::Rc;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::external_event::policy_handler::PolicyHandlerInBus;
use dddk_core::Bus;
use crate::infrastructure::database::database_query_repository::NewsPaperQueryRepositoryAdapter;
use crate::infrastructure::database::database_repository::{establish_connection, NewsPaperRepositoryAdapter};
use crate::usecases::commands::create_news_paper_command_handler::CreateNewsPaperCommandHandler;
use crate::usecases::commands::publish_article_command_handler::PublishArticleCommandHandler;
use crate::usecases::commands::submit_article_command_handler::SubmitArticleCommandHandler;
use crate::usecases::policies::publish_article_if_validated_policy_handler::PublishArticleIfValidatedPolicyHandler;
use crate::usecases::queries::what_are_news_papers_query_handler::WhatAreNewsPaperQueryHandler;
use crate::usecases::queries::what_are_news_papers_query_handler_even_with_unpublished_articles::WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler;

pub struct Context {
    bus: Bus,
}

impl Context {
    pub fn new() -> Context {
        // clone a Rc smart pointer doesn't copy the value, it creates a new pointer. See Rc and Arc documentation for more detail
        let connection = Rc::new(establish_connection());
        let news_paper_repository = Rc::new(NewsPaperRepositoryAdapter::new(connection.clone()));
        let news_paper_query_repository = Rc::new(NewsPaperQueryRepositoryAdapter::new(connection.clone()));

        let create_news_paper_command_handler = CreateNewsPaperCommandHandler::new(news_paper_repository.clone());
        let submit_article_command_handler = SubmitArticleCommandHandler::new(news_paper_repository.clone());
        let publish_article_command_handler = PublishArticleCommandHandler::new(news_paper_repository.clone());
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(create_news_paper_command_handler));
        command_handlers.push(Box::new(publish_article_command_handler));
        command_handlers.push(Box::new(submit_article_command_handler));

        let what_are_news_paper_query_handler = WhatAreNewsPaperQueryHandler::new(news_paper_query_repository.clone());
        let what_are_news_paper_query_handler_even_with_unpublished_articles = WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler::new(news_paper_query_repository.clone());
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(what_are_news_paper_query_handler));
        query_handlers.push(Box::new(what_are_news_paper_query_handler_even_with_unpublished_articles));

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
