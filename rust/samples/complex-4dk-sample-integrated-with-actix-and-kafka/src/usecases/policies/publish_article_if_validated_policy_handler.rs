use std::any::{Any, TypeId};
use dddk_core::dddk::aliases::Commands;
use dddk_core::dddk::command::command::Command;
use dddk_macro::PolicyHandlerInBus;
use dddk_core::dddk::external_event::policy_handler::{PolicyHandler, PolicyHandlerInBus};
use dddk_core::dddk::external_event::external_event::ExternalEvent;
use crate::usecases::policies::article_has_been_reviewed_event::ArticleHasBeenReviewedEvent;
use crate::usecases::commands::publish_article_command_handler::PublishArticleCommand;

#[derive(PolicyHandlerInBus)]
pub struct PublishArticleIfValidatedPolicyHandler {}

impl PolicyHandler<ArticleHasBeenReviewedEvent> for PublishArticleIfValidatedPolicyHandler {
    fn handle(&self, external_event: &ArticleHasBeenReviewedEvent) -> Commands {
        let mut commands = Vec::new() as Vec<Box<dyn Command>>;
        if external_event.is_validated {
            commands.push(Box::new(
                PublishArticleCommand {
                    news_paper_name: external_event.news_paper_name.clone(),
                    article_title: external_event.article_title.clone(),
                })
            );
        }
        Ok(commands)
    }
}