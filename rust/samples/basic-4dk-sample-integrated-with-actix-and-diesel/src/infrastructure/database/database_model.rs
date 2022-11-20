use uuid::Uuid;
use crate::domain::foo::Foo;
use self::super::super::super::schema::foo;

#[derive(Queryable, Insertable)]
#[table_name = "foo"]
pub struct FooModel {
    pub id: Uuid,
    pub title: String,
}

impl FooModel {
    pub fn from_domain(a_foo: &Foo) -> FooModel {
        FooModel {
            id: a_foo.get_id().clone(),
            title: a_foo.get_title().clone(),
        }
    }
}

impl Foo {
    pub(crate) fn from_database(foo_model: &FooModel) -> Foo {
        Foo::new(
            Uuid::from(foo_model.id),
            foo_model.title.clone(),
        )
    }
}
