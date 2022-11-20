use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TaskNotFound {
    id: String,
}

impl TaskNotFound {
    pub fn new(id: String) -> Self {
        TaskNotFound { id }
    }
}

impl Error for TaskNotFound { }

impl Display for TaskNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task with id: [{}] has not been found.", self.id)
    }
}
