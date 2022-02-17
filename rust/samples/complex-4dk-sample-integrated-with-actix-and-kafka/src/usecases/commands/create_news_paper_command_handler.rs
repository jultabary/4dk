use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use dddk_core::dddk::aliases::Events;
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use dddk_core::dddk::command::command_handler::CommandHandler;
use crate::domain::error::NewsPaperAlreadyExist;
use crate::domain::news_paper::NewsPaper;
use crate::domain::news_paper_repository::NewsPaperRepository;

#[derive(Command)]
pub struct CreateNewsPaperCommand {
    pub name: String,
}

impl Debug for CreateNewsPaperCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "CreateNewsPaperCommand[name: [{}]]",
               self.name
        )
    }
}

#[derive(CommandHandlerInBus)]
pub struct CreateNewsPaperCommandHandler {
    news_paper_repository: Rc<dyn NewsPaperRepository>,
}

impl CreateNewsPaperCommandHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperRepository>) -> CreateNewsPaperCommandHandler {
        CreateNewsPaperCommandHandler {
            news_paper_repository
        }
    }
}

impl CommandHandler<CreateNewsPaperCommand> for CreateNewsPaperCommandHandler {
    fn handle(&self, command: &CreateNewsPaperCommand) -> Events {
        if let Some(_) = self.news_paper_repository.find_by_name(&command.name) {
            Err(Box::new(NewsPaperAlreadyExist { news_paper: command.name.clone() }))
        } else {
            let mut news_paper = NewsPaper::new(command.name.clone());
            self.news_paper_repository.save(&news_paper);
            Ok(news_paper.move_generated_events())
        }
    }
}
