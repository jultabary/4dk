use serde::Deserialize;
use serde::Serialize;
use crate::domain::foo::Foo;

#[derive(Serialize, Deserialize)]
pub struct FooApi {
    pub id: String,
    pub title: String
}

impl FooApi {
    pub fn from_domain(foo: &Foo) -> FooApi {
        FooApi {
            id: foo.get_id().to_string(),
            title: foo.get_title().to_string()
        }
    }
}