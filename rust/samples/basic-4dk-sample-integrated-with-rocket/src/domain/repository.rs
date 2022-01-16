use dddk_core::dddk::query::response::Response;

pub trait FooRepository {
    fn get_all_foo(&self) -> Vec<Box<dyn Response>>;
}
