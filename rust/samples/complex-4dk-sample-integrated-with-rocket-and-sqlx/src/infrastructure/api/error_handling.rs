use dddk_core::dddk::aliases::GenericError;
use rocket::http::Status;
use rocket::Request;
use crate::domain::errors::errors::TaskNotFound;

pub fn catch_error_from_bus(error: GenericError) -> Status {
    if let Some(_) = error.downcast_ref::<TaskNotFound>() {
        return Status::NotFound;
    } else {
        Status::InternalServerError
    }
}

#[catch(404)]
pub fn not_found(_req: &Request) -> String {
    "Task not found".to_string()
}
