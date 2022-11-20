use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command_handler::CommandHandler;
use crate::domain::events::task_done::TaskDone;
use crate::domain::task::TaskId;
use crate::TaskRepository;

#[derive(Command, Debug)]
pub struct PassTaskToDoneCommand {
    id: TaskId,
}

impl PassTaskToDoneCommand {
    pub fn new(id: TaskId) -> Self {
        PassTaskToDoneCommand {
            id,
        }
    }
}

#[derive(CommandHandlerInBus)]
pub struct PassTaskToDone {
    task_repository: Rc<dyn TaskRepository>,
}

impl PassTaskToDone {
    pub fn new(task_repository: Rc<dyn TaskRepository>) -> Self {
        PassTaskToDone { task_repository }
    }
}

impl CommandHandler<PassTaskToDoneCommand> for PassTaskToDone {
    fn handle(&self, command: &PassTaskToDoneCommand) -> Events {
        let task_result = self.task_repository.find_by_id(&command.id);
        if task_result.is_ok() {
            let mut task = task_result.unwrap();
            task.pass_task_to_done();
            self.task_repository.update(&task);
            return Ok(vec![Arc::new(TaskDone::new(command.id.to_uuid().to_string()))]);
        }
        return Err(Box::new(task_result.err().unwrap()));
    }
}
