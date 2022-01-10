use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use uuid::Uuid;
use crate::domain::foo::{Foo, FooRepository};
use crate::diesel::RunQueryDsl;

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

pub struct FooRepositoryAdapter<'a> {
    pg_connection: &'a PgConnection
}

impl<'a> FooRepositoryAdapter<'a> {
    pub fn new(pg_connection: &'a PgConnection) -> FooRepositoryAdapter<'a> {
        FooRepositoryAdapter {
            pg_connection
        }
    }
}

impl<'a> FooRepository for FooRepositoryAdapter<'a> {
    fn get_all_foo(&self) -> Vec<Foo> {
        use self::super::super::schema::foo::dsl::*;
        let results = foo
            .load::<FooModel>(self.pg_connection)
            .expect("Error loading posts");
        results.iter().map(|model| {
            model.to_domain()
        }).collect()
    }
}

unsafe impl<'a> Send for FooRepositoryAdapter<'a> { }
unsafe impl<'a> Sync for FooRepositoryAdapter<'a> { }