use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use std::any::{Any, TypeId};
use std::rc::Rc;
use crate::domain::error::NewsPaperDoesNotExist;
use crate::domain::news_paper_repository::NewsPaperRepository;

#[derive(Command, Debug)]
pub struct PublishArticleCommand {
    pub news_paper_name: String,
    pub article_title: String,
}

#[derive(CommandHandlerInBus)]
pub struct PublishArticleCommandHandler {
    repository: Rc<dyn NewsPaperRepository>,
}

impl PublishArticleCommandHandler {
    pub fn new(repository: Rc<dyn NewsPaperRepository>) -> PublishArticleCommandHandler {
        PublishArticleCommandHandler {
            repository
        }
    }
}

impl CommandHandler<PublishArticleCommand> for PublishArticleCommandHandler {
    fn handle(&self, command: &PublishArticleCommand) -> Events {
        if let Some(mut news_paper) = self.repository.find_by_name(&command.news_paper_name) {
            let err = news_paper.publish_article(command.article_title.clone());
            if err.is_err() {
                return Err(err.err().unwrap());
            }
            self.repository.update(&news_paper);
            Ok(news_paper.move_generated_events())
        } else {
            Err(Box::new(NewsPaperDoesNotExist { news_paper: command.news_paper_name.clone() }))
        }
    }
}