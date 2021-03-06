use std::error::Error;
use std::sync::Arc;
use crate::Command;
use crate::dddk::event::event::Event;
use crate::dddk::query::response::Response;

pub type GenericError = Box<dyn Error + Send + Sync + 'static>;
pub type Events = Result<Vec<Arc<dyn Event>>, GenericError>;
pub type Commands = Result<Vec<Box<dyn Command>>, GenericError>;
pub type ResponseFromHandler = Result<Box<dyn Response>, GenericError>;