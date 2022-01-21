use dddk_core::dddk::aliases::GenericError;
use dddk_security::dddk::security::errors::Forbidden;
use rocket::http::Status;
use rocket::Request;

pub fn catch_error_from_bus(error: GenericError) -> Status {
    if let Some(_) = error.downcast_ref::<Forbidden>() {
        return Status::Forbidden;
    } else {
        Status::InternalServerError
    }
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> String {
    "User has not enough privileges for this request".to_string()
}