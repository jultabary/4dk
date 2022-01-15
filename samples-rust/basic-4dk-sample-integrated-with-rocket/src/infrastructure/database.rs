use std::env;
use std::sync::Arc;
use dddk_core::dddk::query::response::Response;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use uuid::Uuid;
use crate::diesel::RunQueryDsl;
use crate::domain::foo::Foo;
use crate::domain::repository::FooRepository;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct FooModel {
    pub id: Uuid,
    pub title: String
}

impl FooModel {
    fn to_domain(&self) -> Foo {
        Foo::new(
        Uuid::from(self.id),
        self.title.clone()
        )
    }
}

pub struct FooRepositoryAdapter {
    pg_connection: Arc<PgConnection>
}

impl FooRepositoryAdapter {
    pub fn new(pg_connection: Arc<PgConnection>) -> FooRepositoryAdapter {
        FooRepositoryAdapter {
            pg_connection
        }
    }
}

impl FooRepository for FooRepositoryAdapter {
    fn get_all_foo(&self) -> Vec<Box<dyn Response>> {
        use self::super::super::schema::foo::dsl::*;
        let results = foo
            .load::<FooModel>(self.pg_connection.as_ref())
            .expect("Error loading posts");
        results.iter().map(|model| {
            Box::new(model.to_domain()) as Box<dyn Response>
        }).collect()
    }
}
