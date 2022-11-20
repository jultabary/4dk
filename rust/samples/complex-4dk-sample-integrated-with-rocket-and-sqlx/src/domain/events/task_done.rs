#[derive(Event, Debug)]
pub struct TaskDone {
    id: String,
}

impl TaskDone {
    pub fn new(id: String) -> Self {
        TaskDone { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
