use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command_handler::CommandHandler;
use crate::domain::events::task_deleted::TaskDeleted;
use crate::domain::task::TaskId;
use crate::domain::task_repository::TaskRepository;

#[derive(Command, Debug)]
pub struct DeleteTaskCommand {
    id: TaskId,
}

impl DeleteTaskCommand {
    pub fn new(id: TaskId) -> Self {
        DeleteTaskCommand {
            id,
        }
    }
}

#[derive(CommandHandlerInBus)]
pub struct DeleteTask {
    task_repository: Rc<dyn TaskRepository>,
}

impl DeleteTask {
    pub fn new(task_repository: Rc<dyn TaskRepository>) -> Self {
        DeleteTask { task_repository }
    }
}

impl CommandHandler<DeleteTaskCommand> for DeleteTask {
    fn handle(&self, command: &DeleteTaskCommand) -> Events {
        self.task_repository.delete_by_id(&command.id);
        Ok(vec![Arc::new(TaskDeleted::new(command.id.to_uuid().to_string()))])
    }
}
