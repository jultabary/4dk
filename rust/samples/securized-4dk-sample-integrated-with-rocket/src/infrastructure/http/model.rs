use crate::domain::foo::Foo;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FooModelApi {
    pub(crate) id: String,
    pub(crate) title: String,
}

impl FooModelApi {
    pub fn from_domain(foo: &Foo) -> FooModelApi {
        FooModelApi {
            id: foo.get_id().to_string(),
            title: foo.get_title().clone(),
        }
    }
}
