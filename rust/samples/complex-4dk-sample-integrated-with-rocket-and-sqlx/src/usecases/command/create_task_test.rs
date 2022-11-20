#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use crate::{App, LOGGER};
    use crate::domain::events::task_created::TaskCreated;
    use crate::usecases::command::create_task::CreateTaskCommand;

    #[test]
    pub fn it_should_execute_usecase_correctly() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
        let app = App::new(false);
        let create_task_command = CreateTaskCommand::new("title".to_string(), "description".to_string());

        // When
        let events_result = app.bus.dispatch_command(&create_task_command);

        // Then
        assert_eq!(events_result.is_ok(), true);
        let events = events_result.unwrap();
        assert_eq!(events.len(), 1);
        let event = events.get(0).unwrap();
        assert_eq!(event.as_any().downcast_ref::<TaskCreated>().is_some(), true);
        let task_created = event.as_any().downcast_ref::<TaskCreated>().unwrap();
        assert_eq!(task_created.title(), "title".to_string());
        assert_eq!(task_created.description(), "description".to_string());
    }
}
