use std::any::Any;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::{PgRow};
use uuid::Uuid;
use crate::domain::errors::errors::TaskNotFound;
use crate::domain::task::{Task, TaskId};
use crate::TaskRepository;
use futures::executor::block_on;

pub struct TaskRepositorySqlXImpl {
    pool: Pool<Postgres>,
}

impl TaskRepositorySqlXImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        TaskRepositorySqlXImpl { pool }
    }

    fn to_task(&self, row: PgRow) -> Task {
        let id: Uuid = Uuid::from_str(row.try_get("id").unwrap()).unwrap();
        let title: String = row.try_get("title").unwrap();
        let description: String = row.try_get("description").unwrap();
        let datetime: Option<DateTime<Utc>> = if row.try_get::<Option<DateTime<Utc>>, _>("datetime").is_ok() { row.try_get::<Option<DateTime<Utc>>, _>("datetime").unwrap() } else { None };
        Task::reconstitute(TaskId::from(id), title, description, datetime.is_some(), datetime)
    }
}

impl TaskRepository for TaskRepositorySqlXImpl {
    fn save(&self, task: &Task) {
            block_on(sqlx::query("INSERT INTO task(id, title, description) values($1, $2, $3)")
                .bind(task.id().to_uuid().to_string())
                .bind(task.title().to_string())
                .bind(task.description().to_string())
                .execute(&self.pool)).unwrap();
    }

    fn update(&self, task: &Task) {
        block_on(sqlx::query("UPDATE task set datetime = $1 where id = $2")
            .bind(task.date().unwrap())
            .bind(task.id().to_uuid().to_string())
            .execute(&self.pool)).unwrap();
    }

    fn find_by_id(&self, task_id: &TaskId) -> Result<Task, TaskNotFound> {
        let id = task_id.to_uuid().to_string();
        let result = block_on(sqlx::query("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| {
                self.to_task(row)
            })
            .fetch_optional(&self.pool)).unwrap();
        if result.is_some() {
            Ok(result.unwrap())
        } else {
            Err(TaskNotFound::new(task_id.to_uuid().to_string()))
        }
    }

    fn find_all(&self) -> Vec<Task> {
        block_on(sqlx::query("SELECT * FROM task")
            .map(|row: PgRow| {
                self.to_task(row)
            })
            .fetch_all(&self.pool)).unwrap()
    }

    fn delete_by_id(&self, task_id: &TaskId) {
        let id = task_id.to_uuid().to_string();
        block_on(sqlx::query("DELETE FROM task where id = $1")
            .bind(id)
            .execute(&self.pool)).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
