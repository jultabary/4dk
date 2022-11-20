#[cfg(test)]
mod tests {
    use crate::{App, block_on, TaskRepositorySqlXImpl};
    use crate::domain::task::Task;

    pub fn before(prod: bool) -> App {
        let app = App::new(prod);
        if prod == true {
            let task_repository = app.task_repository().as_any().downcast_ref::<TaskRepositorySqlXImpl>().unwrap();
            let pool = task_repository.pool();
            block_on(sqlx::query("DELETE FROM task").execute(pool)).unwrap();
        }
        return app;
    }

    #[test]
    pub fn test_task_repository_with_hashmap_implementation() {
        it_should_persist_a_task_when_method_save_is_called(false);
        it_should_return_all_tasks_when_asked(false);
        it_should_delete_desired_task_when_asked(false);
        it_should_update_desired_task_when_asked(false);
    }

    #[test]
    pub fn test_task_repository_with_sql_database_implementation() {
        it_should_persist_a_task_when_method_save_is_called(true);
        it_should_return_all_tasks_when_asked(true);
        it_should_delete_desired_task_when_asked(true);
        it_should_update_desired_task_when_asked(true);
    }

    pub fn it_should_persist_a_task_when_method_save_is_called(prod: bool) {
        // Given
        let app = before(prod);
        let persistence_db = app.task_repository();
        let task = Task::new("title".to_string(), "description".to_string());

        // When
        persistence_db.save(&task);

        // Then
        let task_result = persistence_db.find_by_id(task.id());
        assert_eq!(task_result.is_ok(), true);
        let assert_task = task_result.unwrap();
        assert_eq!(assert_task.id().to_uuid(), task.id().to_uuid());
        assert_eq!(assert_task.title(), task.title());
        assert_eq!(assert_task.description(), task.description());
    }

    pub fn it_should_return_all_tasks_when_asked(prod: bool) {
        // Given
        let app = before(prod);
        let persistence_db = app.task_repository();
        let task1 = Task::new("title1".to_string(), "description1".to_string());
        let task2 = Task::new("title2".to_string(), "description2".to_string());
        persistence_db.save(&task1);
        persistence_db.save(&task2);

        // When
        let tasks = persistence_db.find_all();

        // Then
        assert_eq!(tasks.len(), 2);
    }

    pub fn it_should_delete_desired_task_when_asked(prod: bool) {
        // Given
        let app = before(prod);
        let persistence_db = app.task_repository();
        let task1 = Task::new("title1".to_string(), "description1".to_string());
        persistence_db.save(&task1);

        // When
        persistence_db.delete_by_id(task1.id());

        // Then
        let tasks = persistence_db.find_all();
        assert_eq!(tasks.len(), 0);
    }

    pub fn it_should_update_desired_task_when_asked(prod: bool) {
        // Given
        let app = before(prod);
        let persistence_db = app.task_repository();
        let mut task1 = Task::new("title1".to_string(), "description1".to_string());
        persistence_db.save(&task1);
        task1.pass_task_to_done();

        // When
        persistence_db.update(&task1);

        // Then
        let task_result = persistence_db.find_by_id(task1.id());
        assert_eq!(task_result.is_ok(), true);
        assert_eq!(task_result.unwrap().date().is_some(), true);
    }
}
