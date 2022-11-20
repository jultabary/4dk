use std::rc::Rc;
use dddk_core::dddk::aliases::ResponseFromHandler;
use dddk_core::dddk::query::query_handler::QueryHandler;
use crate::domain::task::Task;
use crate::domain::task_repository::TaskRepository;

#[derive(Query, Debug)]
pub struct WhatAreAllTheTasksQuery { }

impl WhatAreAllTheTasksQuery {
    pub fn new() -> Self {
        WhatAreAllTheTasksQuery {}
    }
}

#[derive(Response, Debug)]
pub struct AllTasks {
    all_tasks: Vec<Task>,
}

impl AllTasks {
    pub fn new(all_tasks: Vec<Task>) -> Self {
        AllTasks { all_tasks }
    }
    pub fn all_tasks(&self) -> &Vec<Task> {
        &self.all_tasks
    }
}

#[derive(QueryHandlerInBus)]
pub struct WhatAreAllTheTasks {
    task_repository: Rc<dyn TaskRepository>,
}

impl WhatAreAllTheTasks {
    pub fn new(task_repository: Rc<dyn TaskRepository>) -> Self {
        WhatAreAllTheTasks { task_repository }
    }
}

impl QueryHandler<WhatAreAllTheTasksQuery> for WhatAreAllTheTasks {
    fn handle(&self, _query: &WhatAreAllTheTasksQuery) -> ResponseFromHandler {
        let tasks = self.task_repository.find_all();
        Ok(Box::new(AllTasks::new(tasks)))
    }
}
