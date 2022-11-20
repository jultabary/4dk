use std::fmt::{Debug};
use crate::domain::task::Task;

#[derive(Event, Debug)]
pub struct TaskCreated {
    id: String,
}

impl TaskCreated {
    pub fn new(task: &Task) -> TaskCreated{
        TaskCreated {
            id: task.id().to_string(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}
