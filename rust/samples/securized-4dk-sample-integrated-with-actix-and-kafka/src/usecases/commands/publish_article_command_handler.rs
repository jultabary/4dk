use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use crate::domain::article::Article;
use crate::domain::error::NewsPaperDoesNotExist;
use crate::domain::news_paper_repository::NewsPaperRepository;

#[derive(Command)]
pub struct PublishArticleCommand {
    pub title: String,
    pub body: String,
    pub  news_paper_name: String,
}

impl Debug for PublishArticleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "PublishArticleCommand[id: [{}], title: [{}] to NewsPaper[{}]",
               self.title,
               self.body,
               self.news_paper_name
        )
    }
}

#[derive(CommandHandlerInBus)]
pub struct PublishArticleCommandHandler {
    news_paper_repository: Rc<dyn NewsPaperRepository>,
}

impl PublishArticleCommandHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperRepository>) -> PublishArticleCommandHandler {
        PublishArticleCommandHandler {
            news_paper_repository
        }
    }
}

impl CommandHandler<PublishArticleCommand> for PublishArticleCommandHandler {
    fn handle(&self, command: &PublishArticleCommand) -> Events {
        if let Some(mut news_paper) = self.news_paper_repository.find_by_name(&command.news_paper_name) {
            let article = Article::new(command.title.clone(), command.body.clone());
            let err = news_paper.publish(article);
            if err.is_err() {
                return Err(Box::new(err.err().unwrap()));
            }
            self.news_paper_repository.save(&news_paper);
            Ok(news_paper.move_generated_events())
        } else {
            Err(Box::new(NewsPaperDoesNotExist { news_paper: command.news_paper_name.clone() }))
        }
    }
}
