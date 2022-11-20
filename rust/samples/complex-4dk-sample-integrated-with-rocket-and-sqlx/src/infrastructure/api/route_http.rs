use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::App;
use rocket::serde::{Serialize, Deserialize};
use uuid::{Uuid};
use crate::domain::events::task_created::TaskCreated;
use crate::domain::events::task_deleted::TaskDeleted;
use crate::domain::events::task_done::TaskDone;
use crate::domain::task::TaskId;
use crate::infrastructure::api::error_handling::catch_error_from_bus;
use crate::usecases::command::create_task::CreateTaskCommand;
use crate::usecases::command::delete_task::DeleteTaskCommand;
use crate::usecases::command::pass_task_to_done::PassTaskToDoneCommand;
use crate::usecases::query::what_are_all_the_tasks::{AllTasks, WhatAreAllTheTasksQuery};

#[derive(Serialize)]
pub struct TaskResponse {
    id: String,
    title: String,
    description: String,
    done: bool,
    date: Option<String>,
}

#[get("/task", format = "json")]
pub fn get_all_task(app: &State<App>) -> Json<Vec<TaskResponse>> {
    let query = WhatAreAllTheTasksQuery::new();
    let mut response = app.bus.dispatch_query(&query).unwrap();
    let all_tasks = response.as_any().downcast_ref::<AllTasks>().unwrap();
    let all_tasks_json_response = all_tasks.all_tasks().iter()
        .map(|task| -> TaskResponse {
            let mut date_time = None as Option<String>;
            if let Some(date) = task.date() {
                date_time = Some(date.to_string());
            }
            TaskResponse {
                id: task.id().to_uuid().to_string(),
                title: task.title().to_string(),
                description: task.description().to_string(),
                done: task.done(),
                date: date_time,
            }
        }).collect::<Vec<TaskResponse>>();
    Json(all_tasks_json_response)
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: String,
}

#[derive(Serialize)]
pub struct TaskCreatedResponse {
    id: String,
}

#[post("/task", format = "json", data = "<create_task_payload>")]
pub fn create_task(create_task_payload: Json<CreateTaskRequest>, app: &State<App>) -> Json<TaskCreatedResponse> {
    let command = CreateTaskCommand::new(create_task_payload.title.to_string(), create_task_payload.description.to_string());
    let events = app.bus.dispatch_command(&command).unwrap();
    let task_created = events.get(0).unwrap().as_any().downcast_ref::<TaskCreated>().unwrap();
    Json(TaskCreatedResponse { id: task_created.id().to_string() })
}


#[derive(Serialize)]
pub struct TaskDoneResponse {
    id: String,
}

#[put("/task/<id>", format = "json")]
pub fn pass_task_to_done(id: Uuid, app: &State<App>) -> Result<Json<TaskDoneResponse>, Status> {
    let command = PassTaskToDoneCommand::new(TaskId::from(id));
    let events_result = app.bus.dispatch_command(&command);
    if events_result.is_err() {
        return Err(catch_error_from_bus(events_result.err().unwrap()));
    }
    let events = events_result.unwrap();
    let task_done = events.get(0).unwrap().as_any().downcast_ref::<TaskDone>().unwrap();
    Ok(Json(TaskDoneResponse { id: task_done.id().to_string()}))
}

#[derive(Serialize)]
pub struct TaskDeletedResponse {
    id: String,
}

#[delete("/task/<id>", format = "json")]
pub fn delete_task(id: Uuid, app: &State<App>) -> Result<Json<TaskDeletedResponse>, Status> {
    let command = DeleteTaskCommand::new(TaskId::from(id));
    let events_result = app.bus.dispatch_command(&command);
    if events_result.is_err() {
        return Err(catch_error_from_bus(events_result.err().unwrap()));
    }
    let events = events_result.unwrap();
    let task_deleted = events.get(0).unwrap().as_any().downcast_ref::<TaskDeleted>().unwrap();
    Ok(Json(TaskDeletedResponse { id: task_deleted.id().to_string()}))
}
