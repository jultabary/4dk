use std::error::Error;
use std::sync::Arc;
use crate::dddk::event::event::Event;
use crate::dddk::query::response::Response;

pub type GenericError = Box<dyn Error + Send + Sync + 'static>;
pub type Events = Result<Vec<Arc<dyn Event>>, GenericError>;
pub type Responses = Result<Vec<Box<dyn Response>>, GenericError>;