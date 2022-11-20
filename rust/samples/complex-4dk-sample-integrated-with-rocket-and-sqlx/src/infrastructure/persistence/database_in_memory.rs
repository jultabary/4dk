use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::errors::errors::TaskNotFound;
use crate::domain::task::{Task, TaskId};
use crate::domain::task_repository::TaskRepository;

pub struct DatabaseInMemory {
    tasks: RefCell<HashMap<Uuid, Task>>,
}

impl DatabaseInMemory {
    pub fn _new() -> Self {
        DatabaseInMemory { tasks: RefCell::new(HashMap::<Uuid, Task>::new()) }
    }
}

impl TaskRepository for DatabaseInMemory {
    fn save(&self, task: &Task) {
        let mut db = self.tasks.borrow_mut();
        db.insert(task.id().to_uuid(), task.clone());
    }

    fn update(&self, task: &Task) {
        let mut db = self.tasks.borrow_mut();
        db.insert(task.id().to_uuid(), task.clone());
    }

    fn find_by_id(&self, task_id: &TaskId) -> Result<Task, TaskNotFound> {
        return match self.tasks.borrow().get(&task_id.to_uuid()) {
            None => {
                Err(TaskNotFound::new(task_id.to_uuid().to_string()))
            }
            Some(task) => {
                Ok(task.clone())
            }
        };
    }

    fn find_all(&self) -> Vec<Task> {
        self.tasks.borrow().values()
            .map(|value| -> Task {
                Task::reconstitute(value.id().clone(), value.title().to_string(), value.description().to_string(), value.done(), value.date())
            })
            .collect()
    }

    fn delete_by_id(&self, task_id: &TaskId) {
        self.tasks.borrow_mut().remove(&task_id.to_uuid());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for TaskId {
    fn clone(&self) -> Self {
        TaskId::from(self.to_uuid())
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task::reconstitute(self.id().clone(), self.title().to_string(), self.description().to_string(), self.done(), self.date())
    }
}

