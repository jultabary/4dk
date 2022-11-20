#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use crate::{App, LOGGER};
    use crate::domain::events::task_done::TaskDone;
    use crate::domain::task::Task;
    use crate::usecases::command::pass_task_to_done::PassTaskToDoneCommand;

    #[test]
    pub fn it_should_execute_usecase_correctly() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
        let app = App::new(false);
        let task = Task::new("title".to_string(), "description".to_string());
        app.task_repository().save(&task);
        let pass_task_to_done_command = PassTaskToDoneCommand::new(task.id().clone());

        // When
        let events_result = app.bus.dispatch_command(&pass_task_to_done_command);

        // Then
        assert_eq!(events_result.is_ok(), true);
        let events = events_result.unwrap();
        assert_eq!(events.len(), 1);
        let event = events.get(0).unwrap();
        assert_eq!(event.as_any().downcast_ref::<TaskDone>().is_some(), true);
        let task_done = event.as_any().downcast_ref::<TaskDone>().unwrap();
        assert_eq!(task_done.id().to_string(), task.id().to_uuid().to_string());
    }

}
