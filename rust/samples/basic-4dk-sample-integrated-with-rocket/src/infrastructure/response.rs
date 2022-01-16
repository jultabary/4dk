use rocket::serde::Serialize;
use crate::domain::foo::Foo;

#[derive(Serialize)]
pub struct FooResponse {
    id: String,
    title: String
}
impl FooResponse {
    pub fn from_domain(foo: &Foo) -> FooResponse {
        FooResponse {
            id: foo.get_id().to_string(),
            title: foo.get_title().clone()
        }
    }
}