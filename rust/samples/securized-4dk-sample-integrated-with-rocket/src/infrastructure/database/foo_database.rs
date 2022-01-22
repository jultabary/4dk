use std::env;
use std::rc::Rc;
use dddk_core::dddk::query::response::Response;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use uuid::Uuid;
use crate::diesel::RunQueryDsl;
use crate::domain::foo::Foo;
use crate::domain::repository::FooRepository;
use crate::schema::foo;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Insertable)]
#[table_name = "foo"]
pub struct FooModel {
    pub id: Uuid,
    pub title: String,
}

impl FooModel {
    fn from_domain(a_foo: &Foo) -> FooModel {
        FooModel {
            id: a_foo.get_id().clone(),
            title: a_foo.get_title().clone(),
        }
    }
}

impl Foo {
    fn from_database(foo_model: &FooModel) -> Foo {
        Foo::new(
            Uuid::from(foo_model.id),
            foo_model.title.clone(),
        )
    }
}

pub struct FooRepositoryAdapter {
    pg_connection: Rc<PgConnection>,
}

impl FooRepositoryAdapter {
    pub fn new(pg_connection: Rc<PgConnection>) -> FooRepositoryAdapter {
        FooRepositoryAdapter {
            pg_connection
        }
    }
}

impl FooRepository for FooRepositoryAdapter {
    fn get_all_foo(&self) -> Vec<Box<dyn Response>> {
        let results: Vec<FooModel> = foo::table
            .load::<FooModel>(self.pg_connection.as_ref())
            .expect("Error loading foos");
        let foos = results
            .iter()
            .map(|model| { Box::new(Foo::from_database(model)) as Box<dyn Response> })
            .collect();
        return foos;
    }

    fn save(&self, a_foo: Foo) {
        let foo_model = FooModel::from_domain(&a_foo);
        diesel::insert_into(foo::table)
            .values(&foo_model)
            .execute(self.pg_connection.as_ref())
            .expect("Error when saving foo");
    }
}
