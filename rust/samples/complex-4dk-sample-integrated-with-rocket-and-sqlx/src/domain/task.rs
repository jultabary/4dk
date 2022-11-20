use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct TaskId {
    uuid: Uuid,
}

impl ToString for TaskId {
    fn to_string(&self) -> String {
        self.uuid.to_string()
    }
}

impl TaskId {
    pub fn new() -> TaskId {
        TaskId {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn from(uuid: Uuid) -> TaskId {
        TaskId {
            uuid,
        }
    }

    pub fn to_uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug)]
pub struct Task {
    id: TaskId,
    title: String,
    done: bool,
    description: String,
    date: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, description: String) -> Task {
        Task {
            id: TaskId::new(),
            title,
            description,
            done: false,
            date: Option::None
        }
    }

    pub fn reconstitute(id: TaskId, title: String, description: String, done: bool, date: Option<DateTime<Utc>>) -> Task {
        Task {
            id,
            title,
            description,
            done,
            date,
        }
    }

    pub fn pass_task_to_done(&mut self) {
        if self.done == false {
            self.done = true;
            self.date = Some(chrono::offset::Utc::now());
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn date(&self) -> Option<DateTime<Utc>> {
        self.date
    }
}
