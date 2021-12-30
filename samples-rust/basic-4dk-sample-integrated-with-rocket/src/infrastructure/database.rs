use std::env;
use diesel::{Connection, PgConnection, RunQueryDsl, sql_query};
use dotenv::dotenv;
use uuid::Uuid;
use crate::domain::foo::{Foo, FooRepository};

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

struct FooRepositoryAdapter<'a> {
    pg_connection: &'a PgConnection
}
impl <'a>FooRepository for FooRepositoryAdapter<'a> {
    fn get_all_foo(&self) -> Vec<Foo> {
        let foo_vec: Vec<FooModel> = sql_query("SELECT * FROM foo")
            .load(self.pg_connection)
            .unwrap();
        foo_vec.iter().map(|model| {
            model.to_domain()
        }).collect()
    }
}
