#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use crate::{App, LOGGER};
    use crate::domain::events::task_deleted::TaskDeleted;
    use crate::domain::task::Task;
    use crate::usecases::command::delete_task::DeleteTaskCommand;

    #[test]
    pub fn it_should_execute_usecase_correctly() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
        let app = App::new(false);
        let task = Task::new("title".to_string(), "description".to_string());
        app.task_repository().save(&task);
        let delete_task_command = DeleteTaskCommand::new(task.id().clone());

        // When
        let events_result = app.bus.dispatch_command(&delete_task_command);

        // Then
        assert_eq!(events_result.is_ok(), true);
        let events = events_result.unwrap();
        assert_eq!(events.len(), 1);
        let event = events.get(0).unwrap();
        assert_eq!(event.as_any().downcast_ref::<TaskDeleted>().is_some(), true);
        let task_deleted = event.as_any().downcast_ref::<TaskDeleted>().unwrap();
        assert_eq!(task_deleted.id().to_string(), task.id().to_uuid().to_string());
    }

}
