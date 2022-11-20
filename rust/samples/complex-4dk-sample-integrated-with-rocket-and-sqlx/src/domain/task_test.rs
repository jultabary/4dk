#[cfg(test)]
mod tests {
    use crate::domain::task::Task;

    #[test]
    pub fn it_should_instantiate_a_new_task_when_factory_new_is_used() {
        // Given
        let title = "title".to_string();
        let description = "description".to_string();

        // When
        let task = Task::new(title.clone(), description.clone());

        // Then
        assert_eq!(task.title(), title);
        assert_eq!(task.description(), description);
        assert_eq!(task.done(), false);
        assert_eq!(task.date(), None);
    }

    #[test]
    pub fn it_should_set_a_datetime_when_task_pass_to_done() {
        // Given
        let title = "title".to_string();
        let description = "description".to_string();
        let mut task = Task::new(title.clone(), description.clone());

        // When
        task.pass_task_to_done();

        // Then
        assert_eq!(task.done(), true);
        assert_eq!(task.date().is_some(), true);
    }
}
