use dddk_core::dddk::query::response::Response;
use crate::domain::foo::Foo;

pub trait FooRepository {
    fn get_all_foo(&self) -> Vec<Box<dyn Response>>;

    fn save(&self, foo: Foo);
}
