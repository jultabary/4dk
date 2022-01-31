use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use std::any::{Any, TypeId};

#[derive(Command, Debug)]
pub struct PublishArticleCommand {
    pub news_paper_name: String,
    pub article_title: String,
}

#[derive(CommandHandlerInBus)]
pub struct PublishArticleCommandHandler {}

impl CommandHandler<PublishArticleCommand> for PublishArticleCommandHandler {
    fn handle(&self, _command: &PublishArticleCommand) -> Events {
        todo!()
    }
}