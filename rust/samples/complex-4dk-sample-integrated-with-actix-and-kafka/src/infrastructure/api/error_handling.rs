use actix_web::{HttpResponse, ResponseError};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomHttpError {
    #[display(fmt = "forbidden")]
    Forbidden,

    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(fmt = "bad request")]
    BadRequest
}

impl ResponseError for CustomHttpError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomHttpError::Forbidden => StatusCode::FORBIDDEN,
            CustomHttpError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomHttpError::BadRequest => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "plain/text; charset=utf-8")
            .body(self.to_string())
    }
}