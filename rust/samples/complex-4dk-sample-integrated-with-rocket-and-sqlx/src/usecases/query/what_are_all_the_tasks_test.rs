#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use crate::{App, LOGGER};
    use crate::domain::task::Task;
    use crate::usecases::query::what_are_all_the_tasks::{AllTasks, WhatAreAllTheTasksQuery};

    #[test]
    pub fn it_should_execute_usecase_correctly() {
        // Given
        let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
        let app = App::new(false);
        let what_are_all_the_tasks_query = WhatAreAllTheTasksQuery::new();
        let task = Task::new("title".to_string(), "description".to_string());
        app.task_repository().save(&task);

        // When
        let response_result = app.bus.dispatch_query(&what_are_all_the_tasks_query);

        // Then
        assert_eq!(response_result.is_ok(), true);
        let mut response = response_result.unwrap();
        let all_tasks_option = response.as_any().downcast_ref::<AllTasks>();
        assert_eq!(all_tasks_option.is_some(), true);
        let all_tasks = all_tasks_option.unwrap();
        assert_eq!(all_tasks.all_tasks().len(), 1);
    }

}
