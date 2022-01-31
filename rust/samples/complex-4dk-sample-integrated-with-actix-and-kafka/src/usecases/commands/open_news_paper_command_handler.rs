use std::fmt::{Debug, Formatter};
use std::any::{Any, TypeId};
use std::rc::Rc;
use dddk_core::dddk::aliases::Events;
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use crate::domain::error::NewsPaperAlreadyExist;
use crate::domain::news_paper::NewsPaper;
use crate::domain::news_paper_repository::NewsPaperRepository;

#[derive(Command)]
pub struct OpenNewsPaperCommand {
    pub name: String,
}

impl Debug for OpenNewsPaperCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "OpenNewsPaperCommand[name: [{}]]",
               self.name
        )
    }
}

#[derive(CommandHandlerInBus)]
pub struct OpenNewsPaperCommandHandler {
    news_paper_repository: Rc<dyn NewsPaperRepository>,
}

impl OpenNewsPaperCommandHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperRepository>) -> OpenNewsPaperCommandHandler {
        OpenNewsPaperCommandHandler {
            news_paper_repository
        }
    }
}

impl CommandHandler<OpenNewsPaperCommand> for OpenNewsPaperCommandHandler {
    fn handle(&self, command: &OpenNewsPaperCommand) -> Events {
        if let Some(_) = self.news_paper_repository.find_by_name(&command.name) {
            Err(Box::new(NewsPaperAlreadyExist { news_paper: command.name.clone() }))
        } else {
            let mut news_paper = NewsPaper::new(command.name.clone());
            self.news_paper_repository.save(&news_paper);
            Ok(news_paper.move_generated_events())
        }
    }
}
