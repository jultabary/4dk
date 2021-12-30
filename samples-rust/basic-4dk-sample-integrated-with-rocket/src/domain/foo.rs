use uuid::Uuid;

pub struct Foo {
    id: Uuid,
    title: String
}

impl Foo {
    pub fn new(id: Uuid, title: String) -> Foo {
        Foo {
            id,
            title
        }
    }
}

pub trait FooRepository {
    fn get_all_foo(&self) -> Vec<Foo>;
}