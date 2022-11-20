use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command_handler::CommandHandler;
use crate::domain::events::task_created::TaskCreated;
use crate::domain::task::Task;
use crate::domain::task_repository::TaskRepository;

#[derive(Command, Debug)]
pub struct CreateTaskCommand {
    title: String,
    description: String,
}

impl CreateTaskCommand {
    pub fn new(title: String, description: String) -> CreateTaskCommand {
        CreateTaskCommand {
            title,
            description,
        }
    }
}

#[derive(CommandHandlerInBus)]
pub struct CreateTask {
    task_repository: Rc<dyn TaskRepository>,
}

impl CreateTask {
    pub fn new(task_repository: Rc<dyn TaskRepository>) -> Self {
        CreateTask { task_repository }
    }
}

impl CommandHandler<CreateTaskCommand> for CreateTask {
    fn handle(&self, command: &CreateTaskCommand) -> Events {
        let task = Task::new(command.title.to_string(), command.description.to_string());
        self.task_repository.save(&task);
        let task_created = Arc::new(TaskCreated::new(&task));
        Ok(vec![task_created])
    }
}

