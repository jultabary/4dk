use std::fmt::{Debug};
use crate::domain::task::Task;

#[derive(Event, Debug)]
pub struct TaskCreated {
    id: String,
    title: String,
    description: String,
}

impl TaskCreated {
    pub fn new(task: &Task) -> TaskCreated{
        TaskCreated {
            id: task.id().to_string(),
            title: task.title().to_string(),
            description: task.description().to_string(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}
