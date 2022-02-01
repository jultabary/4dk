use std::any::Any;

pub trait Response {
    fn as_any(&mut self) -> &mut dyn Any;

    fn get_response_name(&self) -> String;
}