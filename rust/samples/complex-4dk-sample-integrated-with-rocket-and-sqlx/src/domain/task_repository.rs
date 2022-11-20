use std::any::Any;
use crate::domain::errors::errors::TaskNotFound;
use crate::domain::task::{Task, TaskId};

pub trait TaskRepository {
    fn save(&self, task: &Task);

    fn update(&self, task: &Task);

    fn find_by_id(&self, task_id: &TaskId) -> Result<Task, TaskNotFound>;

    fn find_all(&self) -> Vec<Task>;

    fn delete_by_id(&self, task_id: &TaskId);

    fn as_any (&self) -> &dyn Any;
}
