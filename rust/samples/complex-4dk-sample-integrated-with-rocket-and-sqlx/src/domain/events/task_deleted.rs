#[derive(Event, Debug)]
pub struct TaskDeleted {
    id: String,
}

impl TaskDeleted {
    pub fn new(id: String) -> Self {
        TaskDeleted { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
