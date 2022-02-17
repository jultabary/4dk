use crate::domain::foo::Foo;
use dddk_macro::Response;

#[derive(Response)]
pub struct FoosResponse {
    foos: Vec<Foo>
}

impl FoosResponse {
    pub fn new(foos: Vec<Foo>) -> FoosResponse {
        FoosResponse {
            foos
        }
    }

    pub fn get_foos(&self) -> &Vec<Foo> {
        &self.foos
    }
}